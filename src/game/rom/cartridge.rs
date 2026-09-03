use super::{Rom, RomBank};

/// MBC5 cartridge address space used by Link's Awakening DX.
///
/// This layer owns the cartridge-side memory mapping:
/// - 0000-3FFF: fixed ROM bank 0
/// - 4000-7FFF: switchable ROM bank
/// - A000-BFFF: external cartridge RAM
/// - 2000-2FFF: MBC5 ROM bank low 8 bits
/// - 3000-3FFF: MBC5 ROM bank bit 8
/// - 4000-5FFF: MBC5 RAM bank register
/// - 0000-1FFF: external RAM enable
#[derive(Debug)]
pub struct Cartridge {
    rom: Rom,
    rom_bank: RomBank,
    ram: Vec<u8>,
    ram_enabled: bool,
    ram_bank: u8,
}

impl Cartridge {
    pub const RAM_BANK_SIZE: usize = 0x2000;
    pub const RAM_BANK_COUNT: usize = 4;

    pub fn new(rom: Rom) -> Self {
        Self {
            rom,
            rom_bank: RomBank::new(),
            ram: vec![0; Self::RAM_BANK_SIZE * Self::RAM_BANK_COUNT],
            ram_enabled: false,
            ram_bank: 0,
        }
    }

    pub fn rom(&self) -> &Rom {
        &self.rom
    }

    pub fn rom_bank(&self) -> u16 {
        self.rom_bank.bank()
    }

    pub fn ram_bank(&self) -> u8 {
        self.ram_bank
    }

    pub fn ram_enabled(&self) -> bool {
        self.ram_enabled
    }

    /// Reads an address from the cartridge address space.
    pub fn read(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x3FFF => self.rom.read_cpu(address, 0),
            0x4000..=0x7FFF => self.rom_bank.read(&self.rom, address),
            0xA000..=0xBFFF => self.read_ram(address),
            _ => 0xFF,
        }
    }

    /// Writes to the cartridge address space.
    ///
    /// ROM writes are interpreted as MBC5 register writes rather than
    /// modifying the ROM data itself.
    pub fn write(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x1FFF => {
                // MBC5 RAM enable: lower nibble 0x0A enables RAM.
                self.ram_enabled = (value & 0x0F) == 0x0A;
            }
            0x2000..=0x2FFF => {
                self.rom_bank.write_low(value);
            }
            0x3000..=0x3FFF => {
                self.rom_bank.write_high(value);
            }
            0x4000..=0x5FFF => {
                // MBC5 has four RAM banks on this 32 KiB cartridge.
                self.ram_bank = value & 0x03;
            }
            0xA000..=0xBFFF => {
                self.write_ram(address, value);
            }
            _ => {}
        }
    }

    fn read_ram(&self, address: u16) -> u8 {
        if !self.ram_enabled {
            return 0xFF;
        }

        let index = self.ram_index(address);
        self.ram[index]
    }

    fn write_ram(&mut self, address: u16, value: u8) {
        if !self.ram_enabled {
            return;
        }

        let index = self.ram_index(address);
        self.ram[index] = value;
    }

    fn ram_index(&self, address: u16) -> usize {
        let offset = (address - 0xA000) as usize;
        self.ram_bank as usize * Self::RAM_BANK_SIZE + offset
    }
}
