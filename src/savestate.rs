use std::fs::{self, File};
use std::io::Write;

use serde::{Deserialize, Serialize};

use crate::{
    bus::Bus,
    cpu::Cpu,
};

#[derive(Clone, Serialize, Deserialize)]
pub struct SaveState {
    pub cpu: CpuState,
    pub bus: BusState,
}

// ============================================================
// CPU
// ============================================================

#[derive(Clone, Serialize, Deserialize)]
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
    pub ime_pending: bool,
    pub halted: bool,
}

// ============================================================
// BUS
// ============================================================

#[derive(Clone, Serialize, Deserialize)]
pub struct BusState {
    // Memory
    pub vram: Vec<u8>,
    pub wram: Vec<u8>,
    pub oam: Vec<u8>,
    pub hram: Vec<u8>,
    pub io: Vec<u8>,

    // Joypad
    pub joyp: u8,
    pub buttons: u8,

    // Interrupts
    pub ie: u8,
    pub if_reg: u8,

    // ========================================================
    // PPU / LCD
    // ========================================================

    pub ly: u8,
    pub lyc: u8,
    pub ppu_mode: u8,
    pub lcd_cycles: u32,

    pub lcdc: u8,
    pub stat: u8,

    pub scy: u8,
    pub scx: u8,

    pub bgp: u8,
    pub obp0: u8,
    pub obp1: u8,

    pub wy: u8,
    pub wx: u8,

    pub dma: u8,

    // ========================================================
    // TIMER
    // ========================================================

    pub div: u8,
    pub tima: u8,
    pub tma: u8,
    pub tac: u8,

    pub div_cycles: u32,
    pub tima_cycles: u32,

    // ========================================================
    // CARTRIDGE / MBC1
    // ========================================================

    pub mbc1: Mbc1State,

    // ========================================================
    // APU
    // ========================================================

    pub apu: ApuState,
}

// ============================================================
// MBC1
// ============================================================

#[derive(Clone, Serialize, Deserialize)]
pub struct Mbc1State {
    pub rom_bank_low: u8,
    pub rom_bank_high: u8,
    pub banking_mode: u8,

    pub ram_enabled: bool,
    pub ram: Vec<u8>,
}

// ============================================================
// APU
// ============================================================

#[derive(Clone, Serialize, Deserialize)]
pub struct ApuState {
    pub enabled: bool,

    pub frame_sequencer_cycles: u32,
    pub frame_sequencer_step: u8,
    pub sample_cycles: u64,

    pub ch1: SquareChannelState,
    pub ch2: SquareChannelState,
    pub ch3: WaveChannelState,
    pub ch4: NoiseChannelState,

    pub nr50: u8,
    pub nr51: u8,
    pub nr52: u8,

    pub sample_counter: u64,
}

// ============================================================
// CHANNEL 1 / CHANNEL 2
// ============================================================

#[derive(Clone, Serialize, Deserialize)]
pub struct SquareChannelState {
    pub enabled: bool,

    pub nr0: u8,
    pub nr1: u8,
    pub nr2: u8,
    pub nr3: u8,
    pub nr4: u8,

    pub length_counter: u8,
    pub length_enabled: bool,

    pub envelope_volume: u8,
    pub envelope_timer: u8,

    pub frequency_timer: u16,
    pub duty_position: u8,

    pub sweep_timer: u8,
    pub sweep_shadow_frequency: u16,
    pub sweep_enabled: bool,

    pub has_sweep: bool,
}

// ============================================================
// CHANNEL 3
// ============================================================

#[derive(Clone, Serialize, Deserialize)]
pub struct WaveChannelState {
    pub enabled: bool,

    pub nr30: u8,
    pub nr31: u8,
    pub nr32: u8,
    pub nr33: u8,
    pub nr34: u8,

    pub length_counter: u16,
    pub length_enabled: bool,

    pub frequency_timer: u16,
    pub position: u8,

    pub wave_ram: [u8; 16],
}

// ============================================================
// CHANNEL 4
// ============================================================

#[derive(Clone, Serialize, Deserialize)]
pub struct NoiseChannelState {
    pub enabled: bool,

    pub nr41: u8,
    pub nr42: u8,
    pub nr43: u8,
    pub nr44: u8,

    pub length_counter: u8,
    pub length_enabled: bool,

    pub envelope_volume: u8,
    pub envelope_timer: u8,

    pub frequency_timer: u16,

    pub lfsr: u16,
}

// ============================================================
// CAPTURE
// ============================================================

impl SaveState {
    pub fn capture(cpu: &Cpu, bus: &Bus) -> Self {
        Self {
            cpu: CpuState {
                a: cpu.a,
                f: cpu.f,

                b: cpu.b,
                c: cpu.c,

                d: cpu.d,
                e: cpu.e,

                h: cpu.h,
                l: cpu.l,

                pc: cpu.pc,
                sp: cpu.sp,

                ime: cpu.ime,
                ime_pending: cpu.ime_pending,
                halted: cpu.halted,
            },

            bus: BusState {
                vram: bus.vram.to_vec(),
                wram: bus.wram.to_vec(),
                oam: bus.oam.to_vec(),
                hram: bus.hram.to_vec(),
                io: bus.io.to_vec(),

                joyp: bus.joyp,
                buttons: bus.buttons,

                ie: bus.ie,
                if_reg: bus.if_reg,

                // PPU
                ly: bus.ly,
                lyc: bus.lyc,
                ppu_mode: bus.ppu_mode,
                lcd_cycles: bus.lcd_cycles,

                lcdc: bus.lcdc,
                stat: bus.stat,

                scy: bus.scy,
                scx: bus.scx,

                bgp: bus.bgp,
                obp0: bus.obp0,
                obp1: bus.obp1,

                wy: bus.wy,
                wx: bus.wx,

                dma: bus.dma,

                // Timer
                div: bus.div,
                tima: bus.tima,
                tma: bus.tma,
                tac: bus.tac,

                div_cycles: bus.div_cycles,
                tima_cycles: bus.tima_cycles,

                // MBC1
                mbc1: Mbc1State {
                    rom_bank_low: bus.mbc1.rom_bank_low,
                    rom_bank_high: bus.mbc1.rom_bank_high,
                    banking_mode: bus.mbc1.banking_mode,

                    ram_enabled: bus.mbc1.ram_enabled,
                    ram: bus.mbc1.ram.clone(),
                },

                // APU
                apu: ApuState {
                    enabled: bus.apu.enabled,

                    frame_sequencer_cycles:
                        bus.apu.frame_sequencer_cycles,

                    frame_sequencer_step:
                        bus.apu.frame_sequencer_step,

                    sample_cycles:
                        bus.apu.sample_cycles,

                    ch1: SquareChannelState {
                        enabled: bus.apu.ch1.enabled,

                        nr0: bus.apu.ch1.nr0,
                        nr1: bus.apu.ch1.nr1,
                        nr2: bus.apu.ch1.nr2,
                        nr3: bus.apu.ch1.nr3,
                        nr4: bus.apu.ch1.nr4,

                        length_counter:
                            bus.apu.ch1.length_counter,

                        length_enabled:
                            bus.apu.ch1.length_enabled,

                        envelope_volume:
                            bus.apu.ch1.envelope_volume,

                        envelope_timer:
                            bus.apu.ch1.envelope_timer,

                        frequency_timer:
                            bus.apu.ch1.frequency_timer,

                        duty_position:
                            bus.apu.ch1.duty_position,

                        sweep_timer:
                            bus.apu.ch1.sweep_timer,

                        sweep_shadow_frequency:
                            bus.apu.ch1.sweep_shadow_frequency,

                        sweep_enabled:
                            bus.apu.ch1.sweep_enabled,

                        has_sweep:
                            bus.apu.ch1.has_sweep,
                    },

                    ch2: SquareChannelState {
                        enabled: bus.apu.ch2.enabled,

                        nr0: bus.apu.ch2.nr0,
                        nr1: bus.apu.ch2.nr1,
                        nr2: bus.apu.ch2.nr2,
                        nr3: bus.apu.ch2.nr3,
                        nr4: bus.apu.ch2.nr4,

                        length_counter:
                            bus.apu.ch2.length_counter,

                        length_enabled:
                            bus.apu.ch2.length_enabled,

                        envelope_volume:
                            bus.apu.ch2.envelope_volume,

                        envelope_timer:
                            bus.apu.ch2.envelope_timer,

                        frequency_timer:
                            bus.apu.ch2.frequency_timer,

                        duty_position:
                            bus.apu.ch2.duty_position,

                        sweep_timer:
                            bus.apu.ch2.sweep_timer,

                        sweep_shadow_frequency:
                            bus.apu.ch2.sweep_shadow_frequency,

                        sweep_enabled:
                            bus.apu.ch2.sweep_enabled,

                        has_sweep:
                            bus.apu.ch2.has_sweep,
                    },

                    ch3: WaveChannelState {
                        enabled: bus.apu.ch3.enabled,

                        nr30: bus.apu.ch3.nr30,
                        nr31: bus.apu.ch3.nr31,
                        nr32: bus.apu.ch3.nr32,
                        nr33: bus.apu.ch3.nr33,
                        nr34: bus.apu.ch3.nr34,

                        length_counter:
                            bus.apu.ch3.length_counter,

                        length_enabled:
                            bus.apu.ch3.length_enabled,

                        frequency_timer:
                            bus.apu.ch3.frequency_timer,

                        position:
                            bus.apu.ch3.position,

                        wave_ram:
                            bus.apu.ch3.wave_ram,
                    },

                    ch4: NoiseChannelState {
                        enabled: bus.apu.ch4.enabled,

                        nr41: bus.apu.ch4.nr41,
                        nr42: bus.apu.ch4.nr42,
                        nr43: bus.apu.ch4.nr43,
                        nr44: bus.apu.ch4.nr44,

                        length_counter:
                            bus.apu.ch4.length_counter,

                        length_enabled:
                            bus.apu.ch4.length_enabled,

                        envelope_volume:
                            bus.apu.ch4.envelope_volume,

                        envelope_timer:
                            bus.apu.ch4.envelope_timer,

                        frequency_timer:
                            bus.apu.ch4.frequency_timer,

                        lfsr:
                            bus.apu.ch4.lfsr,
                    },

                    nr50: bus.apu.nr50,
                    nr51: bus.apu.nr51,
                    nr52: bus.apu.nr52,

                    sample_counter:
                        bus.apu.sample_counter,
                },
            },
        }
    }

    // ========================================================
    // RESTORE
    // ========================================================

    pub fn restore(self, cpu: &mut Cpu, bus: &mut Bus) {
        // ----------------------------------------------------
        // CPU
        // ----------------------------------------------------

        cpu.a = self.cpu.a;
        cpu.f = self.cpu.f;

        cpu.b = self.cpu.b;
        cpu.c = self.cpu.c;

        cpu.d = self.cpu.d;
        cpu.e = self.cpu.e;

        cpu.h = self.cpu.h;
        cpu.l = self.cpu.l;

        cpu.pc = self.cpu.pc;
        cpu.sp = self.cpu.sp;

        cpu.ime = self.cpu.ime;
        cpu.ime_pending = self.cpu.ime_pending;
        cpu.halted = self.cpu.halted;

        // ----------------------------------------------------
        // MEMORY
        // ----------------------------------------------------

        bus.vram.copy_from_slice(&self.bus.vram);
        bus.wram.copy_from_slice(&self.bus.wram);
        bus.oam.copy_from_slice(&self.bus.oam);
        bus.hram.copy_from_slice(&self.bus.hram);
        bus.io.copy_from_slice(&self.bus.io);

        // ----------------------------------------------------
        // JOYPAD
        // ----------------------------------------------------

        bus.joyp = self.bus.joyp;
        bus.buttons = self.bus.buttons;

        // ----------------------------------------------------
        // INTERRUPTS
        // ----------------------------------------------------

        bus.ie = self.bus.ie;
        bus.if_reg = self.bus.if_reg;

        // ----------------------------------------------------
        // PPU
        // ----------------------------------------------------

        bus.ly = self.bus.ly;
        bus.lyc = self.bus.lyc;
        bus.ppu_mode = self.bus.ppu_mode;
        bus.lcd_cycles = self.bus.lcd_cycles;

        bus.lcdc = self.bus.lcdc;
        bus.stat = self.bus.stat;

        bus.scy = self.bus.scy;
        bus.scx = self.bus.scx;

        bus.bgp = self.bus.bgp;
        bus.obp0 = self.bus.obp0;
        bus.obp1 = self.bus.obp1;

        bus.wy = self.bus.wy;
        bus.wx = self.bus.wx;

        bus.dma = self.bus.dma;

        // ----------------------------------------------------
        // TIMER
        // ----------------------------------------------------

        bus.div = self.bus.div;
        bus.tima = self.bus.tima;
        bus.tma = self.bus.tma;
        bus.tac = self.bus.tac;

        bus.div_cycles = self.bus.div_cycles;
        bus.tima_cycles = self.bus.tima_cycles;

        // ----------------------------------------------------
        // MBC1
        // ----------------------------------------------------

        bus.mbc1.rom_bank_low =
            self.bus.mbc1.rom_bank_low;

        bus.mbc1.rom_bank_high =
            self.bus.mbc1.rom_bank_high;

        bus.mbc1.banking_mode =
            self.bus.mbc1.banking_mode;

        bus.mbc1.ram_enabled =
            self.bus.mbc1.ram_enabled;

        bus.mbc1.ram =
            self.bus.mbc1.ram;

        // ----------------------------------------------------
        // APU
        // ----------------------------------------------------

        bus.apu.enabled =
            self.bus.apu.enabled;

        bus.apu.frame_sequencer_cycles =
            self.bus.apu.frame_sequencer_cycles;

        bus.apu.frame_sequencer_step =
            self.bus.apu.frame_sequencer_step;

        bus.apu.sample_cycles =
            self.bus.apu.sample_cycles;

        // ----------------------------------------------------
        // CH1
        // ----------------------------------------------------

        bus.apu.ch1.enabled =
            self.bus.apu.ch1.enabled;

        bus.apu.ch1.nr0 =
            self.bus.apu.ch1.nr0;

        bus.apu.ch1.nr1 =
            self.bus.apu.ch1.nr1;

        bus.apu.ch1.nr2 =
            self.bus.apu.ch1.nr2;

        bus.apu.ch1.nr3 =
            self.bus.apu.ch1.nr3;

        bus.apu.ch1.nr4 =
            self.bus.apu.ch1.nr4;

        bus.apu.ch1.length_counter =
            self.bus.apu.ch1.length_counter;

        bus.apu.ch1.length_enabled =
            self.bus.apu.ch1.length_enabled;

        bus.apu.ch1.envelope_volume =
            self.bus.apu.ch1.envelope_volume;

        bus.apu.ch1.envelope_timer =
            self.bus.apu.ch1.envelope_timer;

        bus.apu.ch1.frequency_timer =
            self.bus.apu.ch1.frequency_timer;

        bus.apu.ch1.duty_position =
            self.bus.apu.ch1.duty_position;

        bus.apu.ch1.sweep_timer =
            self.bus.apu.ch1.sweep_timer;

        bus.apu.ch1.sweep_shadow_frequency =
            self.bus.apu.ch1.sweep_shadow_frequency;

        bus.apu.ch1.sweep_enabled =
            self.bus.apu.ch1.sweep_enabled;

        bus.apu.ch1.has_sweep =
            self.bus.apu.ch1.has_sweep;

        // ----------------------------------------------------
        // CH2
        // ----------------------------------------------------

        bus.apu.ch2.enabled =
            self.bus.apu.ch2.enabled;

        bus.apu.ch2.nr0 =
            self.bus.apu.ch2.nr0;

        bus.apu.ch2.nr1 =
            self.bus.apu.ch2.nr1;

        bus.apu.ch2.nr2 =
            self.bus.apu.ch2.nr2;

        bus.apu.ch2.nr3 =
            self.bus.apu.ch2.nr3;

        bus.apu.ch2.nr4 =
            self.bus.apu.ch2.nr4;

        bus.apu.ch2.length_counter =
            self.bus.apu.ch2.length_counter;

        bus.apu.ch2.length_enabled =
            self.bus.apu.ch2.length_enabled;

        bus.apu.ch2.envelope_volume =
            self.bus.apu.ch2.envelope_volume;

        bus.apu.ch2.envelope_timer =
            self.bus.apu.ch2.envelope_timer;

        bus.apu.ch2.frequency_timer =
            self.bus.apu.ch2.frequency_timer;

        bus.apu.ch2.duty_position =
            self.bus.apu.ch2.duty_position;

        bus.apu.ch2.sweep_timer =
            self.bus.apu.ch2.sweep_timer;

        bus.apu.ch2.sweep_shadow_frequency =
            self.bus.apu.ch2.sweep_shadow_frequency;

        bus.apu.ch2.sweep_enabled =
            self.bus.apu.ch2.sweep_enabled;

        bus.apu.ch2.has_sweep =
            self.bus.apu.ch2.has_sweep;

        // ----------------------------------------------------
        // CH3
        // ----------------------------------------------------

        bus.apu.ch3.enabled =
            self.bus.apu.ch3.enabled;

        bus.apu.ch3.nr30 =
            self.bus.apu.ch3.nr30;

        bus.apu.ch3.nr31 =
            self.bus.apu.ch3.nr31;

        bus.apu.ch3.nr32 =
            self.bus.apu.ch3.nr32;

        bus.apu.ch3.nr33 =
            self.bus.apu.ch3.nr33;

        bus.apu.ch3.nr34 =
            self.bus.apu.ch3.nr34;

        bus.apu.ch3.length_counter =
            self.bus.apu.ch3.length_counter;

        bus.apu.ch3.length_enabled =
            self.bus.apu.ch3.length_enabled;

        bus.apu.ch3.frequency_timer =
            self.bus.apu.ch3.frequency_timer;

        bus.apu.ch3.position =
            self.bus.apu.ch3.position;

        bus.apu.ch3.wave_ram =
            self.bus.apu.ch3.wave_ram;

        // ----------------------------------------------------
        // CH4
        // ----------------------------------------------------

        bus.apu.ch4.enabled =
            self.bus.apu.ch4.enabled;

        bus.apu.ch4.nr41 =
            self.bus.apu.ch4.nr41;

        bus.apu.ch4.nr42 =
            self.bus.apu.ch4.nr42;

        bus.apu.ch4.nr43 =
            self.bus.apu.ch4.nr43;

        bus.apu.ch4.nr44 =
            self.bus.apu.ch4.nr44;

        bus.apu.ch4.length_counter =
            self.bus.apu.ch4.length_counter;

        bus.apu.ch4.length_enabled =
            self.bus.apu.ch4.length_enabled;

        bus.apu.ch4.envelope_volume =
            self.bus.apu.ch4.envelope_volume;

        bus.apu.ch4.envelope_timer =
            self.bus.apu.ch4.envelope_timer;

        bus.apu.ch4.frequency_timer =
            self.bus.apu.ch4.frequency_timer;

        bus.apu.ch4.lfsr =
            self.bus.apu.ch4.lfsr;

        // ----------------------------------------------------
        // APU MIXER
        // ----------------------------------------------------

        bus.apu.nr50 =
            self.bus.apu.nr50;

        bus.apu.nr51 =
            self.bus.apu.nr51;

        bus.apu.nr52 =
            self.bus.apu.nr52;

        bus.apu.sample_counter =
            self.bus.apu.sample_counter;
    }
}

// ============================================================
// FILE SAVE
// ============================================================

pub fn save_to_file(state: &SaveState, path: &str) {
    let bytes = bincode::serialize(state).unwrap();

    let mut file = File::create(path).unwrap();
    file.write_all(&bytes).unwrap();
}

// ============================================================
// FILE LOAD
// ============================================================

pub fn load_from_file(path: &str) -> SaveState {
    let bytes = fs::read(path).unwrap();

    bincode::deserialize(&bytes).unwrap()
}

pub fn compare_cpu(cpu1: &Cpu, cpu2: &Cpu) -> Result<(), String> {
    macro_rules! check {
        ($field:ident) => {
            if cpu1.$field != cpu2.$field {
                return Err(format!(
                    "CPU {}: {:?} != {:?}",
                    stringify!($field),
                    cpu1.$field,
                    cpu2.$field
                ));
            }
        };
    }

    check!(a);
    check!(f);
    check!(b);
    check!(c);
    check!(d);
    check!(e);
    check!(h);
    check!(l);

    check!(pc);
    check!(sp);

    check!(ime);
    check!(ime_pending);
    check!(halted);

    Ok(())
}

pub fn compare_bus(bus1: &Bus, bus2: &Bus) -> Result<(), String> {
    macro_rules! check {
        ($field:ident) => {
            if bus1.$field != bus2.$field {
                return Err(format!(
                    "BUS {} różni się",
                    stringify!($field)
                ));
            }
        };
    }

    check!(joyp);
    check!(buttons);

    check!(ly);
    check!(lyc);
    check!(ppu_mode);
    check!(lcd_cycles);

    check!(lcdc);
    check!(stat);

    check!(scx);
    check!(scy);

    check!(bgp);
    check!(obp0);
    check!(obp1);

    check!(wx);
    check!(wy);

    check!(dma);

    check!(div);
    check!(tima);
    check!(tma);
    check!(tac);

    check!(div_cycles);
    check!(tima_cycles);

    check!(ie);
    check!(if_reg);

    if bus1.vram != bus2.vram {
        return Err("VRAM różni się".into());
    }

    if bus1.wram != bus2.wram {
        return Err("WRAM różni się".into());
    }

    if bus1.oam != bus2.oam {
        return Err("OAM różni się".into());
    }

    if bus1.hram != bus2.hram {
        return Err("HRAM różni się".into());
    }

    if bus1.io != bus2.io {
        return Err("IO różni się".into());
    }

    Ok(())
}