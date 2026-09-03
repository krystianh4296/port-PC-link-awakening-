use crate::input::Input;
use crate::rom::{Cartridge, Rom};

pub struct Game {
    running: bool,
    cartridge: Cartridge,
}

impl Game {
    pub fn new(rom: Rom) -> Self {
        Self {
            running: true,
            cartridge: Cartridge::new(rom),
        }
    }

    pub fn update(&mut self, _input: &Input, _delta_time: f32) {
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn cartridge(&self) -> &Cartridge {
        &self.cartridge
    }

    pub fn cartridge_mut(&mut self) -> &mut Cartridge {
        &mut self.cartridge
    }

    pub fn rom(&self) -> &Rom {
        self.cartridge.rom()
    }

    pub fn rom_bank(&self) -> u16 {
        self.cartridge.rom_bank()
    }

    pub fn read_rom(&self, address: u16) -> u8 {
        self.cartridge.read(address)
    }

    pub fn write_cartridge(&mut self, address: u16, value: u8) {
        self.cartridge.write(address, value);
    }

    pub fn select_rom_bank(&mut self, bank: u16) {
        self.cartridge.write(0x2000, bank as u8);
        self.cartridge.write(0x3000, (bank >> 8) as u8);
    }
}
