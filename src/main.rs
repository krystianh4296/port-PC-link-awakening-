mod bus;
mod cpu;
mod rom;
mod apu;
mod audio;
mod save;
mod savestate;

use std::env;

use bus::Bus;
use cpu::Cpu;
use minifb::{Key, Window, WindowOptions};
use audio::Audio;
use savestate::{SaveState, save_to_file, load_from_file};

const WIDTH: usize = 160;
const HEIGHT: usize = 144;

const TILE_DEBUG_WIDTH: usize = 128;
const TILE_DEBUG_HEIGHT: usize = 192;

fn main() {
    let rom_path = env::args().nth(1).unwrap_or_else(|| {
        "Legend of Zelda, The - Links Awakening (USA, Europe) (Rev 2).gb".to_string()
    });
    let audio = Audio::new();

    let mut bus = Bus::new(&rom_path);
    let mut cpu = Cpu::new();
    cpu.reset();

    bus.set_audio(audio);

    println!("ROM wczytany poprawnie.");
    println!("Rozmiar: {} bajtów", bus.rom.size());
    println!("Tytuł: {}", bus.rom.title());
    println!("Cartridge type: {:02X}", bus.rom.cartridge_type());
    println!("ROM size code: {:02X}", bus.rom.rom_size_code());
    println!("RAM size code: {:02X}", bus.rom.ram_size_code());
    println!("CPU uruchomiony. PC={:04X} SP={:04X}", cpu.pc, cpu.sp);
    println!("Sterowanie: WASD = D-pad, J = A, K = B, U = Select, I = Start");

    let mut window = Window::new(
        "Game Boy Emulator",
        WIDTH,
        HEIGHT,
        WindowOptions {
            scale: minifb::Scale::X4,
            ..WindowOptions::default()
        },
    )
    .expect("Nie udało się utworzyć okna");

        window.set_target_fps(60);

    let mut tile_window = Window::new(
        "VRAM Tiles",
        TILE_DEBUG_WIDTH,
        TILE_DEBUG_HEIGHT,
        WindowOptions {
            scale: minifb::Scale::X4,
            ..WindowOptions::default()
        },
    )
    .expect("Nie udało się utworzyć okna VRAM");

    let mut buffer = vec![0u32; WIDTH * HEIGHT];
    let mut tile_buffer = vec![0u32; TILE_DEBUG_WIDTH * TILE_DEBUG_HEIGHT];
    let mut steps: u64 = 0;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let mut buttons = 0xFFu8;
        let cycles = cpu.step(&mut bus);
        bus.step(cycles, &mut buffer);

        steps += 1;

        if steps % 1_000_000 == 0 {
            println!("===== OPCODES po {} instrukcjach =====", steps);

            for opcode in 0..256 {
                let count = cpu.opcode_counts[opcode];

                if count != 0 {
                    println!("OP {:02X}: {}", opcode, count);
                }
            }
        }
        if window.is_key_pressed(Key::F5, minifb::KeyRepeat::No) {
            bus.save_game();
        }

        if window.is_key_pressed(Key::F6, minifb::KeyRepeat::No) {
            bus.load_game();
        }
        if window.is_key_pressed(Key::F8, minifb::KeyRepeat::No) {
            let state = SaveState::capture(&cpu, &bus);
            save_to_file(&state, "save.state");
            println!("Savestate zapisany.");
        }

        if window.is_key_pressed(Key::F9, minifb::KeyRepeat::No) {
            let state = load_from_file("save.state");
            state.restore(&mut cpu, &mut bus);
            println!("Savestate wczytany.");
        }

        if window.is_key_down(Key::D) {
            buttons &= !(1 << 0);
        }
        if window.is_key_down(Key::A) {
            buttons &= !(1 << 1);
        }
        if window.is_key_down(Key::W) {
            buttons &= !(1 << 2);
        }
        if window.is_key_down(Key::S) {
            buttons &= !(1 << 3);
        }
        if window.is_key_down(Key::J) {
            buttons &= !(1 << 4);
        }
        if window.is_key_down(Key::K) {
            buttons &= !(1 << 5);
        }
        if window.is_key_down(Key::U) {
            buttons &= !(1 << 6);
        }
        if window.is_key_down(Key::I) {
            buttons &= !(1 << 7);
        }

        bus.set_buttons(buttons);

        let cycles = cpu.step(&mut bus);
        let frame_ready = bus.step(cycles, &mut buffer);

        steps += 1;

        if steps % 50_000 == 0 {
            bus.render_tile_debug(&mut tile_buffer);
            tile_window
                .update_with_buffer(&tile_buffer, TILE_DEBUG_WIDTH, TILE_DEBUG_HEIGHT)
                .unwrap();
        }

        if frame_ready {
            window
                .update_with_buffer(&buffer, WIDTH, HEIGHT)
                .expect("Błąd aktualizacji ekranu");
        }
    }
}

#[cfg(test)]
mod boot_progress {
    use super::*;

    #[test]
    fn leaves_ly_wait_loop() {
        let rom = r"C:\GameBoyPort\gameboy-port\Legend of Zelda, The - Links Awakening (USA, Europe) (Rev 2).gb";
        let mut bus = Bus::new(rom);
        let mut cpu = Cpu::new();
        cpu.reset();
        let mut buffer = vec![0u32; WIDTH * HEIGHT];

        for _ in 0..3_000_000 {
            let cycles = cpu.step(&mut bus);
            bus.step(cycles, &mut buffer);
        }

        assert!(
            cpu.pc < 0x2887 || cpu.pc > 0x288C,
            "CPU nadal czeka na LY=145: PC={:04X} LY={} LCDC={:02X} SP={:04X}",
            cpu.pc,
            bus.ly,
            bus.lcdc,
            cpu.sp
        );
        println!(
            "OK PC={:04X} LY={} LCDC={:02X} SP={:04X} IE={:02X} IF={:02X}",
            cpu.pc, bus.ly, bus.lcdc, cpu.sp, bus.ie, bus.if_reg
        );
    }
}
