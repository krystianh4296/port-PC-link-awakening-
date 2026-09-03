use gameboy_port::game::game::Game;
use gameboy_port::input::Input;
use gameboy_port::rendering::renderer::Renderer;
use gameboy_port::rom::Rom;

use std::env;
use std::time::{Duration, Instant};

const TARGET_FPS: u32 = 60;
const FRAME_TIME: Duration =
    Duration::from_nanos(1_000_000_000 / TARGET_FPS as u64);

fn main() {
    let rom_path = env::args()
        .nth(1)
        .unwrap_or_else(|| {
            "Legend of Zelda, The - Link's Awakening DX (USA, Europe) (Rev 2).gbc".to_string()
        });

    println!("Ładowanie ROM: {}", rom_path);

    match std::fs::metadata(&rom_path) {
        Ok(metadata) => {
            println!("  Rozmiar pliku: {} bytes", metadata.len());
        }
        Err(error) => {
            eprintln!("  Nie można sprawdzić pliku: {error}");
        }
    }

    let rom = match Rom::load(&rom_path) {
        Ok(rom) => rom,
        Err(error) => {
            eprintln!("Błąd ROM: {error}");
            std::process::exit(1);
        }
    };

    println!("ROM załadowany poprawnie.");
    println!("  Rozmiar:      {} bytes", rom.len());
    println!("  Banki ROM:    {}", rom.bank_count());
    println!("  Tytuł:        {}", rom.header().title);
    println!(
        "  Cartridge:    {:02X}",
        rom.header().cartridge_type
    );
    println!(
        "  ROM size:     {:02X}",
        rom.header().rom_size_code
    );
    println!(
        "  RAM size:     {:02X}",
        rom.header().ram_size_code
    );

    let mut game = Game::new(rom);

    println!();
    println!("Test mapowania ROM/MBC5:");
    println!("  Bank początkowy: {}", game.rom_bank());
    println!("  [0000] = {:02X}", game.read_rom(0x0000));
    println!("  [4000] = {:02X}", game.read_rom(0x4000));

    game.select_rom_bank(2);
    println!("  Po wyborze banku 2:");
    println!("  Bank aktywny:    {}", game.rom_bank());
    println!("  [4000] = {:02X}", game.read_rom(0x4000));

    println!();
    println!("Uruchamianie natywnej wersji gry...");

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
