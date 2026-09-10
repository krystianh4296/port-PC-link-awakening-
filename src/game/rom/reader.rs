use std::fs;
use std::path::{Path, PathBuf};

pub const ROM_BANK_SIZE: usize = 0x4000;
pub const EXPECTED_ROM_SIZE: usize = 0x100000;
pub const EXPECTED_MD5: &str = "7351daa3c0a91d8f6fe2fbcca6182478";

#[derive(Debug, Clone)]
pub struct RomHeader {
    pub title: String,
    pub cartridge_type: u8,
    pub rom_size_code: u8,
    pub ram_size_code: u8,
}

#[derive(Debug)]
pub enum RomError {
    Io(std::io::Error),
    NotFound { path: PathBuf },
    TooSmall { actual: usize },
    InvalidSize { actual: usize, expected: usize },
    InvalidMd5 { actual: String, expected: &'static str },
}

impl std::fmt::Display for RomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(err) => write!(f, "Nie można odczytać ROM: {err}"),
            Self::NotFound { path } => write!(f, "Nie znaleziono pliku ROM: {}", path.display()),
            Self::TooSmall { actual } => write!(f, "ROM jest zbyt mały: {actual} bajtów"),
            Self::InvalidSize { actual, expected } => write!(f, "Nieprawidłowy rozmiar ROM: {actual} bajtów, oczekiwano {expected}"),
            Self::InvalidMd5 { actual, expected } => write!(f, "Nieprawidłowy ROM MD5: {actual}, oczekiwano {expected}"),
        }
    }
}

impl std::error::Error for RomError {}

impl From<std::io::Error> for RomError {
    fn from(err: std::io::Error) -> Self { Self::Io(err) }
}

#[derive(Debug, Clone)]
pub struct Rom {
    path: PathBuf,
    data: Vec<u8>,
    header: RomHeader,
}

impl Rom {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, RomError> {
        let requested = path.as_ref().to_path_buf();
        let resolved = resolve_rom_path(&requested).unwrap_or(requested.clone());

        if !resolved.is_file() {
            return Err(RomError::NotFound { path: resolved });
        }

        let data = fs::read(&resolved).map_err(RomError::Io)?;

        if data.len() < 0x150 {
            return Err(RomError::TooSmall { actual: data.len() });
        }
        if data.len() != EXPECTED_ROM_SIZE {
            return Err(RomError::InvalidSize { actual: data.len(), expected: EXPECTED_ROM_SIZE });
        }

        let digest = format!("{:x}", md5::compute(&data));
        if digest != EXPECTED_MD5 {
            return Err(RomError::InvalidMd5 { actual: digest, expected: EXPECTED_MD5 });
        }

        let title = data[0x134..0x144]
            .iter()
            .copied()
            .take_while(|&byte| byte != 0)
            .filter(|&byte| byte.is_ascii_graphic() || byte == b' ')
            .map(char::from)
            .collect();

        let header = RomHeader {
            title,
            cartridge_type: data[0x147],
            rom_size_code: data[0x148],
            ram_size_code: data[0x149],
        };

        Ok(Self { path: resolved, data, header })
    }

    pub fn path(&self) -> &Path { &self.path }
    pub fn header(&self) -> &RomHeader { &self.header }
    pub fn len(&self) -> usize { self.data.len() }
    pub fn bank_count(&self) -> usize { self.data.len() / ROM_BANK_SIZE }

    pub fn read(&self, address: usize) -> u8 { self.data[address] }

    pub fn read_word(&self, address: usize) -> u16 {
        u16::from_le_bytes([self.read(address), self.read(address + 1)])
    }

    pub fn bank(&self, bank: usize) -> &[u8] {
        let start = bank * ROM_BANK_SIZE;
        let end = start + ROM_BANK_SIZE;
        &self.data[start..end]
    }

    pub fn read_cpu(&self, address: u16, rom_bank: u16) -> u8 {
        match address {
            0x0000..=0x3FFF => self.data[address as usize],
            0x4000..=0x7FFF => {
                let bank = (rom_bank as usize) % self.bank_count();
                self.bank(bank)[address as usize - 0x4000]
            }
            _ => panic!("Adres {:04X} nie jest adresem ROM", address),
        }
    }

    pub fn tile_bytes(&self, address: usize) -> [u8; 16] {
        self.data[address..address + 16]
            .try_into()
            .expect("Tile poza ROM")
    }
}

fn resolve_rom_path(path: &Path) -> Option<PathBuf> {
    let requested = if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir().ok()?.join(path)
    };

    if requested.is_file() {
        return Some(requested);
    }

    let mut search_dirs = vec![];
    let cwd = std::env::current_dir().ok()?;
    let mut dir = cwd.clone();
    loop {
        search_dirs.push(dir.clone());
        if let Some(parent) = dir.parent() {
            if parent == dir {
                break;
            }
            dir = parent.to_path_buf();
        } else {
            break;
        }
    }

    if let Ok(exe) = std::env::current_exe() {
        if let Some(parent) = exe.parent() {
            search_dirs.push(parent.to_path_buf());
        }
    }

    // De-duplicate while preserving order.
    let mut seen = std::collections::HashSet::new();
    search_dirs.retain(|dir| seen.insert(dir.clone()));

    let path_name = path.file_name().and_then(|f| f.to_str()).unwrap_or_default();
    let candidate_names = [
        "Legend of Zelda, The - Link's Awakening DX (USA, Europe) (Rev 2).gbc",
        "Legend of Zelda, The - Link's Awakening DX (USA, Europe) (Rev 2).gb",
        "Legend of Zelda, The - Links Awakening (USA, Europe) (Rev 2).gbc",
        "Legend of Zelda, The - Links Awakening (USA, Europe) (Rev 2).gb",
        path_name,
    ];

    let mut matches = Vec::new();
    for dir in &search_dirs {
        for candidate_name in &candidate_names {
            if candidate_name.is_empty() { continue; }
            let candidate = dir.join(candidate_name);
            if candidate.is_file() {
                matches.push(candidate);
            }
        }

        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let file_name = entry.file_name();
                let name = file_name.to_string_lossy().to_ascii_lowercase();
                if name.contains("link") && name.contains("awakening") && (name.ends_with(".gb") || name.ends_with(".gbc")) {
                    let candidate = entry.path();
                    if candidate.is_file() {
                        matches.push(candidate);
                    }
                }
            }
        }
    }

    matches.into_iter().find(|p| p.is_file())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolves_explicit_project_rom_path_without_touching_global_cwd() {
        let temp_dir = std::env::temp_dir().join(format!("gb-port-test-{}", std::process::id()));
        let _ = std::fs::create_dir_all(&temp_dir);
        let rom_name = "Legend of Zelda, The - Link's Awakening DX (USA, Europe) (Rev 2).gbc";
        let rom_path = temp_dir.join(rom_name);
        std::fs::write(&rom_path, vec![0u8; 0x100000]).unwrap();

        assert_eq!(resolve_rom_path(&rom_path), Some(rom_path.clone()));

        let _ = std::fs::remove_file(rom_path);
        let _ = std::fs::remove_dir(temp_dir);
    }
}
