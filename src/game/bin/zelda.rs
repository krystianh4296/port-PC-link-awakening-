use gameboy_port::game::Game;
use gameboy_port::input::Input;
use gameboy_port::rendering::renderer::Renderer;
use gameboy_port::rom::Rom;

use std::time::{Duration, Instant};

const TARGET_FPS: u32 = 60;
const FRAME_TIME: Duration =
    Duration::from_nanos(1_000_000_000 / TARGET_FPS as u64);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rom = Rom::load(
        "Legend of Zelda, The - Link's Awakening DX (USA, Europe) (Rev 2).gbc",
    )?;

    let mut game = Game::new(rom);
    let mut renderer = Renderer::new();
    let mut input = Input::new();

    while renderer.is_open() && game.is_running() {
        let frame_start = Instant::now();

        input.update(renderer.window());
        game.update(&input, 1.0 / TARGET_FPS as f32);

        if game.frame_ready() {
            renderer.copy_frame(game.framebuffer());
            game.take_frame_ready();
            renderer.draw();
        }

        let elapsed = frame_start.elapsed();
        if elapsed < FRAME_TIME {
            std::thread::sleep(FRAME_TIME - elapsed);
        }
    }

    Ok(())
}
