use gameboy_port::graphics::Tile;
use gameboy_port::rom::Rom;
use gameboy_port::rendering::renderer::Renderer;

use std::time::{Duration, Instant};

const TARGET_FPS: u32 = 60;
const FRAME_TIME: Duration =
    Duration::from_nanos(1_000_000_000 / TARGET_FPS as u64);

const TILE_DATA_START: usize = 0x2C000;

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

    for tile_index in 0..512 {
        let address = TILE_DATA_START + tile_index * 16;

        let bytes = rom.tile_bytes(address);
        let tile = Tile::decode(&bytes);

        let tx = tile_index % 16;
        let ty = tile_index / 16;

        tile.render(renderer, tx * 8, ty * 8);
    }
}