use super::reader::{Rom, ROM_BANK_SIZE};

/// MBC5 ROM bank register.
///
/// MBC5 selects a 9-bit ROM bank number:
/// - lower 8 bits: 0x2000-0x2FFF
/// - bit 8:         0x3000-0x3FFF
#[derive(Debug, Clone, Copy)]
pub struct RomBank {
    bank: u16,
}

impl RomBank {
    pub const DEFAULT_BANK: u16 = 1;
    pub const MAX_BANK: u16 = 0x01FF;

    pub fn new() -> Self {
        Self {
            bank: Self::DEFAULT_BANK,
        }
    }

    pub fn bank(&self) -> u16 {
        self.bank
    }

    /// Writes the low 8 bits of the MBC5 ROM bank register.
    pub fn write_low(&mut self, value: u8) {
        self.bank = (self.bank & 0x0100) | value as u16;
    }

    /// Writes bit 8 of the MBC5 ROM bank register.
    pub fn write_high(&mut self, value: u8) {
        self.bank = (self.bank & 0x00FF) | (((value & 0x01) as u16) << 8);
    }

    pub fn set(&mut self, bank: u16) {
        self.bank = bank & Self::MAX_BANK;
    }

    pub fn read(&self, rom: &Rom, address: u16) -> u8 {
        assert!(
            (0x4000..=0x7FFF).contains(&address),
            "MBC5 banked ROM address out of range: {:04X}",
            address
        );

        let bank = (self.bank as usize) % rom.bank_count();
        rom.bank(bank)[address as usize - ROM_BANK_SIZE]
    }
}

impl Default for RomBank {
    fn default() -> Self {
        Self::new()
    }
}
