use gameboy_port::game::Game;
use gameboy_port::input::Input;
use gameboy_port::rendering::renderer::Renderer;
use gameboy_port::rom::Rom;

use std::time::{Duration, Instant};

const TARGET_FPS: u32 = 60;
const FRAME_TIME: Duration =
    Duration::from_nanos(1_000_000_000 / TARGET_FPS as u64);

fn print_first_frame_diagnostics(game: &Game) {
    println!("\n=== PPU/VRAM DIAGNOSTICS: FIRST FRAME ===");
    println!("CPU: PC={:04X} SP={:04X} AF={:04X} BC={:04X} DE={:04X} HL={:04X}",
        game.cpu().pc,
        game.cpu().sp,
        game.cpu().af(),
        game.cpu().bc(),
        game.cpu().de(),
        game.cpu().hl(),
    );
    println!("PPU: LY={:02X} LCDC={:02X} STAT={:02X} SCX={:02X} SCY={:02X} WY={:02X} WX={:02X}",
        game.read(0xFF44),
        game.read(0xFF40),
        game.read(0xFF41),
        game.read(0xFF43),
        game.read(0xFF42),
        game.read(0xFF4A),
        game.read(0xFF4B),
    );
    println!("VRAM bank: {:02X}", game.read(0xFF4F));

    print!("VRAM 8000-801F:");
    for address in 0x8000..=0x801F {
        print!(" {:02X}", game.read(address));
    }
    println!();

    print!("BG MAP 9800-981F:");
    for address in 0x9800..=0x981F {
        print!(" {:02X}", game.read(address));
    }
    println!();

    println!("CGB BG palette: FF68={:02X} FF69={:02X}",
        game.read(0xFF68),
        game.read(0xFF69),
    );

    let lo0 = game.read(0x8000);
    let hi0 = game.read(0x8001);
    let lo1 = game.read(0x8002);
    let hi1 = game.read(0x8003);

    println!("TILE 0 bytes: row0={:02X} {:02X} row1={:02X} {:02X}",
        lo0, hi0, lo1, hi1);

    println!("TILE 0 decoded 8x8 (DMG pixel indices):");
    for row in 0..8 {
        let lo = game.read(0x8000 + row * 2);
        let hi = game.read(0x8001 + row * 2);
        print!("  ");
        for bit in (0..8).rev() {
            let color = ((lo >> bit) & 1) | (((hi >> bit) & 1) << 1);
            print!("{}", color);
        }
        println!();
    }

    println!("=========================================\n");
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rom = Rom::load(
        "Legend of Zelda, The - Link's Awakening DX (USA, Europe) (Rev 2).gbc",
    )?;

    let mut game = Game::new(rom);
    let mut renderer = Renderer::new();
    let mut input = Input::new();
    let mut diagnostics_printed = false;

    // Główna pętla emulatora:
    // 1. pobierz wejście,
    // 2. wykonuj CPU + sprzęt do zakończenia bieżącej klatki,
    // 3. odbierz gotowy framebuffer,
    // 4. narysuj klatkę,
    // 5. utrzymuj docelowe 60 FPS.
    while renderer.is_open() && game.is_running() {
        let frame_start = Instant::now();

        input.update(renderer.window());

        while !game.frame_ready() {
            game.step();
        }

        if !diagnostics_printed {
            print_first_frame_diagnostics(&game);
            game.print_vram_diagnostics();
            diagnostics_printed = true;
        }

        renderer.copy_frame(game.framebuffer());
        game.take_frame_ready();
        renderer.draw();

        let elapsed = frame_start.elapsed();
        if elapsed < FRAME_TIME {
            std::thread::sleep(FRAME_TIME - elapsed);
        }
    }

    Ok(())
}
