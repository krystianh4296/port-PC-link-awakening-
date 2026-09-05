//! Compatibility entry point retained for existing launch shortcuts.
//!
//! This used to display a frozen scene assembled from a handful of emulator
//! frames. That path could not show later VRAM/tile-map updates. Run the live
//! emulator here instead.

use gameboy_port::audio::Audio;
use gameboy_port::game::Game;
use gameboy_port::input::Input;
use gameboy_port::rendering::renderer::Renderer;
use gameboy_port::rom::Rom;
use std::time::{Duration, Instant};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rom = Rom::load("Legend of Zelda, The - Link's Awakening DX (USA, Europe) (Rev 2).gbc")?;
    let mut game = Game::new(rom);
    game.set_audio(Audio::new());
    let mut renderer = Renderer::new();
    let mut input = Input::new();

    const FRAME_TIME: Duration = Duration::from_nanos(1_000_000_000 / 60);
    while renderer.is_open() && game.is_running() {
        let frame_start = Instant::now();
        input.update(renderer.window());
        game.apply_input(&input);
        while !game.frame_ready() {
            game.step();
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
