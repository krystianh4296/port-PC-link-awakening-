use crate::game::{Cpu, GameMemory};
use crate::input::Input;
use crate::rom::Rom;
use crate::rom::Cartridge;

pub struct Game {
    running: bool,
    cpu: Cpu,
    memory: GameMemory,
}

impl Game {
    pub fn new(rom: Rom) -> Self {
        Self {
            running: true,
            cpu: Cpu::new(),
            memory: GameMemory::new(Cartridge::new(rom)),
        }
    }

    pub fn update(
        &mut self,
        _input: &Input,
        _delta_time: f32,
    ) {
        // około jedna instrukcja CPU na wywołanie.
        self.cpu.step(&mut self.memory);
        println!(
            "PC={:04X} OP={:02X}",
            self.cpu.pc,
            self.memory.read(self.cpu.pc)
        );
    }
    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn memory(&self) -> &GameMemory {
        &self.memory
    }

    pub fn memory_mut(&mut self) -> &mut GameMemory {
        &mut self.memory
    }

    pub fn cartridge(&self) -> &Cartridge {
        self.memory.cartridge()
    }

    pub fn cartridge_mut(&mut self) -> &mut Cartridge {
        self.memory.cartridge_mut()
    }

    pub fn rom(&self) -> &Rom {
        self.memory.cartridge().rom()
    }

    pub fn rom_bank(&self) -> u16 {
        self.memory.cartridge().rom_bank()
    }

    pub fn read(&self, address: u16) -> u8 {
        self.memory.read(address)
    }

    pub fn write(&mut self, address: u16, value: u8) {
        self.memory.write(address, value);
    }

    pub fn read_word(&self, address: u16) -> u16 {
        self.memory.read_word(address)
    }

    pub fn write_word(&mut self, address: u16, value: u16) {
        self.memory.write_word(address, value);
    }

    pub fn select_rom_bank(&mut self, bank: u16) {
        self.memory.write(0x2000, bank as u8);
        self.memory.write(0x3000, (bank >> 8) as u8);
    }
}
