use gameboy_port::game::game::Game;
use gameboy_port::input::Input;
use gameboy_port::rendering::renderer::Renderer;

use std::time::{Duration, Instant};

const TARGET_FPS: u32 = 60;
const FRAME_TIME: Duration =
    Duration::from_nanos(1_000_000_000 / TARGET_FPS as u64);

fn main() {
    let mut game = Game::new();
    let mut input = Input::new();
    let mut renderer = Renderer::new();

    while game.is_running() && renderer.is_open() {
        let frame_start = Instant::now();

        input.update(renderer.window());

        game.update(&input, FRAME_TIME.as_secs_f32());

        renderer.draw();

        let elapsed = frame_start.elapsed();

        if elapsed < FRAME_TIME {
            std::thread::sleep(FRAME_TIME - elapsed);
        }
    }
}