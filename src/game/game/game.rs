use crate::input::Input;
use crate::rom::{Rom, RomBank};

pub struct Game {
    running: bool,
    rom: Rom,
    rom_bank: RomBank,
}

impl Game {
    pub fn new(rom: Rom) -> Self {
        Self {
            running: true,
            rom,
            rom_bank: RomBank::new(),
        }
    }

    pub fn update(&mut self, _input: &Input, _delta_time: f32) {
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn rom(&self) -> &Rom {
        &self.rom
    }

    pub fn rom_bank(&self) -> u16 {
        self.rom_bank.bank()
    }

    pub fn read_rom(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x3FFF => self.rom.read_cpu(address, 0),
            0x4000..=0x7FFF => self.rom_bank.read(&self.rom, address),
            _ => panic!("Adres {:04X} nie jest adresem ROM", address),
        }
    }

    pub fn select_rom_bank(&mut self, bank: u16) {
        self.rom_bank.set(bank);
    }
}
