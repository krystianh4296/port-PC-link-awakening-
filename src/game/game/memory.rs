use crate::game::hardware::timer::Timer;
use crate::rom::Cartridge;

/// CPU-visible memory map for the native Link's Awakening DX port.
///
/// Cartridge-owned regions are delegated to `Cartridge` while the internal
/// Game Boy memory regions are kept here. This gives the future CPU one
/// stable read/write interface instead of exposing individual memory arrays.
#[derive(Debug)]
pub struct GameMemory {
    cartridge: Cartridge,
    vram: [u8; 0x2000],
    wram: [u8; 0x8000],
    oam: [u8; 0x00A0],
    io: [u8; 0x0080],
    hram: [u8; 0x007F],
    ie: u8,
    timer: Timer,
}

impl GameMemory {
    pub fn new(cartridge: Cartridge) -> Self {
        Self {
            cartridge,
            vram: [0; 0x2000],
            wram: [0; 0x8000],
            oam: [0; 0x00A0],
            io: [0; 0x0080],
            hram: [0; 0x007F],
            ie: 0,
            timer: Timer::new(),
        }
    }

    pub fn cartridge(&self) -> &Cartridge {
        &self.cartridge
    }

    pub fn cartridge_mut(&mut self) -> &mut Cartridge {
        &mut self.cartridge
    }

    /// Reads one byte using the Game Boy CPU address map.
    pub fn read(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x7FFF | 0xA000..=0xBFFF => self.cartridge.read(address),
            0x8000..=0x9FFF => self.vram[(address - 0x8000) as usize],
            0xC000..=0xDFFF => self.wram[(address - 0xC000) as usize],
            0xE000..=0xFDFF => self.wram[(address - 0xE000) as usize],
            0xFE00..=0xFE9F => self.oam[(address - 0xFE00) as usize],
            0xFEA0..=0xFEFF => 0xFF,
            0xFF04..=0xFF07 => self.timer.read(address),
            0xFF00..=0xFF03 | 0xFF08..=0xFF7F => {
                self.io[(address - 0xFF00) as usize]
            }
            0xFF80..=0xFFFE => self.hram[(address - 0xFF80) as usize],
            0xFFFF => self.ie,
        }
    }

    /// Writes one byte using the Game Boy CPU address map.
    pub fn write(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x7FFF | 0xA000..=0xBFFF => self.cartridge.write(address, value),
            0x8000..=0x9FFF => self.vram[(address - 0x8000) as usize] = value,
            0xC000..=0xDFFF => self.wram[(address - 0xC000) as usize] = value,
            0xE000..=0xFDFF => self.wram[(address - 0xE000) as usize] = value,
            0xFE00..=0xFE9F => self.oam[(address - 0xFE00) as usize] = value,
            0xFEA0..=0xFEFF => {},
            0xFF04..=0xFF07 => {
                self.timer.write(address, value);

                if self.timer.take_interrupt() {
                    self.io[0x0F] |= 0x04;
                }
            }
            0xFF00..=0xFF03 | 0xFF08..=0xFF7F => {
                self.io[(address - 0xFF00) as usize] = value;
            }
            0xFF80..=0xFFFE => self.hram[(address - 0xFF80) as usize] = value,
            0xFFFF => self.ie = value,
        }
    }

    pub fn read_word(&self, address: u16) -> u16 {
        let lo = self.read(address);
        let hi = self.read(address.wrapping_add(1));
        u16::from_le_bytes([lo, hi])
    }

    pub fn write_word(&mut self, address: u16, value: u16) {
        let [lo, hi] = value.to_le_bytes();
        self.write(address, lo);
        self.write(address.wrapping_add(1), hi);
    }
    pub fn step(&mut self, cycles: u32) {
        self.timer.step(cycles);

        if self.timer.take_interrupt() {
            self.io[0x0F] |= 0x04;
        }
    }
    #[cfg(test)]
    fn new_test() -> Self {
        use crate::rom::Rom;
        use std::path::Path;

        let rom = Rom::load(Path::new("test.rom"))
            .expect("test ROM required");
        Self::new(Cartridge::new(rom))
    }
}

#[cfg(test)]
mod tests {
    use super::GameMemory;
    use crate::rom::{Cartridge, Rom};

    fn test_memory() -> GameMemory {
        let rom_path = std::env::var("GAMEBOY_ROM")
            .expect("GAMEBOY_ROM must point to a .gb/.gbc ROM");

        let rom = Rom::load(rom_path).expect("failed to load test ROM");
        GameMemory::new(Cartridge::new(rom))
    }

    #[test]
    fn timer_overflow_sets_timer_interrupt_flag() {
        let mut memory = test_memory();

        memory.write(0xFF05, 0xFF);
        memory.write(0xFF06, 0x42);
        memory.write(0xFF07, 0x05);

        memory.step(16);

        assert_eq!(memory.read(0xFF05), 0x00);

        memory.step(4);

        assert_eq!(memory.read(0xFF05), 0x42);
        assert_eq!(memory.read(0xFF0F) & 0x04, 0x04);
    }
}