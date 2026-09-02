mod bus;
mod cpu;
mod rom;
mod apu;
mod audio;
mod save;
mod savestate;
mod debug;

use std::env;
use std::time::{Duration, Instant};
use bus::Bus;
use cpu::Cpu;
use debug::{ConsoleCommand, DebugConsole, Debugger};
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
    bus.load_game();

    let mut cpu = Cpu::new();
    cpu.reset();

    bus.set_audio(audio);

    let mut debugger = Debugger::new();

    println!("ROM wczytany poprawnie.");
    println!("Rozmiar: {} bajtów", bus.rom.size());
    println!("Tytuł: {}", bus.rom.title());
    println!("Cartridge type: {:02X}", bus.rom.cartridge_type());
    println!("ROM size code: {:02X}", bus.rom.rom_size_code());
    println!("RAM size code: {:02X}", bus.rom.ram_size_code());
    println!("CPU uruchomiony. PC={:04X} SP={:04X}", cpu.pc, cpu.sp);
    println!("Sterowanie: WASD = D-pad, J = A, K = B, U = Select, I = Start");
    println!("DEBUG: F1=włącz/wyłącz, F2=1 instrukcja, F3=continue, F4=break, F10=BP PC, F11=usuń BP, F12=status");
    println!("DEBUG CONSOLE: BP 01A6 | WATCH FF44 | TRACE 0100:0200 | DIS 03CE 3 | HELP");

    let debug_console = DebugConsole::new();

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

    let frame_duration = Duration::from_secs_f64(1.0 / 60.0);
    let mut next_frame_time = Instant::now() + frame_duration;

    let mut f1_key_lock = false;
    let mut f2_key_lock = false;
    let mut f3_key_lock = false;
    let mut f4_key_lock = false;
    let mut f5_key_lock = false;
    let mut f6_key_lock = false;
    let mut f8_key_lock = false;
    let mut f9_key_lock = false;
    let mut f10_key_lock = false;
    let mut f11_key_lock = false;
    let mut f12_key_lock = false;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        while let Some(command) = debug_console.try_read() {
            let command = ConsoleCommand::parse(&command);

            debugger.execute_console_command(
                command,
                &cpu,
                &mut bus,
            );
        }

        let mut buttons = 0xFFu8;

        let f1_down = window.is_key_down(Key::F1);
        let f2_down = window.is_key_down(Key::F2);
        let f3_down = window.is_key_down(Key::F3);
        let f4_down = window.is_key_down(Key::F4);
        let f5_down = window.is_key_down(Key::F5);
        let f6_down = window.is_key_down(Key::F6);
        let f8_down = window.is_key_down(Key::F8);
        let f9_down = window.is_key_down(Key::F9);
        let f10_down = window.is_key_down(Key::F10);
        let f11_down = window.is_key_down(Key::F11);
        let f12_down = window.is_key_down(Key::F12);

        if f1_down && !f1_key_lock {
            if debugger.enabled {
                debugger.disable();
                println!("DEBUG: wyłączony");
            } else {
                debugger.enable();
                debugger.print_status(&cpu);
                println!("DEBUG: włączony");
            }
            f1_key_lock = true;
        }
        if !f1_down { f1_key_lock = false; }

        if f2_down && !f2_key_lock {
            debugger.step();
            println!("DEBUG: STEP -> wykonana zostanie dokładnie 1 instrukcja CPU");
            f2_key_lock = true;
        }
        if !f2_down { f2_key_lock = false; }

        if f3_down && !f3_key_lock {
            debugger.continue_execution();
            println!("DEBUG: continue");
            f3_key_lock = true;
        }
        if !f3_down { f3_key_lock = false; }

        if f4_down && !f4_key_lock {
            debugger.break_now(format!("Manual break at PC={:04X}", cpu.pc));
            debugger.print_status(&cpu);
            debugger.print_stop_disassembly(&mut bus, cpu.pc);
            f4_key_lock = true;
        }
        if !f4_down { f4_key_lock = false; }

        if f5_down && !f5_key_lock {
            bus.save_game();
            println!("Gra zapisana.");
            f5_key_lock = true;
        }
        if !f5_down { f5_key_lock = false; }

        if f6_down && !f6_key_lock {
            bus.load_game();
            println!("Gra wczytana.");
            f6_key_lock = true;
        }
        if !f6_down { f6_key_lock = false; }

        if f8_down && !f8_key_lock {
            let state = SaveState::capture(&cpu, &bus);
            save_to_file(&state, "save.state");
            println!("Savestate zapisany.");
            f8_key_lock = true;
        }
        if !f8_down { f8_key_lock = false; }

        if f9_down && !f9_key_lock {
            let state = load_from_file("save.state");
            state.restore(&mut cpu, &mut bus);
            println!("Savestate wczytany.");
            f9_key_lock = true;
        }
        if !f9_down { f9_key_lock = false; }

        if f10_down && !f10_key_lock {
            if debugger.has_breakpoint(cpu.pc) {
                println!("DEBUG: breakpoint już istnieje na {:04X}", cpu.pc);
            } else {
                debugger.add_breakpoint(cpu.pc);
                println!("DEBUG: breakpoint dodany na {:04X}", cpu.pc);
            }
            f10_key_lock = true;
        }
        if !f10_down { f10_key_lock = false; }

        if f11_down && !f11_key_lock {
            debugger.remove_breakpoint(cpu.pc);
            println!("DEBUG: breakpoint usunięty z {:04X}", cpu.pc);
            f11_key_lock = true;
        }
        if !f11_down { f11_key_lock = false; }

        if f12_down && !f12_key_lock {
            debugger.print_status(&cpu);
            debugger.print_stop_disassembly(&mut bus, cpu.pc);
            f12_key_lock = true;
        }
        if !f12_down { f12_key_lock = false; }

        if window.is_key_down(Key::D) { buttons &= !(1 << 0); }
        if window.is_key_down(Key::A) { buttons &= !(1 << 1); }
        if window.is_key_down(Key::W) { buttons &= !(1 << 2); }
        if window.is_key_down(Key::S) { buttons &= !(1 << 3); }
        if window.is_key_down(Key::J) { buttons &= !(1 << 4); }
        if window.is_key_down(Key::K) { buttons &= !(1 << 5); }
        if window.is_key_down(Key::U) { buttons &= !(1 << 6); }
        if window.is_key_down(Key::I) { buttons &= !(1 << 7); }

        bus.set_buttons(buttons);

        let mut frame_ready = false;

        while !frame_ready {
            while let Some(command) = debug_console.try_read() {
                let command = ConsoleCommand::parse(&command);

                debugger.execute_console_command(
                    command,
                    &cpu,
                    &mut bus,
                );
            }

            let execute_instruction = debugger.before_instruction(&cpu, &mut bus);

            if !execute_instruction {
                window.update();
                break;
            }

            let cycles = cpu.step(&mut bus);
            frame_ready = bus.step(cycles, &mut buffer);

            debugger.after_instruction_hook(&cpu, &mut bus);

            steps += 1;
        }

        if frame_ready {
            debugger.next_frame(&mut bus);

            let now = Instant::now();

            if now < next_frame_time {
                std::thread::sleep(next_frame_time - now);
            }

            window
                .update_with_buffer(&buffer, WIDTH, HEIGHT)
                .expect("Błąd aktualizacji ekranu");

            next_frame_time += frame_duration;

            // Jeśli emulator został chwilowo zatrzymany/debugger
            // i termin kolejnej klatki już minął, nie próbujemy
            // nadrabiać setek opóźnionych klatek.
            let now = Instant::now();

            if next_frame_time < now {
                next_frame_time = now + frame_duration;
            }
        }

        if steps % 50_000 == 0 && steps != 0 {
            bus.render_tile_debug(&mut tile_buffer);
            tile_window.update_with_buffer(&tile_buffer, TILE_DEBUG_WIDTH, TILE_DEBUG_HEIGHT).unwrap();
        }
    }
}

fn parse_hex_u16(value: &str) -> Option<u16> {
    let value = value.trim_start_matches('$');
    u16::from_str_radix(value, 16).ok()
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
    }
}
