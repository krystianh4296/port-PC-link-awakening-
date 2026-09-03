use std::fs;
use std::path::{Path, PathBuf};
pub struct Rom {
    data: Vec<u8>,
    path: PathBuf,
}

impl Rom {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let path = path.as_ref().to_path_buf();

        let data = fs::read(&path).map_err(|e| format!("Nie udało się wczytać ROM-u: {}", e))?;

        if data.len() < 0x150 {
            return Err("ROM jest za mały.".to_string());
        }

        println!(
            "ROM size: {} bytes ({:.1} KiB)",
            data.len(),
            data.len() as f64 / 1024.0
        );

        println!("Cartridge type: {:02X}", data[0x147]);
        println!("ROM size code: {:02X}", data[0x148]);
        println!("RAM size code: {:02X}", data[0x149]);

        Ok(Self { data, path })
    }

    pub fn save_path(&self) -> PathBuf {
        self.path.with_extension("sav")
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn read(&self, address: usize) -> u8 {
        self.data.get(address).copied().unwrap_or(0xFF)
    }

    pub fn title(&self) -> String {
        self.data[0x134..0x143]
            .iter()
            .take_while(|&&b| b != 0)
            .map(|&b| b as char)
            .collect()
    }

    pub fn cartridge_type(&self) -> u8 {
        self.data[0x147]
    }

    pub fn rom_size_code(&self) -> u8 {
        self.data[0x148]
    }

    pub fn ram_size_code(&self) -> u8 {
        self.data[0x149]
    }

    pub fn rom_bank_count(&self) -> usize {
        (self.data.len() / 0x4000).max(1)
    }
}

// ============================================================
// MBC1
// ============================================================

fn ram_size_bytes(code: u8) -> usize {
    match code {
        0x01 => 0x0800,  // 2 KB
        0x02 => 0x2000,  // 8 KB
        0x03 => 0x8000,  // 32 KB
        0x04 => 0x20000, // 128 KB
        0x05 => 0x10000, // 64 KB
        _ => 0,
    }
}

pub struct Mbc1 {
    pub(crate) rom_bank_low: u8,
    pub(crate) rom_bank_high: u8,
    pub(crate) banking_mode: u8,

    pub(crate) ram_enabled: bool,
    pub(crate) ram: Vec<u8>,
}

impl Mbc1 {
    pub fn new(rom: &Rom) -> Self {
        Self {
            rom_bank_low: 1,
            rom_bank_high: 0,
            banking_mode: 0,

            ram_enabled: false,
            ram: vec![0; ram_size_bytes(rom.ram_size_code())],
        }
    }

    // --------------------------------------------------------
    // MBC1
    // --------------------------------------------------------

    pub fn write_ram_enable(&mut self, value: u8) {
        self.ram_enabled = (value & 0x0F) == 0x0A;
    }

    pub fn select_rom_bank(&mut self, value: u8) {
        let mut bank = value & 0x1F;

        if bank == 0 {
            bank = 1;
        }

        self.rom_bank_low = bank;
    }

    pub fn select_ram_bank(&mut self, value: u8) {
        self.rom_bank_high = value & 0x03;
    }

    pub fn select_banking_mode(&mut self, value: u8) {
        self.banking_mode = value & 0x01;
    }

    fn rom_bank_4000(&self) -> usize {
        let low = self.rom_bank_low as usize;
        let high = self.rom_bank_high as usize;

        if self.banking_mode == 0 {
            (high << 5) | low
        } else {
            low
        }
    }

    fn rom_bank_0000(&self) -> usize {
        if self.banking_mode == 0 {
            0
        } else {
            (self.rom_bank_high as usize) << 5
        }
    }

    fn ram_bank(&self) -> usize {
        if self.banking_mode == 0 {
            0
        } else {
            self.rom_bank_high as usize
        }
    }

    // --------------------------------------------------------
    // ROM
    // --------------------------------------------------------

    pub fn read(&self, rom: &Rom, address: u16) -> u8 {
        match address {
            0x0000..=0x3FFF => {
                let bank = self.rom_bank_0000() % rom.rom_bank_count();

                rom.read(bank * 0x4000 + address as usize)
            }

            0x4000..=0x7FFF => {
                let bank = self.rom_bank_4000() % rom.rom_bank_count();

                rom.read(bank * 0x4000 + (address as usize - 0x4000))
            }

            _ => 0xFF,
        }
    }

    // --------------------------------------------------------
    // SRAM
    // --------------------------------------------------------

    pub fn write_ram(&mut self, address: u16, value: u8) {
        if !self.ram_enabled || self.ram.is_empty() {
            return;
        }

        let ram_banks = (self.ram.len() / 0x2000).max(1);

        let bank = self.ram_bank() % ram_banks;

        let offset = bank * 0x2000 + (address as usize - 0xA000);

        if let Some(slot) = self.ram.get_mut(offset) {
            *slot = value;
        }
    }

    pub fn read_ram(&self, address: u16) -> u8 {
        if !self.ram_enabled || self.ram.is_empty() {
            return 0xFF;
        }

        let ram_banks = (self.ram.len() / 0x2000).max(1);

        let bank = self.ram_bank() % ram_banks;

        let offset = bank * 0x2000 + (address as usize - 0xA000);

        self.ram.get(offset).copied().unwrap_or(0xFF)
    }
    pub fn ram(&self) -> &[u8] {
        &self.ram
    }

    pub fn ram_mut(&mut self) -> &mut [u8] {
        &mut self.ram
    }
}
