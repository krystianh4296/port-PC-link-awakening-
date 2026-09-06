use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

pub const SAVE_STATE_PATH: &str = "link_awakening.state";
const MAGIC: [u8; 8] = *b"LADXST01";
const VERSION: u32 = 1;

#[derive(Debug, Serialize, Deserialize)]
pub struct SaveState {
    pub magic: [u8; 8],
    pub version: u32,
    pub rom_hash: [u8; 16],
    pub cpu: CpuState,
    pub vram: Vec<u8>,
    pub wram: Vec<u8>,
    pub oam: Vec<u8>,
    pub hram: Vec<u8>,
    pub ppu_registers: Vec<u8>,
    pub bg_palette_ram: Vec<u8>,
    pub obj_palette_ram: Vec<u8>,
    pub timer_registers: Vec<u8>,
    pub serial_registers: Vec<u8>,
    pub joypad_register: u8,
    pub interrupt_registers: [u8; 2],
    pub hdma_registers: [u8; 5],
    pub speed_registers: [u8; 2],
    pub vram_bank: u8,
    pub wram_bank: u8,
    pub apu_registers: Vec<u8>,
    pub generic_io: Vec<u8>,
    pub wave_ram: [u8; 16],
    pub rom_bank: u16,
    pub ram_bank: u8,
    pub ram_enabled: bool,
    pub cartridge_ram: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CpuState {
    pub a: u8,
    pub f: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub sp: u16,
    pub pc: u16,
    pub ime: bool,
    pub ime_pending: bool,
    pub halted: bool,
    pub halt_bug: bool,
    pub opcode_counts: [u64; 256],
}

impl SaveState {
    pub fn new() -> Self {
        Self {
            magic: MAGIC,
            version: VERSION,
            rom_hash: [0; 16],
            cpu: CpuState {
                a: 0, f: 0, b: 0, c: 0, d: 0, e: 0, h: 0, l: 0,
                sp: 0, pc: 0, ime: false, ime_pending: false,
                halted: false, halt_bug: false, opcode_counts: [0; 256],
            },
            vram: vec![0; 0x4000],
            wram: vec![0; 0x8000],
            oam: vec![0; 0x00A0],
            hram: vec![0; 0x007F],
            ppu_registers: vec![0; 11],
            bg_palette_ram: vec![0; 64],
            obj_palette_ram: vec![0; 64],
            timer_registers: vec![0; 4],
            serial_registers: vec![0; 2],
            joypad_register: 0,
            interrupt_registers: [0; 2],
            hdma_registers: [0; 5],
            speed_registers: [0; 2],
            vram_bank: 0,
            wram_bank: 1,
            apu_registers: vec![0; 21],
            generic_io: Vec::new(),
            wave_ram: [0; 16],
            rom_bank: 1,
            ram_bank: 0,
            ram_enabled: false,
            cartridge_ram: vec![0; 0x8000],
        }
    }

    pub fn save_atomic(&self, path: impl AsRef<Path>) -> Result<(), String> {
        let path = path.as_ref();
        let tmp = path.with_extension("state.tmp");
        let encoded = bincode::serialize(self)
            .map_err(|e| format!("Nie udało się serializować save-state: {e}"))?;
        fs::write(&tmp, encoded)
            .map_err(|e| format!("Nie udało się zapisać save-state: {e}"))?;
        fs::rename(&tmp, path)
            .map_err(|e| format!("Nie udało się zatwierdzić save-state: {e}"))?;
        Ok(())
    }

    pub fn load(path: impl AsRef<Path>) -> Result<Self, String> {
        let data = fs::read(path.as_ref())
            .map_err(|e| format!("Nie udało się odczytać save-state: {e}"))?;
        let state: Self = bincode::deserialize(&data)
            .map_err(|e| format!("Nieprawidłowy save-state: {e}"))?;
        if state.magic != MAGIC {
            return Err("Nieprawidłowy nagłówek save-state.".to_string());
        }
        if state.version != VERSION {
            return Err(format!("Nieobsługiwana wersja save-state: {}", state.version));
        }
        if state.vram.len() != 0x4000 || state.wram.len() != 0x8000
            || state.oam.len() != 0x00A0 || state.hram.len() != 0x007F
            || state.ppu_registers.len() != 11 || state.bg_palette_ram.len() != 64
            || state.obj_palette_ram.len() != 64 || state.timer_registers.len() != 4
            || state.serial_registers.len() != 2 || state.apu_registers.len() != 21
            || state.cartridge_ram.len() != 0x8000
        {
            return Err("Save-state ma nieprawidłowy rozmiar pamięci.".to_string());
        }
        Ok(state)
    }
}
