use gameboy_port::game::game::Game;
use gameboy_port::input::Input;
use gameboy_port::rendering::renderer::Renderer;
use gameboy_port::rom::Rom;
use gameboy_port::graphics::Tile;

use std::env;
use std::time::{Duration, Instant};

const TARGET_FPS: u32 = 60;
const FRAME_TIME: Duration =
    Duration::from_nanos(1_000_000_000 / TARGET_FPS as u64);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rom = Rom::load("Legend of Zelda, The - Link's Awakening DX (USA, Europe) (Rev 2).gbc")?;

    let bytes = rom.tile_bytes(0x8000);
    let tile = Tile::decode(&bytes);

    println!("{:?}", tile);
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
    println!("  Cartridge:    {:02X}", rom.header().cartridge_type);
    println!("  ROM size:     {:02X}", rom.header().rom_size_code);
    println!("  RAM size:     {:02X}", rom.header().ram_size_code);

    let mut game = Game::new(rom);

    println!();
    println!("Test zunifikowanej mapy pamięci:");
    println!("  Bank początkowy: {}", game.rom_bank());
    println!("  [0000] = {:02X}", game.read(0x0000));
    println!("  [4000] = {:02X}", game.read(0x4000));

    game.select_rom_bank(2);
    println!("  Po wyborze banku 2:");
    println!("  Bank aktywny:    {}", game.rom_bank());
    println!("  [4000] = {:02X}", game.read(0x4000));

    game.write(0xC000, 0x12);
    println!("  [C000] zapis/odczyt = {:02X}", game.read(0xC000));

    game.write(0x8000, 0x34);
    println!("  [8000] zapis/odczyt = {:02X}", game.read(0x8000));

    game.write(0xFF80, 0x56);
    println!("  [FF80] zapis/odczyt = {:02X}", game.read(0xFF80));

    println!();
    println!("Uruchamianie natywnej wersji gry...");

    let mut input = Input::new();
    let mut renderer = Renderer::new();

    while game.is_running() && renderer.is_open() {
        let frame_start = Instant::now();

        input.update(renderer.window());

        game.update(&input, FRAME_TIME.as_secs_f32());

        render_tileset(&mut renderer, &rom);

        renderer.draw();

        let elapsed = frame_start.elapsed();

        if elapsed < FRAME_TIME {
            std::thread::sleep(FRAME_TIME - elapsed);
        }
    }
        print_ascii(&tile);

Ok(())
}
pub fn print_ascii(tile:&Tile){
    for y in 0..8{
        for x in 0..8{
            let c = match tile.pixel(x,y){
                0=>' ',
                1=>'░',
                2=>'▒',
                _=>'█',
            };
            print!("{c}");
        }
        println!();
    }
    fn render_tileset(renderer: &mut Renderer, rom: &Rom) {
        renderer.clear(0xFF202020);

        for tile_index in 0..512 {
            const TILE_DATA_START: usize = 0x2C000;

            let address = TILE_DATA_START + tile_index * 16;
            let bytes = rom.tile_bytes(address);
            let tile = Tile::decode(&bytes);

            let tx = tile_index % 16;
            let ty = tile_index / 16;

            tile.render(renderer, tx * 8, ty * 8);
        }
    }
}