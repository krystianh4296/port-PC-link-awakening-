use crate::game::{Cpu, GameMemory};
use crate::input::Input;
use crate::audio::Audio;
use crate::rom::{Cartridge, Rom};

pub struct Game {
    running: bool,
    cpu: Cpu,
    memory: GameMemory,
}

impl Game {
    pub fn new(rom: Rom) -> Self {
        let mut cpu = Cpu::new();
        cpu.reset_cgb();

        Self {
            running: true,
            cpu,
            memory: GameMemory::new(Cartridge::new(rom)),
        }
    }

    pub fn update(&mut self, _input: &Input, _delta_time: f32) {
        self.apply_input(_input);
        while !self.memory.frame_ready() {
            self.step();
        }
    }

    pub fn apply_input(&mut self, input: &Input) {
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

        // DMA advances the hardware clocks but does not execute an LR35902
        // instruction. A transfer can finish at another mode boundary, so
        // consume all resulting stall time before returning to the CPU.
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
    pub fn frame_ready(&self) -> bool { self.memory.frame_ready() }
    pub fn take_frame_ready(&mut self) -> bool { self.memory.take_frame_ready() }

    pub fn cartridge(&self) -> &Cartridge { self.memory.cartridge() }
    pub fn cartridge_mut(&mut self) -> &mut Cartridge { self.memory.cartridge_mut() }
    pub fn rom(&self) -> &Rom { self.memory.cartridge().rom() }
    pub fn rom_bank(&self) -> u16 { self.memory.cartridge().rom_bank() }

    pub fn read(&self, address: u16) -> u8 { self.memory.read(address) }
    pub fn write(&mut self, address: u16, value: u8) { self.memory.write(address, value); }
    pub fn read_word(&self, address: u16) -> u16 { self.memory.read_word(address) }
    pub fn write_word(&mut self, address: u16, value: u16) { self.memory.write_word(address, value); }

    pub fn print_vram_diagnostics(&self) {
        self.memory.print_vram_diagnostics();
    }

    pub fn select_rom_bank(&mut self, bank: u16) {
        self.memory.write(0x2000, bank as u8);
        self.memory.write(0x3000, (bank >> 8) as u8);
    }
}
