use crate::bus::Bus;
use crate::cpu::Cpu;
use crate::savestate::{
    ApuState, BusState, CpuState, Mbc1State, NoiseChannelState,
    SaveState, SquareChannelState, WaveChannelState,
};
#[derive(Clone, Copy)]
pub struct DiffOptions {
    pub max_vram_lines: usize,
    pub max_wram_lines: usize,
    pub max_oam_lines: usize,
    pub show_opcode_counts: bool,
    pub show_debug_counters: bool,
}

impl Default for DiffOptions {
    fn default() -> Self {
        Self {
            max_vram_lines: 32,
            max_wram_lines: 32,
            max_oam_lines: 32,
            show_opcode_counts: false,
            show_debug_counters: false,
        }
    }
}
#[derive(Clone)]
pub struct DebugSnapshot {
    pub state: SaveState,
    pub opcode_counts: [u64; 256],
    pub debug_frames: u64,
}

impl DebugSnapshot {
    pub fn capture(cpu: &Cpu, bus: &Bus) -> Self {
        Self {
            state: SaveState::capture(cpu, bus),
            opcode_counts: cpu.opcode_counts,
            debug_frames: bus.debug_frames,
        }
    }
    pub fn compare_with(&self, cpu: &Cpu, bus: &Bus) -> Vec<String> {
        let current = Self::capture(cpu, bus);
        self.diff(&current)
    }

    pub fn print_diff(&self, cpu: &Cpu, bus: &Bus) {
        let differences = self.compare_with(cpu, bus);

        if differences.is_empty() {
            println!("DEBUG SNAPSHOT: stany są identyczne.");
            return;
        }

        println!(
            "DEBUG SNAPSHOT: wykryto {} różnic:",
            differences.len()
        );

        for difference in differences {
            println!("  {}", difference);
        }
    }

    pub fn diff(&self, other: &Self) -> Vec<String> {
        let mut differences = Vec::new();

        diff_cpu(&self.state.cpu, &other.state.cpu, &mut differences);
        diff_bus(&self.state.bus, &other.state.bus, &mut differences);

        for i in 0..256 {
            if self.opcode_counts[i] != other.opcode_counts[i] {
                differences.push(format!(
                    "CPU.opcode_counts[{:02X}]: {} != {}",
                    i, self.opcode_counts[i], other.opcode_counts[i]
                ));
            }
        }

        if self.debug_frames != other.debug_frames {
            differences.push(format!(
                "Bus.debug_frames: {} != {}",
                self.debug_frames, other.debug_frames
            ));
        }

        differences
    }

    pub fn is_identical(&self, other: &Self) -> bool {
        self.diff(other).is_empty()
    }
}

fn diff_cpu(a: &CpuState, b: &CpuState, out: &mut Vec<String>) {
    macro_rules! check {
        ($field:ident) => {
            if a.$field != b.$field {
                out.push(format!(
                    "CPU.{}: {:?} != {:?}",
                    stringify!($field), a.$field, b.$field
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
}

fn diff_bus(a: &BusState, b: &BusState, out: &mut Vec<String>) {
    diff_bytes("Bus.vram", &a.vram, &b.vram, out);
    diff_bytes("Bus.wram", &a.wram, &b.wram, out);
    diff_bytes("Bus.oam", &a.oam, &b.oam, out);
    diff_bytes("Bus.hram", &a.hram, &b.hram, out);
    diff_bytes("Bus.io", &a.io, &b.io, out);

    macro_rules! check {
        ($field:ident) => {
            if a.$field != b.$field {
                out.push(format!(
                    "Bus.{}: {:?} != {:?}",
                    stringify!($field), a.$field, b.$field
                ));
            }
        };
    }

    check!(joyp);
    check!(buttons);
    check!(ie);
    check!(if_reg);

    check!(ly);
    check!(lyc);
    check!(ppu_mode);
    check!(lcd_cycles);
    check!(lcdc);
    check!(stat);
    check!(scy);
    check!(scx);
    check!(bgp);
    check!(obp0);
    check!(obp1);
    check!(wy);
    check!(wx);
    check!(dma);

    check!(div);
    check!(tima);
    check!(tma);
    check!(tac);
    check!(div_cycles);
    check!(tima_cycles);

    diff_mbc1(&a.mbc1, &b.mbc1, out);
    diff_apu(&a.apu, &b.apu, out);
}

fn diff_mbc1(a: &Mbc1State, b: &Mbc1State, out: &mut Vec<String>) {
    macro_rules! check {
        ($field:ident) => {
            if a.$field != b.$field {
                out.push(format!(
                    "MBC1.{}: {:?} != {:?}",
                    stringify!($field), a.$field, b.$field
                ));
            }
        };
    }

    check!(rom_bank_low);
    check!(rom_bank_high);
    check!(banking_mode);
    check!(ram_enabled);
    diff_bytes("MBC1.ram", &a.ram, &b.ram, out);
}

fn diff_apu(a: &ApuState, b: &ApuState, out: &mut Vec<String>) {
    macro_rules! check {
        ($field:ident) => {
            if a.$field != b.$field {
                out.push(format!(
                    "APU.{}: {:?} != {:?}",
                    stringify!($field), a.$field, b.$field
                ));
            }
        };
    }

    check!(enabled);
    check!(frame_sequencer_cycles);
    check!(frame_sequencer_step);
    check!(sample_cycles);
    check!(nr50);
    check!(nr51);
    check!(nr52);
    check!(sample_counter);

    diff_square("APU.CH1", &a.ch1, &b.ch1, out);
    diff_square("APU.CH2", &a.ch2, &b.ch2, out);
    diff_wave("APU.CH3", &a.ch3, &b.ch3, out);
    diff_noise("APU.CH4", &a.ch4, &b.ch4, out);
}

fn diff_square(name: &str, a: &SquareChannelState, b: &SquareChannelState, out: &mut Vec<String>) {
    macro_rules! check {
        ($field:ident) => {
            if a.$field != b.$field {
                out.push(format!(
                    "{}.{}: {:?} != {:?}",
                    name, stringify!($field), a.$field, b.$field
                ));
            }
        };
    }

    check!(enabled);
    check!(nr0);
    check!(nr1);
    check!(nr2);
    check!(nr3);
    check!(nr4);
    check!(length_counter);
    check!(length_enabled);
    check!(envelope_volume);
    check!(envelope_timer);
    check!(frequency_timer);
    check!(duty_position);
    check!(sweep_timer);
    check!(sweep_shadow_frequency);
    check!(sweep_enabled);
    check!(has_sweep);
}

fn diff_wave(name: &str, a: &WaveChannelState, b: &WaveChannelState, out: &mut Vec<String>) {
    macro_rules! check {
        ($field:ident) => {
            if a.$field != b.$field {
                out.push(format!(
                    "{}.{}: {:?} != {:?}",
                    name, stringify!($field), a.$field, b.$field
                ));
            }
        };
    }

    check!(enabled);
    check!(nr30);
    check!(nr31);
    check!(nr32);
    check!(nr33);
    check!(nr34);
    check!(length_counter);
    check!(length_enabled);
    check!(frequency_timer);
    check!(position);
    check!(wave_ram);
}

fn diff_noise(name: &str, a: &NoiseChannelState, b: &NoiseChannelState, out: &mut Vec<String>) {
    macro_rules! check {
        ($field:ident) => {
            if a.$field != b.$field {
                out.push(format!(
                    "{}.{}: {:?} != {:?}",
                    name, stringify!($field), a.$field, b.$field
                ));
            }
        };
    }

    check!(enabled);
    check!(nr41);
    check!(nr42);
    check!(nr43);
    check!(nr44);
    check!(length_counter);
    check!(length_enabled);
    check!(envelope_volume);
    check!(envelope_timer);
    check!(frequency_timer);
    check!(lfsr);
}

fn diff_bytes(name: &str, a: &[u8], b: &[u8], out: &mut Vec<String>) {
    if a == b {
        return;
    }

    let max_len = a.len().max(b.len());
    for i in 0..max_len {
        let av = a.get(i).copied();
        let bv = b.get(i).copied();
        if av != bv {
            out.push(format!(
                "{}[0x{:04X}]: {:?} != {:?}",
                name, i, av, bv
            ));
        }
    }
}
