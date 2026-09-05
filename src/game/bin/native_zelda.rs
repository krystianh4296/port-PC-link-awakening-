//! Pierwszy uruchamialny fragment natywnej reimplementacji.

use gameboy_port::native::NativeGame;
use gameboy_port::rendering::renderer::Renderer;
use gameboy_port::rom::Rom;
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rom = Rom::load("Legend of Zelda, The - Link's Awakening DX (USA, Europe) (Rev 2).gbc")?;
    let mut game = NativeGame::from_rom(&rom)?;
    let mut renderer = Renderer::with_title("Zelda - ROM reference scene");
    let mut last_frame = Instant::now();
    let mut framebuffer = Box::new([0u32; 160 * 144]);

    println!("Native engine reference ROM: {}", game.reference_title());
    while renderer.is_open() && game.is_running() {
        let now = Instant::now();
        let elapsed = now.duration_since(last_frame).as_secs_f32();
        last_frame = now;
        game.update(renderer.window(), elapsed);
        game.render(&mut framebuffer);
        renderer.copy_frame(&framebuffer);
        renderer.draw();
    }
    Ok(())
}
