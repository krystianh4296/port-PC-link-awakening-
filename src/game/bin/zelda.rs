use gameboy_port::graphics::Tile;
use gameboy_port::rom::Rom;
use gameboy_port::rendering::renderer::Renderer;

use std::time::{Duration, Instant};

const TARGET_FPS: u32 = 60;
const FRAME_TIME: Duration =
    Duration::from_nanos(1_000_000_000 / TARGET_FPS as u64);

const GRAPHICS_BANK: usize = 0x0C;
const TILE_SIZE: usize = 16;
const TILES_PER_ROW: usize = 32;
const TILE_COUNT: usize = 1024;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rom = Rom::load(
        "Legend of Zelda, The - Link's Awakening DX (USA, Europe) (Rev 2).gbc"
    )?;

    let mut renderer = Renderer::new();

    while renderer.is_open() {
        let frame_start = Instant::now();

        render_tileset(&mut renderer, &rom);

        renderer.draw();

        let elapsed = frame_start.elapsed();
        if elapsed < FRAME_TIME {
            std::thread::sleep(FRAME_TIME - elapsed);
        }
    }

    Ok(())
}

fn render_tileset(renderer: &mut Renderer, rom: &Rom) {
    renderer.clear(0xFF202020);

    let bank = rom.bank(GRAPHICS_BANK);

    for tile_index in 0..TILE_COUNT {
        let offset = tile_index * TILE_SIZE;

        let bytes: [u8; 16] = bank[offset..offset + TILE_SIZE]
            .try_into()
            .expect("Nieprawidłowy zakres tile");

        let tile = Tile::decode(&bytes);

        let x = (tile_index % TILES_PER_ROW) * 8;
        let y = (tile_index / TILES_PER_ROW) * 8;

        tile.render(renderer, x, y);
    }
}