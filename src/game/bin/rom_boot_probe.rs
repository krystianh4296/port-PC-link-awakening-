//! Deterministyczny test startu ROM-u bez okna.

use gameboy_port::game::Game;
use gameboy_port::rom::Rom;

const ROM_PATH: &str = "Legend of Zelda, The - Link's Awakening DX (USA, Europe) (Rev 2).gbc";
const TARGET_FRAMES: u16 = 600;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rom = Rom::load(ROM_PATH)?;
    let mut game = Game::new(rom);
    let mut frames = 0u16;
    let mut cycles = 0u64;
    let scripted_start = std::env::args().any(|arg| arg == "--start");

    while frames < TARGET_FRAMES {
        // Deterministyczny odpowiednik pięciu klatek wciśniętego Start.
        // Pozwala porównywać później ten sam replay z portem natywnym.
        let start_pressed = scripted_start && (120..125).contains(&frames);
        game.memory_mut().set_joypad_button(7, start_pressed);
        cycles += game.step() as u64;
        if game.take_frame_ready() { frames += 1; }
    }

    let checksum = game.framebuffer().iter().fold(0u64, |hash, pixel| {
        hash.rotate_left(5) ^ u64::from(*pixel)
    });
    let chromatic_pixels = game.framebuffer().iter().filter(|pixel| {
        let red = (**pixel >> 16) & 0xFF;
        let green = (**pixel >> 8) & 0xFF;
        let blue = **pixel & 0xFF;
        red != green || green != blue
    }).count();
    println!(
        "boot-ok frames={frames} cycles={cycles} start={scripted_start} pc={:04X} colors={chromatic_pixels} apu=NR52:{:02X}/NR50:{:02X}/NR51:{:02X} frame-hash={checksum:016X}",
        game.cpu().pc, game.read(0xFF26), game.read(0xFF24), game.read(0xFF25)
    );
    Ok(())
}
