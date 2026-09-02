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
                halted: cpu.halted,
            },

            bus: BusState {
                vram: bus.vram,
                wram: bus.wram,
                eram: bus.eram,
                oam: bus.oam,
                hram: bus.hram,
                io: bus.io,

                ie: bus.ie,
                if_reg: bus.if_reg,

                lcdc: bus.lcdc,
                stat: bus.stat,
                ly: bus.ly,
                lyc: bus.lyc,
                scx: bus.scx,
                scy: bus.scy,

                bgp: bus.bgp,
                obp0: bus.obp0,
                obp1: bus.obp1,

                lcd_cycles: bus.lcd_cycles,
                timer_cycles: bus.timer_cycles,

                current_rom_bank: bus.current_rom_bank,
                current_ram_bank: bus.current_ram_bank,
                ram_enabled: bus.ram_enabled,
            },
        }
    }
    pub fn restore(self, cpu: &mut Cpu, bus: &mut Bus) {
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
        cpu.halted = self.cpu.halted;

        bus.vram = self.bus.vram;
        bus.wram = self.bus.wram;
        bus.eram = self.bus.eram;
        bus.oam = self.bus.oam;
        bus.hram = self.bus.hram;
        bus.io = self.bus.io;

        bus.ie = self.bus.ie;
        bus.if_reg = self.bus.if_reg;

        bus.lcdc = self.bus.lcdc;
        bus.stat = self.bus.stat;
        bus.ly = self.bus.ly;
        bus.lyc = self.bus.lyc;
        bus.scx = self.bus.scx;
        bus.scy = self.bus.scy;

        bus.bgp = self.bus.bgp;
        bus.obp0 = self.bus.obp0;
        bus.obp1 = self.bus.obp1;

        bus.lcd_cycles = self.bus.lcd_cycles;
        bus.timer_cycles = self.bus.timer_cycles;

        bus.current_rom_bank = self.bus.current_rom_bank;
        bus.current_ram_bank = self.bus.current_ram_bank;
        bus.ram_enabled = self.bus.ram_enabled;
    }
pub fn save_to_file(state: &SaveState, path: &str) {
    let bytes = bincode::serde::encode_to_vec(
        state,
        bincode::config::standard()
    ).unwrap();

    let mut file = File::create(path).unwrap();
    file.write_all(&bytes).unwrap();
}
pub fn load_from_file(path: &str) -> SaveState {
    let bytes = fs::read(path).unwrap();

    let (state, _) = bincode::serde::decode_from_slice(
        &bytes,
        bincode::config::standard()
    ).unwrap();

    state
}
}