use crate::{bus::Bus, cpu::Cpu};

#[derive(Clone)]
pub struct SaveState {
    pub cpu: CpuState,
    pub bus: BusState,
}

#[derive(Clone)]
pub struct CpuState {
    pub a: u8,
    pub f: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,

    pub pc: u16,
    pub sp: u16,

    pub ime: bool,
    pub halted: bool,
}

#[derive(Clone)]
pub struct BusState {
    pub vram: [u8; 0x2000],
    pub wram: [u8; 0x2000],
    pub eram: [u8; 0x2000],
    pub oam: [u8; 0xA0],
    pub hram: [u8; 0x7F],

    pub io: [u8; 0x80],

    pub ie: u8,
    pub if_reg: u8,

    pub lcdc: u8,
    pub stat: u8,
    pub ly: u8,
    pub lyc: u8,
    pub scx: u8,
    pub scy: u8,

    pub bgp: u8,
    pub obp0: u8,
    pub obp1: u8,

    pub lcd_cycles: u32,
    pub timer_cycles: u32,

    pub current_rom_bank: u8,
    pub current_ram_bank: u8,
    pub ram_enabled: bool,
}

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