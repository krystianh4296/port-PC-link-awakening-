use std::collections::{HashMap, HashSet, VecDeque};

use crate::{bus::Bus, cpu::Cpu};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DebugAction {
    Run,
    Step,
    Continue,
    Break,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WatchType {
    Change,
    Read,
    Write,
}

#[derive(Clone, Debug)]
pub struct MemoryWatch {
    pub address: u16,
    pub kind: WatchType,
    pub last_value: u8,
}

#[derive(Clone, Debug)]
pub struct TraceEntry {
    pub address: u16,
    pub bytes: Vec<u8>,
    pub text: String,
}

#[derive(Clone, Debug)]
pub struct MemoryChange {
    pub frame: u64,
    pub address: u16,
    pub old_value: u8,
    pub new_value: u8,
}

#[derive(Clone, Debug)]
pub struct DisassembledInstruction {
    pub address: u16,
    pub bytes: Vec<u8>,
    pub text: String,
    pub length: u8,
}

pub struct Debugger {
    pub enabled: bool,
    pub action: DebugAction,
    pub breakpoints: HashSet<u16>,
    pub watches: HashMap<u16, MemoryWatch>,
    pub trace_enabled: bool,
    pub trace_start: u16,
    pub trace_end: u16,
    pub history: VecDeque<TraceEntry>,
    pub history_limit: usize,
    pub frame: u64,
    pub stop_reason: Option<String>,
    pub last_pc: Option<u16>,
}

impl Debugger {
    pub fn new() -> Self {
        Self {
            enabled: false,
            action: DebugAction::Run,
            breakpoints: HashSet::new(),
            watches: HashMap::new(),
            trace_enabled: false,
            trace_start: 0,
            trace_end: 0,
            history: VecDeque::with_capacity(256),
            history_limit: 256,
            frame: 0,
            stop_reason: None,
            last_pc: None,
        }
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
        self.action = DebugAction::Run;
        self.stop_reason = None;
    }

    pub fn break_now(&mut self, reason: impl Into<String>) {
        self.enabled = true;
        self.action = DebugAction::Break;
        self.stop_reason = Some(reason.into());
    }

    pub fn step(&mut self) {
        self.enabled = true;
        self.action = DebugAction::Step;
        self.stop_reason = None;
    }

    pub fn continue_execution(&mut self) {
        self.enabled = true;
        self.action = DebugAction::Continue;
        self.stop_reason = None;
    }

    pub fn add_breakpoint(&mut self, address: u16) {
        self.breakpoints.insert(address);
    }

    pub fn remove_breakpoint(&mut self, address: u16) {
        self.breakpoints.remove(&address);
    }

    pub fn clear_breakpoints(&mut self) {
        self.breakpoints.clear();
    }

    pub fn has_breakpoint(&self, address: u16) -> bool {
        self.breakpoints.contains(&address)
    }

    pub fn set_trace_range(&mut self, start: u16, end: u16) {
        self.trace_start = start;
        self.trace_end = end;
        self.trace_enabled = true;
    }

    pub fn disable_trace(&mut self) {
        self.trace_enabled = false;
    }

    pub fn watch(&mut self, bus: &mut Bus, address: u16) {
        let value = bus.read(address);
        self.watches.insert(
            address,
            MemoryWatch {
                address,
                kind: WatchType::Change,
                last_value: value,
            },
        );
    }

    pub fn watch_kind(&mut self, bus: &mut Bus, address: u16, kind: WatchType) {
        let value = bus.read(address);
        self.watches.insert(
            address,
            MemoryWatch {
                address,
                kind,
                last_value: value,
            },
        );
    }

    pub fn unwatch(&mut self, address: u16) {
        self.watches.remove(&address);
    }

    pub fn before_instruction(&mut self, cpu: &Cpu, bus: &mut Bus) -> bool {
        if !self.enabled {
            return true;
        }

        if self.action == DebugAction::Break {
            return false;
        }

        if self.breakpoints.contains(&cpu.pc) {
            self.action = DebugAction::Break;
            self.stop_reason = Some(format!("Breakpoint hit at {:04X}", cpu.pc));
            self.print_status(cpu);
            return false;
        }

        if self.trace_enabled && Self::in_trace_range(cpu.pc, self.trace_start, self.trace_end) {
            let instruction = disassemble_at(bus, cpu.pc);
            self.trace_instruction(&instruction);
        }

        // Keep watch values synchronized. Exact read/write watchpoints require
        // instrumentation in Bus::read/Bus::write; change watches are checked here.
        for watch in self.watches.values_mut() {
            let value = bus.read(watch.address);
            if watch.kind == WatchType::Change && value != watch.last_value {
                println!(
                    "WATCH ${:04X}: {:02X} -> {:02X} (frame {})",
                    watch.address, watch.last_value, value, self.frame
                );
                watch.last_value = value;
            }
        }

        true
    }

    pub fn after_instruction_hook(&mut self) {
        if self.action == DebugAction::Step {
            self.action = DebugAction::Break;
            self.stop_reason = Some("Single step complete".to_string());
        }
    }

    pub fn next_frame(&mut self, bus: &mut Bus) {
        self.frame = self.frame.wrapping_add(1);

        for watch in self.watches.values_mut() {
            let value = bus.read(watch.address);
            if watch.kind == WatchType::Change && value != watch.last_value {
                println!(
                    "frame {}: ${:04X}: {:02X} -> {:02X}",
                    self.frame, watch.address, watch.last_value, value
                );
                watch.last_value = value;
            }
        }
    }

    pub fn print_status(&self, cpu: &Cpu) {
        println!(
            "CPU: PC={:04X} AF={:04X} BC={:04X} DE={:04X} HL={:04X} SP={:04X} IME={} HALT={}",
            cpu.pc,
            cpu.af(),
            cpu.bc(),
            cpu.de(),
            cpu.hl(),
            cpu.sp,
            cpu.ime,
            cpu.halted
        );
        println!(
            "FLAGS: Z={} N={} H={} C={}",
            cpu.f & 0x80 != 0,
            cpu.f & 0x40 != 0,
            cpu.f & 0x20 != 0,
            cpu.f & 0x10 != 0
        );

        if let Some(reason) = &self.stop_reason {
            println!("DEBUG: {}", reason);
        }
    }

    pub fn dump_memory(&self, bus: &mut Bus, address: u16, length: usize) {
        for offset in (0..length).step_by(16) {
            let mut line = format!("{:04X}: ", address.wrapping_add(offset as u16));
            for i in 0..16 {
                if offset + i >= length {
                    break;
                }
                let addr = address.wrapping_add((offset + i) as u16);
                line.push_str(&format!("{:02X} ", bus.read(addr)));
            }
            println!("{}", line);
        }
    }

    pub fn print_history(&self) {
        for entry in &self.history {
            println!(
                "{:04X}: {:<11} {}",
                entry.address,
                format_bytes(&entry.bytes),
                entry.text
            );
        }
    }

    pub fn trace_instruction(&mut self, instruction: &DisassembledInstruction) {
        if self.history.len() >= self.history_limit {
            self.history.pop_front();
        }

        let entry = TraceEntry {
            address: instruction.address,
            bytes: instruction.bytes.clone(),
            text: instruction.text.clone(),
        };

        println!(
            "{:04X}: {:<11} {}",
            entry.address,
            format_bytes(&entry.bytes),
            entry.text
        );

        self.history.push_back(entry);
    }

    fn in_trace_range(address: u16, start: u16, end: u16) -> bool {
        if start <= end {
            address >= start && address <= end
        } else {
            address >= start || address <= end
        }
    }
}

pub fn disassemble_at(bus: &mut Bus, address: u16) -> DisassembledInstruction {
    let opcode = bus.read(address);

    if opcode == 0xCB {
        let cb = bus.read(address.wrapping_add(1));
        let text = cb_mnemonic(cb);
        return DisassembledInstruction {
            address,
            bytes: vec![0xCB, cb],
            text,
            length: 2,
        };
    }

    let length = opcode_length(opcode);
    let mut bytes = Vec::with_capacity(length as usize);
    for i in 0..length {
        bytes.push(bus.read(address.wrapping_add(i as u16)));
    }

    let text = opcode_mnemonic(opcode, &bytes, address);

    DisassembledInstruction {
        address,
        bytes,
        text,
        length,
    }
}

pub fn disassemble_range(bus: &mut Bus, start: u16, instruction_count: usize) -> Vec<DisassembledInstruction> {
    let mut result = Vec::with_capacity(instruction_count);
    let mut pc = start;

    for _ in 0..instruction_count {
        let instruction = disassemble_at(bus, pc);
        let length = instruction.length.max(1) as u16;
        pc = pc.wrapping_add(length);
        result.push(instruction);
    }

    result
}

pub fn print_disassembly(bus: &mut Bus, start: u16, instruction_count: usize) {
    for instruction in disassemble_range(bus, start, instruction_count) {
        println!(
            "{:04X}: {:<11} {}",
            instruction.address,
            format_bytes(&instruction.bytes),
            instruction.text
        );
    }
}

fn format_bytes(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02X}", b)).collect::<Vec<_>>().join(" ")
}

fn opcode_length(opcode: u8) -> u8 {
    match opcode {
        0xCB => 2,

        // 16-bit immediate / address instructions.
        0x01 | 0x08 | 0x11 | 0x21 | 0x31 |
        0xC3 | 0xC4 | 0xC9 | 0xCA | 0xCC | 0xCD |
        0xD4 | 0xDA | 0xDC | 0xE1 | 0xE5 | 0xEA |
        0xFA | 0xF1 | 0xF5 | 0xF8 | 0xF9 => 3,

        // Relative branches, 8-bit immediate and high-page immediates.
        0x06 | 0x0E | 0x10 | 0x16 | 0x18 | 0x1E |
        0x20 | 0x26 | 0x28 | 0x2E | 0x30 | 0x36 | 0x38 | 0x3E |
        0xC6 | 0xCE | 0xD6 | 0xDE | 0xE0 | 0xE6 | 0xEE | 0xF0 | 0xF6 | 0xFE => 2,

        _ => 1,
    }
}

fn opcode_mnemonic(opcode: u8, bytes: &[u8], pc: u16) -> String {
    let imm8 = || bytes.get(1).copied().unwrap_or(0);
    let imm16 = || {
        u16::from_le_bytes([
            bytes.get(1).copied().unwrap_or(0),
            bytes.get(2).copied().unwrap_or(0),
        ])
    };

    match opcode {
        0x00 => "NOP".to_string(),
        0x01 => format!("LD BC,${:04X}", imm16()),
        0x02 => "LD (BC),A".to_string(),
        0x03 => "INC BC".to_string(),
        0x04 => "INC B".to_string(),
        0x05 => "DEC B".to_string(),
        0x06 => format!("LD B,${:02X}", imm8()),
        0x07 => "RLCA".to_string(),
        0x08 => format!("LD (${:04X}),SP", imm16()),
        0x09 => "ADD HL,BC".to_string(),
        0x0A => "LD A,(BC)".to_string(),
        0x0B => "DEC BC".to_string(),
        0x0C => "INC C".to_string(),
        0x0D => "DEC C".to_string(),
        0x0E => format!("LD C,${:02X}", imm8()),
        0x0F => "RRCA".to_string(),

        0x10 => "STOP $00".to_string(),
        0x11 => format!("LD DE,${:04X}", imm16()),
        0x12 => "LD (DE),A".to_string(),
        0x13 => "INC DE".to_string(),
        0x14 => "INC D".to_string(),
        0x15 => "DEC D".to_string(),
        0x16 => format!("LD D,${:02X}", imm8()),
        0x17 => "RLA".to_string(),
        0x18 => jr_text(pc, imm8()),
        0x19 => "ADD HL,DE".to_string(),
        0x1A => "LD A,(DE)".to_string(),
        0x1B => "DEC DE".to_string(),
        0x1C => "INC E".to_string(),
        0x1D => "DEC E".to_string(),
        0x1E => format!("LD E,${:02X}", imm8()),
        0x1F => "RRA".to_string(),

        0x20 => jr_cc_text("NZ", pc, imm8()),
        0x21 => format!("LD HL,${:04X}", imm16()),
        0x22 => "LD (HL+),A".to_string(),
        0x23 => "INC HL".to_string(),
        0x24 => "INC H".to_string(),
        0x25 => "DEC H".to_string(),
        0x26 => format!("LD H,${:02X}", imm8()),
        0x27 => "DAA".to_string(),
        0x28 => jr_cc_text("Z", pc, imm8()),
        0x29 => "ADD HL,HL".to_string(),
        0x2A => "LD A,(HL+)".to_string(),
        0x2B => "DEC HL".to_string(),
        0x2C => "INC L".to_string(),
        0x2D => "DEC L".to_string(),
        0x2E => format!("LD L,${:02X}", imm8()),
        0x2F => "CPL".to_string(),

        0x30 => jr_cc_text("NC", pc, imm8()),
        0x31 => format!("LD SP,${:04X}", imm16()),
        0x32 => "LD (HL-),A".to_string(),
        0x33 => "INC SP".to_string(),
        0x34 => "INC (HL)".to_string(),
        0x35 => "DEC (HL)".to_string(),
        0x36 => format!("LD (HL),${:02X}", imm8()),
        0x37 => "SCF".to_string(),
        0x38 => jr_cc_text("C", pc, imm8()),
        0x39 => "ADD HL,SP".to_string(),
        0x3A => "LD A,(HL-)".to_string(),
        0x3B => "DEC SP".to_string(),
        0x3C => "INC A".to_string(),
        0x3D => "DEC A".to_string(),
        0x3E => format!("LD A,${:02X}", imm8()),
        0x3F => "CCF".to_string(),

        0x40..=0x7F => {
            if opcode == 0x76 {
                "HALT".to_string()
            } else {
                format!("LD {},{}", r8_name((opcode >> 3) & 7), r8_name(opcode & 7))
            }
        }

        0x80..=0xBF => {
            let op = ["ADD A,", "ADC A,", "SUB ", "SBC A,", "AND ", "XOR ", "OR ", "CP "][(opcode >> 3 & 7) as usize];
            format!("{}{}", op, r8_name(opcode & 7))
        }

        0xC0 => "RET NZ".to_string(),
        0xC1 => "POP BC".to_string(),
        0xC2 => format!("JP NZ,${:04X}", imm16()),
        0xC3 => format!("JP ${:04X}", imm16()),
        0xC4 => format!("CALL NZ,${:04X}", imm16()),
        0xC5 => "PUSH BC".to_string(),
        0xC6 => format!("ADD A,${:02X}", imm8()),
        0xC7 => "RST $00".to_string(),
        0xC8 => "RET Z".to_string(),
        0xC9 => "RET".to_string(),
        0xCA => format!("JP Z,${:04X}", imm16()),
        0xCB => "PREFIX CB".to_string(),
        0xCC => format!("CALL Z,${:04X}", imm16()),
        0xCD => format!("CALL ${:04X}", imm16()),
        0xCE => format!("ADC A,${:02X}", imm8()),
        0xCF => "RST $08".to_string(),

        0xD0 => "RET NC".to_string(),
        0xD1 => "POP DE".to_string(),
        0xD2 => format!("JP NC,${:04X}", imm16()),
        0xD3 => "DB $D3 ; INVALID".to_string(),
        0xD4 => format!("CALL NC,${:04X}", imm16()),
        0xD5 => "PUSH DE".to_string(),
        0xD6 => format!("SUB ${:02X}", imm8()),
        0xD7 => "RST $10".to_string(),
        0xD8 => "RET C".to_string(),
        0xD9 => "RETI".to_string(),
        0xDA => format!("JP C,${:04X}", imm16()),
        0xDB => "DB $DB ; INVALID".to_string(),
        0xDC => format!("CALL C,${:04X}", imm16()),
        0xDD => "DB $DD ; INVALID".to_string(),
        0xDE => format!("SBC A,${:02X}", imm8()),
        0xDF => "RST $18".to_string(),

        0xE0 => format!("LDH ($FF00+${:02X}),A", imm8()),
        0xE1 => "POP HL".to_string(),
        0xE2 => "LD (C),A".to_string(),
        0xE3 => "DB $E3 ; INVALID".to_string(),
        0xE4 => "DB $E4 ; INVALID".to_string(),
        0xE5 => "PUSH HL".to_string(),
        0xE6 => format!("AND ${:02X}", imm8()),
        0xE7 => "RST $20".to_string(),
        0xE8 => format!("ADD SP,{:+#04X}", imm8() as i8),
        0xE9 => "JP (HL)".to_string(),
        0xEA => format!("LD (${:04X}),A", imm16()),
        0xEB => "DB $EB ; INVALID".to_string(),
        0xEC => "DB $EC ; INVALID".to_string(),
        0xED => "DB $ED ; INVALID".to_string(),
        0xEE => format!("XOR ${:02X}", imm8()),
        0xEF => "RST $28".to_string(),

        0xF0 => format!("LDH A,($FF00+${:02X})", imm8()),
        0xF1 => "POP AF".to_string(),
        0xF2 => "LD A,(C)".to_string(),
        0xF3 => "DI".to_string(),
        0xF4 => "DB $F4 ; INVALID".to_string(),
        0xF5 => "PUSH AF".to_string(),
        0xF6 => format!("OR ${:02X}", imm8()),
        0xF7 => "RST $30".to_string(),
        0xF8 => format!("LD HL,SP{:+#04X}", imm8() as i8),
        0xF9 => "LD SP,HL".to_string(),
        0xFA => format!("LD A,(${:04X})", imm16()),
        0xFB => "EI".to_string(),
        0xFC => "DB $FC ; INVALID".to_string(),
        0xFD => "DB $FD ; INVALID".to_string(),
        0xFE => format!("CP ${:02X}", imm8()),
        0xFF => "RST $38".to_string(),
    }
}

fn r8_name(index: u8) -> &'static str {
    match index {
        0 => "B",
        1 => "C",
        2 => "D",
        3 => "E",
        4 => "H",
        5 => "L",
        6 => "(HL)",
        7 => "A",
        _ => unreachable!(),
    }
}

fn jr_text(pc: u16, raw: u8) -> String {
    let target = pc
        .wrapping_add(2)
        .wrapping_add((raw as i8 as i16) as u16);
    format!("JR ${:04X}", target)
}

fn jr_cc_text(condition: &str, pc: u16, raw: u8) -> String {
    let target = pc
        .wrapping_add(2)
        .wrapping_add((raw as i8 as i16) as u16);
    format!("JR {},${:04X}", condition, target)
}

fn cb_mnemonic(opcode: u8) -> String {
    let x = opcode >> 6;
    let y = (opcode >> 3) & 7;
    let z = opcode & 7;
    let target = r8_name(z);

    match x {
        0 => {
            let op = ["RLC", "RRC", "RL", "RR", "SLA", "SRA", "SWAP", "SRL"][y as usize];
            format!("{} {}", op, target)
        }
        1 => format!("BIT {},{}", y, target),
        2 => format!("RES {},{}", y, target),
        3 => format!("SET {},{}", y, target),
        _ => unreachable!(),
    }
}
