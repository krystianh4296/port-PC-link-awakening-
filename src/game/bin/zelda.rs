use gameboy_port::game::Game;
use gameboy_port::audio::Audio;
use gameboy_port::input::Input;
use gameboy_port::rendering::renderer::Renderer;
use gameboy_port::rendering::tile_viewer::TileViewer;
use gameboy_port::rom::Rom;
use minifb::{Key, KeyRepeat};

use std::time::{Duration, Instant};

fn print_first_frame_diagnostics(game: &Game) {
    println!("\n=== PPU/VRAM DIAGNOSTICS ===");
    println!("CPU: PC={:04X} SP={:04X} AF={:04X} BC={:04X} DE={:04X} HL={:04X}",
        game.cpu().pc, game.cpu().sp, game.cpu().af(), game.cpu().bc(), game.cpu().de(), game.cpu().hl());
    println!("PPU: LY={:02X} LCDC={:02X} STAT={:02X} SCX={:02X} SCY={:02X} WY={:02X} WX={:02X}",
        game.read(0xFF44), game.read(0xFF40), game.read(0xFF41), game.read(0xFF43),
        game.read(0xFF42), game.read(0xFF4A), game.read(0xFF4B));
    println!("VRAM bank: {:02X}", game.read(0xFF4F));

    print!("VRAM 8000-801F:");
    for address in 0x8000..=0x801F { print!(" {:02X}", game.read(address)); }
    println!();

    print!("BG MAP 9800-981F:");
    for address in 0x9800..=0x981F { print!(" {:02X}", game.read(address)); }
    println!();

    println!("CGB BG palette: FF68={:02X} FF69={:02X}", game.read(0xFF68), game.read(0xFF69));
    println!("TILE 0 bytes: row0={:02X} {:02X} row1={:02X} {:02X}",
        game.read(0x8000), game.read(0x8001), game.read(0x8002), game.read(0x8003));

    println!("TILE 0 decoded 8x8 (DMG pixel indices):");
    for row in 0..8 {
        let lo = game.read(0x8000 + row * 2);
        let hi = game.read(0x8001 + row * 2);
        print!("  ");
        for bit in (0..8).rev() { print!("{}", ((lo >> bit) & 1) | (((hi >> bit) & 1) << 1)); }
        println!();
    }
    println!("=========================================\n");
}

/// Dumps the live CGB BG attribute map from VRAM bank 1.
/// This is intentionally read from bank 1 directly through the normal FF4F
/// banking interface, so it verifies the bytes actually present during gameplay
/// rather than a copied/derived map.
fn print_live_bg_attribute_diagnostics(game: &mut Game) {
    let previous_bank = game.read(0xFF4F) & 1;
    let lcdc = game.read(0xFF40);
    let scx = game.read(0xFF43);
    let scy = game.read(0xFF42);
    let map_base = if lcdc & 0x08 != 0 { 0x9C00u16 } else { 0x9800u16 };

    game.write(0xFF4F, 1);

    let mut attributes = [0u8; 32 * 32];
    for i in 0..attributes.len() {
        attributes[i] = game.read(map_base + i as u16);
    }

    game.write(0xFF4F, previous_bank);

    let nonzero = attributes.iter().filter(|&&a| a != 0).count();
    let mut palette_counts = [0usize; 8];
    let mut bank1_count = 0usize;
    let mut flip_x_count = 0usize;
    let mut flip_y_count = 0usize;
    let mut priority_count = 0usize;
    for &a in &attributes {
        palette_counts[(a & 0x07) as usize] += 1;
        bank1_count += usize::from(a & 0x08 != 0);
        flip_x_count += usize::from(a & 0x20 != 0);
        flip_y_count += usize::from(a & 0x40 != 0);
        priority_count += usize::from(a & 0x80 != 0);
    }

    let screen_tile_x = ((scx as usize) >> 3) & 31;
    let screen_tile_y = ((scy as usize) >> 3) & 31;
    let screen_attr = attributes[screen_tile_y * 32 + screen_tile_x];

    println!("\n=== LIVE CGB BG ATTRIBUTES ===");
    println!("map={:04X} VRAM bank=1 SCX={:02X} SCY={:02X}", map_base, scx, scy);
    println!("nonzero={}/1024 palette={:?} bank1={} flip_x={} flip_y={} priority={}",
        nonzero, palette_counts, bank1_count, flip_x_count, flip_y_count, priority_count);
    println!("top-left screen tile ({},{}) attr={:02X}", screen_tile_x, screen_tile_y, screen_attr);
    println!("attribute map:");
    for row in 0..32 {
        print!("{:02X}: ", row * 32);
        for col in 0..32 { print!("{:02X} ", attributes[row * 32 + col]); }
        println!();
    }
    println!("==============================\n");
}

/// Dumps the actual bank-0 tile-number map used by the BG renderer, together
/// with the bank-1 attributes for the same 32x32 entries. The visible 20x18
/// screen region is also printed using the current SCX/SCY. This separates a
/// bad tile-number map from bad CGB attributes or bad tile decoding.
fn print_live_bg_tile_map_diagnostics(game: &mut Game) {
    let previous_bank = game.read(0xFF4F) & 1;
    let lcdc = game.read(0xFF40);
    let scx = game.read(0xFF43);
    let scy = game.read(0xFF42);
    let map_base = if lcdc & 0x08 != 0 { 0x9C00u16 } else { 0x9800u16 };
    let tile_base = if lcdc & 0x10 != 0 { 0x8000u16 } else { 0x9000u16 };

    let mut tile_map = [0u8; 32 * 32];
    let mut attributes = [0u8; 32 * 32];

    game.write(0xFF4F, 0);
    for i in 0..tile_map.len() {
        tile_map[i] = game.read(map_base + i as u16);
    }

    game.write(0xFF4F, 1);
    for i in 0..attributes.len() {
        attributes[i] = game.read(map_base + i as u16);
    }

    game.write(0xFF4F, previous_bank);

    let nonzero_tiles = tile_map.iter().filter(|&&t| t != 0).count();
    let unique_tiles = {
        let mut seen = [false; 256];
        for &tile in &tile_map { seen[tile as usize] = true; }
        seen.iter().filter(|&&v| v).count()
    };

    let visible_x = (scx as usize) >> 3;
    let visible_y = (scy as usize) >> 3;

    println!("\n=== LIVE CGB BG TILE MAP ===");
    println!("map={:04X} tile_base={:04X} LCDC={:02X} SCX={:02X} SCY={:02X}",
        map_base, tile_base, lcdc, scx, scy);
    println!("bank0 map: nonzero={}/1024 unique_tile_ids={}", nonzero_tiles, unique_tiles);
    println!("full bank0 tile-number map (32x32):");
    for row in 0..32 {
        print!("{:02X}: ", row * 32);
        for col in 0..32 { print!("{:02X} ", tile_map[row * 32 + col]); }
        println!();
    }

    println!("visible 20x18 tile IDs / attrs (screen origin uses SCX/SCY tile coordinates):");
    for row in 0..18usize {
        print!("row {:02}: IDs ", row);
        for col in 0..20usize {
            let map_x = (visible_x + col) & 31;
            let map_y = (visible_y + row) & 31;
            let index = map_y * 32 + map_x;
            print!("{:02X} ", tile_map[index]);
        }
        print!(" | ATTR ");
        for col in 0..20usize {
            let map_x = (visible_x + col) & 31;
            let map_y = (visible_y + row) & 31;
            let index = map_y * 32 + map_x;
            print!("{:02X} ", attributes[index]);
        }
        println!();
    }

    let mut bank1_tiles = 0usize;
    let mut flip_x = 0usize;
    let mut flip_y = 0usize;
    let mut priority = 0usize;
    for &a in &attributes {
        bank1_tiles += usize::from(a & 0x08 != 0);
        flip_x += usize::from(a & 0x20 != 0);
        flip_y += usize::from(a & 0x40 != 0);
        priority += usize::from(a & 0x80 != 0);
    }
    println!("attribute flags: tile_bank1={} flip_x={} flip_y={} priority={}",
        bank1_tiles, flip_x, flip_y, priority);
    println!("================================\n");
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rom_path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "Legend of Zelda, The - Link's Awakening DX (USA, Europe) (Rev 2).gbc".to_string());

    let rom = Rom::load(&rom_path)?;
    let mut game = Game::new(rom);
    game.set_audio(Audio::new());
    let mut renderer = Renderer::new();
    let mut tile_viewer = TileViewer::new();
    let mut input = Input::new();
    let mut diagnostics_printed = false;

    const TARGET_FPS: u32 = 60;
    const FRAME_TIME: Duration = Duration::from_nanos(1_000_000_000 / TARGET_FPS as u64);

    while renderer.is_open() && game.is_running() {
        let frame_start = Instant::now();
        input.update(renderer.window());
        game.apply_input(&input);

        while !game.frame_ready() { game.step(); }

        // Graphics initialization in the ROM continues after the first VBlank,
        // so collect the counters on the following frame instead of stopping here.
        if !diagnostics_printed {
            game.take_frame_ready();
            while !game.frame_ready() { game.step(); }
            print_first_frame_diagnostics(&game);
            game.print_vram_diagnostics();
            diagnostics_printed = true;
        }

        // F9 captures the actual CGB attribute map at the current gameplay state.
        if renderer.window().is_key_pressed(Key::F9, KeyRepeat::No) {
            print_live_bg_attribute_diagnostics(&mut game);
        }

        // F10 captures the actual bank-0 tile-number map and pairs it with the
        // bank-1 attributes. This is the primary diagnostic for the wrong map
        // layout problem; it does not alter renderer behavior.
        if renderer.window().is_key_pressed(Key::F10, KeyRepeat::No) {
            print_live_bg_tile_map_diagnostics(&mut game);
        }

        renderer.copy_frame(game.framebuffer());
        game.take_frame_ready();
        renderer.draw();
        tile_viewer.update(&mut game);

        let elapsed = frame_start.elapsed();
        if elapsed < FRAME_TIME { std::thread::sleep(FRAME_TIME - elapsed); }
    }
    Ok(())
}
