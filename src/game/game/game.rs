use crate::game::{Cpu, GameMemory};
use crate::game::save_state::{CpuState, SaveState, SAVE_STATE_PATH};
use crate::input::Input;
use crate::audio::Audio;
use crate::rom::{Cartridge, Rom};

const PPU_REGISTERS: &[u16] = &[
    0xFF40, 0xFF41, 0xFF42, 0xFF43, 0xFF44, 0xFF45, 0xFF47, 0xFF4A, 0xFF4B,
    0xFF68, 0xFF6A,
];

const APU_REGISTERS: &[u16] = &[
    0xFF10, 0xFF11, 0xFF12, 0xFF13, 0xFF14,
    0xFF16, 0xFF17, 0xFF18, 0xFF19,
    0xFF1A, 0xFF1B, 0xFF1C, 0xFF1D, 0xFF1E,
    0xFF20, 0xFF21, 0xFF22, 0xFF23,
    0xFF24, 0xFF25, 0xFF26,
];

const GENERIC_IO_REGISTERS: &[u16] = &[
    0xFF03, 0xFF08, 0xFF09, 0xFF0A, 0xFF0B, 0xFF0C, 0xFF0D, 0xFF0E,
    0xFF48, 0xFF49, 0xFF4C, 0xFF4D, 0xFF4E, 0xFF50,
    0xFF51, 0xFF52, 0xFF53, 0xFF54, 0xFF55,
    0xFF56, 0xFF57, 0xFF58, 0xFF59, 0xFF5A, 0xFF5B, 0xFF5C, 0xFF5D,
    0xFF5E, 0xFF5F, 0xFF60, 0xFF61, 0xFF62, 0xFF63, 0xFF64, 0xFF65,
    0xFF66, 0xFF67, 0xFF6C, 0xFF6D, 0xFF6E, 0xFF6F,
    0xFF71, 0xFF72, 0xFF73, 0xFF74, 0xFF75, 0xFF76, 0xFF77, 0xFF78,
    0xFF79, 0xFF7A, 0xFF7B, 0xFF7C, 0xFF7D, 0xFF7E, 0xFF7F,
];

pub struct Game {
    running: bool,
    cpu: Cpu,
    memory: GameMemory,
}

impl Game {
    pub fn new(rom: Rom) -> Self {
        let mut cpu = Cpu::new();
        cpu.reset_cgb();

        let mut game = Self {
            running: true,
            cpu,
            memory: GameMemory::new(Cartridge::new(rom)),
        };

        match game.load_state_from_file(SAVE_STATE_PATH) {
            Ok(()) => println!("Save-state loaded: {SAVE_STATE_PATH}"),
            Err(error) if !std::path::Path::new(SAVE_STATE_PATH).exists() => {
                println!("No save-state found; starting a new session.");
                let _ = error;
            }
            Err(error) => eprintln!("Save-state ignored: {error}"),
        }

        game
    }

    pub fn update(&mut self, input: &Input, _delta_time: f32) {
        self.apply_input(input);
        while !self.frame_ready() {
            self.step();
        }
    }

    pub fn apply_input(&mut self, input: &Input) {
        if input.escape_pressed() {
            match self.save_state_to_file(SAVE_STATE_PATH) {
                Ok(()) => println!("Game saved: {SAVE_STATE_PATH}"),
                Err(error) => eprintln!("Failed to save game: {error}"),
            }
            self.running = false;
            return;
        }

        use crate::input::GameButton;
        for button in [GameButton::Right, GameButton::Left, GameButton::Up, GameButton::Down,
            GameButton::A, GameButton::B, GameButton::Select, GameButton::Start] {
            self.memory.set_joypad_button(button as u8, input.is_pressed(button));
        }
    }

    pub fn step(&mut self) -> u32 {
        self.memory.set_cpu_pc(self.cpu.pc);
        let mut cycles = self.cpu.step(&mut self.memory);
        let mut dma_stall_cycles = self.memory.step(cycles);
        while dma_stall_cycles != 0 {
            cycles += dma_stall_cycles;
            dma_stall_cycles = self.memory.step(dma_stall_cycles);
        }
        cycles
    }

    pub fn is_running(&self) -> bool { self.running }
    pub fn memory(&self) -> &GameMemory { &self.memory }
    pub fn memory_mut(&mut self) -> &mut GameMemory { &mut self.memory }
    pub fn set_audio(&mut self, audio: Audio) { self.memory.set_audio(audio); }
    pub fn cpu(&self) -> &Cpu { &self.cpu }
    pub fn framebuffer(&self) -> &[u32; 160 * 144] { self.memory.framebuffer() }
    pub fn frame_ready(&self) -> bool { self.memory.frame_ready() || !self.running }
    pub fn take_frame_ready(&mut self) -> bool { self.memory.take_frame_ready() }

    pub fn cartridge(&self) -> &Cartridge { self.memory.cartridge() }
    pub fn cartridge_mut(&mut self) -> &mut Cartridge { self.memory.cartridge_mut() }
    pub fn rom(&self) -> &Rom { self.memory.cartridge().rom() }
    pub fn rom_bank(&self) -> u16 { self.memory.cartridge().rom_bank() }

    pub fn read(&self, address: u16) -> u8 { self.memory.read(address) }
    pub fn write(&mut self, address: u16, value: u8) { self.memory.write(address, value); }
    pub fn read_word(&self, address: u16) -> u16 { self.memory.read_word(address) }
    pub fn write_word(&mut self, address: u16, value: u16) { self.memory.write_word(address, value); }

    pub fn print_vram_diagnostics(&self) { self.memory.print_vram_diagnostics(); }

    pub fn select_rom_bank(&mut self, bank: u16) {
        self.memory.write(0x2000, bank as u8);
        self.memory.write(0x3000, (bank >> 8) as u8);
    }

    pub fn save_state_to_file(&mut self, path: impl AsRef<std::path::Path>) -> Result<(), String> {
        let state = self.capture_save_state()?;
        state.save_atomic(path)
    }

    pub fn load_state_from_file(&mut self, path: impl AsRef<std::path::Path>) -> Result<(), String> {
        let state = SaveState::load(path)?;
        let expected_hash = self.rom_hash();
        if state.rom_hash != expected_hash {
            return Err("Save-state belongs to a different ROM.".to_string());
        }
        self.restore_save_state(state)
    }

    fn rom_hash(&self) -> [u8; 16] {
        let mut bytes = Vec::with_capacity(self.rom().size());
        for address in 0..self.rom().size() {
            bytes.push(self.rom().read(address));
        }
        md5::compute(bytes).0
    }

    fn read_range(&self, start: u16, len: usize) -> Vec<u8> {
        (0..len).map(|i| self.memory.read(start.wrapping_add(i as u16))).collect()
    }

    fn write_range(&mut self, start: u16, data: &[u8]) {
        for (i, &value) in data.iter().enumerate() {
            self.memory.write(start.wrapping_add(i as u16), value);
        }
    }

    fn capture_save_state(&mut self) -> Result<SaveState, String> {
        let mut state = SaveState::new();
        state.rom_hash = self.rom_hash();

        state.cpu = CpuState {
            a: self.cpu.a, f: self.cpu.f, b: self.cpu.b, c: self.cpu.c,
            d: self.cpu.d, e: self.cpu.e, h: self.cpu.h, l: self.cpu.l,
            sp: self.cpu.sp, pc: self.cpu.pc,
            ime: self.cpu.ime, ime_pending: self.cpu.ime_pending,
            halted: self.cpu.halted, halt_bug: self.cpu.halt_bug,
            opcode_counts: self.cpu.opcode_counts,
        };

        let previous_vram_bank = self.memory.read(0xFF4F) & 1;
        self.memory.write(0xFF4F, 0);
        state.vram[..0x2000].copy_from_slice(&self.read_range(0x8000, 0x2000));
        self.memory.write(0xFF4F, 1);
        state.vram[0x2000..].copy_from_slice(&self.read_range(0x8000, 0x2000));
        self.memory.write(0xFF4F, previous_vram_bank);
        state.vram_bank = previous_vram_bank;

        state.wram[..0x1000].copy_from_slice(&self.read_range(0xC000, 0x1000));
        let previous_wram_bank = self.memory.read(0xFF70) & 7;
        for bank in 1..=7u8 {
            self.memory.write(0xFF70, bank);
            let offset = bank as usize * 0x1000;
            state.wram[offset..offset + 0x1000].copy_from_slice(&self.read_range(0xD000, 0x1000));
        }
        self.memory.write(0xFF70, previous_wram_bank.max(1));
        state.wram_bank = previous_wram_bank.max(1);

        state.oam.copy_from_slice(&self.read_range(0xFE00, 0x00A0));
        state.hram.copy_from_slice(&self.read_range(0xFF80, 0x007F));
        state.ppu_registers = PPU_REGISTERS.iter().map(|&a| self.memory.read(a)).collect();
        state.bg_palette_ram = (0..64).map(|i| { self.memory.write(0xFF68, i); self.memory.read(0xFF69) }).collect();
        state.obj_palette_ram = (0..64).map(|i| { self.memory.write(0xFF6A, i); self.memory.read(0xFF6B) }).collect();
        state.timer_registers = (0xFF04..=0xFF07).map(|a| self.memory.read(a)).collect();
        state.serial_registers = (0xFF01..=0xFF02).map(|a| self.memory.read(a)).collect();
        state.joypad_register = self.memory.read(0xFF00);
        state.interrupt_registers = [self.memory.read(0xFF0F), self.memory.read(0xFFFF)];
        state.hdma_registers = std::array::from_fn(|i| self.memory.read(0xFF51 + i as u16));
        state.speed_registers = [self.memory.read(0xFF4D), 0];
        state.apu_registers = APU_REGISTERS.iter().map(|&a| self.memory.read(a)).collect();
        state.wave_ram = std::array::from_fn(|i| self.memory.read(0xFF30 + i as u16));
        state.cartridge_ram.copy_from_slice(self.cartridge().ram());
        state.rom_bank = self.cartridge().rom_bank();
        state.ram_bank = self.cartridge().ram_bank();
        state.ram_enabled = self.cartridge().ram_enabled();
        state.generic_io = GENERIC_IO_REGISTERS.iter().map(|&a| self.memory.read(a)).collect();

        Ok(state)
    }

    fn restore_save_state(&mut self, state: SaveState) -> Result<(), String> {
        self.cpu.a = state.cpu.a; self.cpu.f = state.cpu.f;
        self.cpu.b = state.cpu.b; self.cpu.c = state.cpu.c;
        self.cpu.d = state.cpu.d; self.cpu.e = state.cpu.e;
        self.cpu.h = state.cpu.h; self.cpu.l = state.cpu.l;
        self.cpu.sp = state.cpu.sp; self.cpu.pc = state.cpu.pc;
        self.cpu.ime = state.cpu.ime; self.cpu.ime_pending = state.cpu.ime_pending;
        self.cpu.halted = state.cpu.halted; self.cpu.halt_bug = state.cpu.halt_bug;
        self.cpu.opcode_counts = state.cpu.opcode_counts;

        self.memory.write(0xFF4F, 0);
        self.write_range(0x8000, &state.vram[..0x2000]);
        self.memory.write(0xFF4F, 1);
        self.write_range(0x8000, &state.vram[0x2000..]);
        self.memory.write(0xFF4F, state.vram_bank & 1);

        self.write_range(0xC000, &state.wram[..0x1000]);
        for bank in 1..=7u8 {
            self.memory.write(0xFF70, bank);
            let offset = bank as usize * 0x1000;
            self.write_range(0xD000, &state.wram[offset..offset + 0x1000]);
        }
        self.memory.write(0xFF70, state.wram_bank.max(1));
        self.write_range(0xFE00, &state.oam);
        self.write_range(0xFF80, &state.hram);

        for (&address, &value) in PPU_REGISTERS.iter().zip(state.ppu_registers.iter()) {
            if address != 0xFF44 { self.memory.write(address, value); }
        }
        for (i, &value) in state.bg_palette_ram.iter().enumerate() {
            self.memory.write(0xFF68, i as u8); self.memory.write(0xFF69, value);
        }
        for (i, &value) in state.obj_palette_ram.iter().enumerate() {
            self.memory.write(0xFF6A, i as u8); self.memory.write(0xFF6B, value);
        }
        for (i, &value) in state.timer_registers.iter().enumerate() { self.memory.write(0xFF04 + i as u16, value); }
        for (i, &value) in state.serial_registers.iter().enumerate() { self.memory.write(0xFF01 + i as u16, value); }
        self.memory.write(0xFF00, state.joypad_register);
        self.memory.write(0xFF0F, state.interrupt_registers[0]);
        self.memory.write(0xFFFF, state.interrupt_registers[1]);
        for (i, &value) in state.hdma_registers.iter().enumerate() { self.memory.write(0xFF51 + i as u16, value); }
        self.memory.write(0xFF4D, state.speed_registers[0]);

        for (address, &value) in GENERIC_IO_REGISTERS.iter().zip(state.generic_io.iter()) {
            if !matches!(*address, 0xFF4D | 0xFF51..=0xFF55) {
                self.memory.write(*address, value);
            }
        }

        for (address, &value) in APU_REGISTERS.iter().zip(state.apu_registers.iter()) {
            self.memory.write(*address, value);
        }
        for (i, &value) in state.wave_ram.iter().enumerate() { self.memory.write(0xFF30 + i as u16, value); }

        self.memory.write(0x0000, 0x0A);
        for bank in 0..4u8 {
            self.memory.write(0x4000, bank);
            let offset = bank as usize * 0x2000;
            for (i, &value) in state.cartridge_ram[offset..offset + 0x2000].iter().enumerate() {
                self.memory.write(0xA000 + i as u16, value);
            }
        }
        self.memory.write(0x2000, state.rom_bank as u8);
        self.memory.write(0x3000, (state.rom_bank >> 8) as u8);
        self.memory.write(0x4000, state.ram_bank & 3);
        self.memory.write(0x0000, if state.ram_enabled { 0x0A } else { 0x00 });

        Ok(())
    }
}
