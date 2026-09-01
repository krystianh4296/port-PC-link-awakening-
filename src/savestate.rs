use std::fs;
use std::path::{Path, PathBuf};

use crate::cpu::Cpu;

const SAVE_STATE_MAGIC: &[u8; 8] = b"GBSTATE1";

pub struct SaveState;

impl SaveState {
    pub fn path_from_rom(rom_path: &Path) -> PathBuf {
        rom_path.with_extension("state")
    }

    pub fn save_cpu(cpu: &Cpu, path: &Path) -> Result<(), String> {
        let mut data = Vec::new();

        // Nagłówek
        data.extend_from_slice(SAVE_STATE_MAGIC);

        // CPU
        data.push(cpu.a);
        data.push(cpu.f);

        data.push(cpu.b);
        data.push(cpu.c);

        data.push(cpu.d);
        data.push(cpu.e);

        data.push(cpu.h);
        data.push(cpu.l);

        data.extend_from_slice(&cpu.sp.to_le_bytes());
        data.extend_from_slice(&cpu.pc.to_le_bytes());

        data.push(cpu.ime as u8);
        data.push(cpu.ime_pending as u8);
        data.push(cpu.halted as u8);

        fs::write(path, data)
            .map_err(|e| format!("Nie można zapisać Save State: {}", e))?;

        println!(
            "SAVE STATE: zapisano -> {}",
            path.display()
        );

        Ok(())
    }

    pub fn load_cpu(cpu: &mut Cpu, path: &Path) -> Result<(), String> {
        let data = fs::read(path)
            .map_err(|e| format!("Nie można wczytać Save State: {}", e))?;

        if data.len() < 8 {
            return Err("Save State jest uszkodzony.".to_string());
        }

        if &data[0..8] != SAVE_STATE_MAGIC {
            return Err("Nieprawidłowy plik Save State.".to_string());
        }

        let mut offset = 8;

        cpu.a = data[offset];
        offset += 1;

        cpu.f = data[offset] & 0xF0;
        offset += 1;

        cpu.b = data[offset];
        offset += 1;

        cpu.c = data[offset];
        offset += 1;

        cpu.d = data[offset];
        offset += 1;

        cpu.e = data[offset];
        offset += 1;

        cpu.h = data[offset];
        offset += 1;

        cpu.l = data[offset];
        offset += 1;

        cpu.sp = u16::from_le_bytes([
            data[offset],
            data[offset + 1],
        ]);
        offset += 2;

        cpu.pc = u16::from_le_bytes([
            data[offset],
            data[offset + 1],
        ]);
        offset += 2;

        cpu.ime = data[offset] != 0;
        offset += 1;

        cpu.ime_pending = data[offset] != 0;
        offset += 1;

        cpu.halted = data[offset] != 0;

        println!(
            "SAVE STATE: wczytano <- {} | PC={:04X} SP={:04X}",
            path.display(),
            cpu.pc,
            cpu.sp
        );

        Ok(())
    }
}