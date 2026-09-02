use std::collections::{HashMap, HashSet, VecDeque};

use crate::{bus::Bus, cpu::Cpu};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DebugAction { Run, Step, Continue, Break }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WatchType { Change, Read, Write }

#[derive(Clone, Debug)]
pub struct MemoryWatch { pub address: u16, pub kind: WatchType, pub last_value: u8 }

#[derive(Clone, Debug)]
pub struct TraceEntry { pub address: u16, pub bytes: Vec<u8>, pub text: String }

#[derive(Clone, Debug)]
pub struct DisassembledInstruction { pub address: u16, pub bytes: Vec<u8>, pub text: String, pub length: u8 }

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
}

impl Debugger {
    pub fn new() -> Self {
        Self { 
            enabled:false, 
            action:DebugAction::Run, 
            breakpoints:HashSet::new(), 
            watches:HashMap::new(), 
            trace_enabled:false, 
            trace_start:0, 
            trace_end:0, 
            history:VecDeque::with_capacity(256), 
            history_limit:256, 
            frame:0, 
            stop_reason:None 
        }
    }
    pub fn enable(&mut self) { 
        self.enabled=true; 
    }
    pub fn disable(&mut self) { 
        self.enabled=false; 
        self.action=DebugAction::Run; 
        self.stop_reason=None; 
    }
    pub fn break_now(&mut self, reason: impl Into<String>) { 
        self.enabled=true; 
        self.action=DebugAction::Break; 
        self.stop_reason=Some(reason.into()); 
    }
    pub fn step(&mut self) { 
        self.enabled=true; 
        self.action=DebugAction::Step; 
        self.stop_reason=None; 
    }
    pub fn continue_execution(&mut self) { 
        self.enabled=true; 
        self.action=DebugAction::Continue; 
        self.stop_reason=None; 
    }
    pub fn add_breakpoint(&mut self, address:u16) { 
        self.breakpoints.insert(address); 
    }
    pub fn remove_breakpoint(&mut self, address:u16) { 
        self.breakpoints.remove(&address); 
    }
    pub fn clear_breakpoints(&mut self) { 
        self.breakpoints.clear(); 
    }
    pub fn has_breakpoint(&self, address:u16) -> bool { 
        self.breakpoints.contains(&address) 
    }

    pub fn list_breakpoints(&self) {
        if self.breakpoints.is_empty() { 
            println!("DEBUG: brak breakpointów");
            return; 
        }
        let mut points:Vec<_>=self.breakpoints.iter().copied().collect(); 
        points.sort_unstable();
        for address in points { 
            println!("BP {:04X}",address); 
        }
    }

    pub fn set_trace_range(&mut self,start:u16,end:u16) { 
        self.trace_start=start; 
        self.trace_end=end; 
        self.trace_enabled=true; 
        println!("DEBUG: trace {:04X}-{:04X}",start,end); 
    }
    pub fn disable_trace(&mut self) { 
        self.trace_enabled=false; 
        println!("DEBUG: trace wyłączony"); 
    }

    pub fn watch(&mut self,bus:&mut Bus,address:u16) {
        let value=bus.read(address);
        self.watches.insert(address,MemoryWatch{address,kind:WatchType::Change,last_value:value});
        println!("WATCH ${:04X} = {:02X}",address,value);
    }
    pub fn watch_kind(&mut self,bus:&mut Bus,address:u16,kind:WatchType) { 
        let value=bus.read(address); 
        self.watches.insert(address,MemoryWatch{address,kind,last_value:value}); 
    }
    pub fn unwatch(&mut self,address:u16) { 
        if self.watches.remove(&address).is_some() { 
            println!("DEBUG: watch usunięty ${:04X}",address); 
        } 
    }
    pub fn list_watches(&self) {
        if self.watches.is_empty() { 
            println!("DEBUG: brak watchpointów"); 
            return; 
        }
        let mut addresses:Vec<_>=self.watches.keys().copied().collect(); addresses.sort_unstable();
        for address in addresses { 
            let w=&self.watches[&address]; 
            println!("WATCH ${:04X} = {:02X} ({:?})",address,w.last_value,w.kind); 
        }
    }

    pub fn before_instruction(&mut self,cpu:&Cpu,bus:&mut Bus)->bool {
        if !self.enabled { return true; }
        if self.action==DebugAction::Break { return false; }
        if self.breakpoints.contains(&cpu.pc) {
            self.action=DebugAction::Break;
            self.stop_reason=Some(format!("Breakpoint hit at {:04X}",cpu.pc));
            self.print_status(cpu); self.print_stop_disassembly(bus,cpu.pc); return false;
        }
        if self.trace_enabled && Self::in_trace_range(cpu.pc,self.trace_start,self.trace_end) { 
            let i=disassemble_at(bus,cpu.pc); self.trace_instruction(&i); 
        }
        self.check_change_watches(bus); true
    }

    pub fn after_instruction_hook(&mut self,cpu:&Cpu,bus:&mut Bus) {
        self.check_change_watches(bus);
        if self.action==DebugAction::Step {
            self.action=DebugAction::Break;
            self.stop_reason=Some("Single step complete".to_string());
            self.print_status(cpu); self.print_stop_disassembly(bus,cpu.pc);
        }
    }

    fn check_change_watches(&mut self,bus:&mut Bus) {
        for w in self.watches.values_mut() {
            let value=bus.read(w.address);
            if w.kind==WatchType::Change && value!=w.last_value { 
                println!("WATCH ${:04X}: {:02X} -> {:02X} (frame {})",
                w.address,
                w.last_value,
                value,self.frame); 
                w.last_value=value; 
            }
        }
    }
    pub fn next_frame(&mut self,bus:&mut Bus) { self.frame=self.frame.wrapping_add(1); self.check_change_watches(bus); }

    pub fn print_status(&self,cpu:&Cpu) {
        println!("CPU: PC={:04X} AF={:04X} BC={:04X} DE={:04X} HL={:04X} SP={:04X} IME={} HALT={}",
        cpu.pc,
        cpu.af(),
        cpu.bc(),
        cpu.de(),
        cpu.hl(),
        cpu.sp,
        cpu.ime,
        cpu.halted);
        println!("FLAGS: Z={} N={} H={} C={}",
        cpu.f&0x80!=0,
        cpu.f&0x40!=0,
        cpu.f&0x20!=0,
        cpu.f&0x10!=0);
        if let Some(reason)=&self.stop_reason { 
            println!("DEBUG: {}",reason); 
        }
    }
    pub fn print_stop_disassembly(&self,bus:&mut Bus,pc:u16) { 
        println!("PC={:04X}",pc); 
        self.print_disassembly(bus,pc,3); 
    }
    pub fn print_disassembly(&self,bus:&mut Bus,start:u16,count:usize) { 
        for i in disassemble_range(bus,start,count) { 
            println!("{:04X}: {:<11} {}",i.address,format_bytes(&i.bytes),i.text); 
        } 
    }
    pub fn dump_memory(&self,bus:&mut Bus,address:u16,length:usize) { 
        for offset in (0..length).step_by(16) { 
            let mut line=format!("{:04X}: ",address.wrapping_add(offset as u16)); 
            for i in 0..16 { 
                if offset+i>=length { break; 
                } 
                line.push_str(&format!("{:02X} ",bus.read(address.wrapping_add((offset+i) as u16))));
            } 
            println!("{}",line); 
        } 
    }
    pub fn print_history(&self) { 
        for e in &self.history { 
            println!("{:04X}: {:<11} {}",e.address,format_bytes(&e.bytes),e.text); 
        } 
    }
    pub fn trace_instruction(&mut self,i:&DisassembledInstruction) { 
        if self.history.len()>=self.history_limit { 
            self.history.pop_front(); 
        } 
        let e=TraceEntry{address:i.address,bytes:i.bytes.clone(),text:i.text.clone()}; 
        println!("{:04X}: {:<11} {}",e.address,format_bytes(&e.bytes),e.text); 
        self.history.push_back(e); 
    }
    fn in_trace_range(address:u16,start:u16,end:u16)->bool { 
        if start<=end { 
            address>=start&&address<=end 
        } 
        else { 
            address>=start||address<=end 
        } 
    }
}

pub fn disassemble_at(bus:&mut Bus,address:u16)->DisassembledInstruction {
    let opcode=bus.read(address); let length=opcode_length(opcode); let mut bytes=Vec::with_capacity(length as usize);
    for i in 0..length { bytes.push(bus.read(address.wrapping_add(i as u16))); }
    let text=opcode_mnemonic(opcode,&bytes,address); DisassembledInstruction{address,bytes,text,length}
}
pub fn disassemble_range(bus:&mut Bus,start:u16,count:usize)->Vec<DisassembledInstruction> { 
    let mut out=Vec::with_capacity(count); 
    let mut pc=start; for _ in 0..count { 
        let i=disassemble_at(bus,pc); 
        pc=pc.wrapping_add(i.length.max(1) as u16); 
        out.push(i); 
    } 
    out 
}
pub fn print_disassembly(bus:&mut Bus,start:u16,count:usize) { 
    for i in disassemble_range(bus,start,count) { 
        println!("{:04X}: {:<11} {}",i.address,format_bytes(&i.bytes),i.text); 
    } 
}
fn format_bytes(bytes:&[u8])->String { 
    bytes.iter().map(|b|format!("{:02X}",b)).collect::<Vec<_>>().join(" ") 
}

fn opcode_length(op:u8)->u8 {
    if op==0xCB { return 2; }
    match op {
        0x01|0x08|0x11|0x21|0x31|0xC2|0xC3|0xC4|0xCA|0xCC|0xCD|0xD2|0xD4|0xDA|0xDC|0xEA|0xFA=>3,
        0x06|0x0E|0x10|0x16|0x18|0x1E|0x20|0x26|0x28|0x2E|0x30|0x36|0x38|0x3E|0xC6|0xCE|0xD6|0xDE|0xE0|0xE6|0xE8|0xEE|0xF0|0xF6|0xF8|0xFE=>2,
        _=>1,
    }
}

fn reg_name(i:u8)->&'static str { match i&7 {0=>"B",1=>"C",2=>"D",3=>"E",4=>"H",5=>"L",6=>"(HL)",_=>"A"} }
fn opcode_mnemonic(op:u8,b:&[u8],pc:u16)->String {
    let n=||b.get(1).copied().unwrap_or(0); let nn=||u16::from_le_bytes([b.get(1).copied().unwrap_or(0),b.get(2).copied().unwrap_or(0)]);
    if op==0xCB { return cb_mnemonic(n()); }
    match op {
        0x00=>"NOP".into(),
        0x01=>format!("LD BC,${:04X}",nn()),
        0x02=>"LD (BC),A".into(),
        0x03=>"INC BC".into(),
        0x04=>"INC B".into(),
        0x05=>"DEC B".into(),
        0x06=>format!("LD B,${:02X}",n()),
        0x07=>"RLCA".into(),
        0x08=>format!("LD (${:04X}),SP",nn()),
        0x09=>"ADD HL,BC".into(),
        0x0A=>"LD A,(BC)".into(),
        0x0B=>"DEC BC".into(),
        0x0C=>"INC C".into(),
        0x0D=>"DEC C".into(),
        0x0E=>format!("LD C,${:02X}",n()),
        0x0F=>"RRCA".into(),
        0x10=>"STOP $00".into(),
        0x11=>format!("LD DE,${:04X}",nn()),
        0x12=>"LD (DE),A".into(),
        0x13=>"INC DE".into(),
        0x14=>"INC D".into(),
        0x15=>"DEC D".into(),
        0x16=>format!("LD D,${:02X}",n()),
        0x17=>"RLA".into(),
        0x18=>jr_text(pc,n()),
        0x19=>"ADD HL,DE".into(),
        0x1A=>"LD A,(DE)".into(),
        0x1B=>"DEC DE".into(),
        0x1C=>"INC E".into(),
        0x1D=>"DEC E".into(),
        0x1E=>format!("LD E,${:02X}",n()),
        0x1F=>"RRA".into(),
        0x20=>jr_cc("NZ",pc,n()),
        0x21=>format!("LD HL,${:04X}",nn()),
        0x22=>"LD (HL+),A".into(),
        0x23=>"INC HL".into(),
        0x24=>"INC H".into(),
        0x25=>"DEC H".into(),
        0x26=>format!("LD H,${:02X}",n()),
        0x27=>"DAA".into(),
        0x28=>jr_cc("Z",pc,n()),
        0x29=>"ADD HL,HL".into(),
        0x2A=>"LD A,(HL+)".into(),
        0x2B=>"DEC HL".into(),
        0x2C=>"INC L".into(),
        0x2D=>"DEC L".into(),
        0x2E=>format!("LD L,${:02X}",n()),
        0x2F=>"CPL".into(),
        0x30=>jr_cc("NC",pc,n()),
        0x31=>format!("LD SP,${:04X}",nn()),
        0x32=>"LD (HL-),A".into(),
        0x33=>"INC SP".into(),
        0x34=>"INC (HL)".into(),
        0x35=>"DEC (HL)".into(),
        0x36=>format!("LD (HL),${:02X}",n()),
        0x37=>"SCF".into(),
        0x38=>jr_cc("C",pc,n()),
        0x39=>"ADD HL,SP".into(),
        0x3A=>"LD A,(HL-)".into(),
        0x3B=>"DEC SP".into(),
        0x3C=>"INC A".into(),
        0x3D=>"DEC A".into(),
        0x3E=>format!("LD A,${:02X}",n()),
        0x3F=>"CCF".into(),
        0x76=>"HALT".into(),
        0x40..=0x7F=>format!("LD {},{}",reg_name(op>>3),reg_name(op)),
        0x80..=0x87=>format!("ADD A,{}",reg_name(op)),
        0x88..=0x8F=>format!("ADC A,{}",reg_name(op)),
        0x90..=0x97=>format!("SUB {}",reg_name(op)),
        0x98..=0x9F=>format!("SBC A,{}",reg_name(op)),
        0xA0..=0xA7=>format!("AND {}",reg_name(op)),
        0xA8..=0xAF=>format!("XOR {}",reg_name(op)),
        0xB0..=0xB7=>format!("OR {}",reg_name(op)),
        0xB8..=0xBF=>format!("CP {}",reg_name(op)),
        0xC0=>"RET NZ".into(),
        0xC1=>"POP BC".into(),
        0xC2=>format!("JP NZ,${:04X}",nn()),
        0xC3=>format!("JP ${:04X}",nn()),
        0xC4=>format!("CALL NZ,${:04X}",nn()),
        0xC5=>"PUSH BC".into(),
        0xC6=>format!("ADD A,${:02X}",n()),
        0xC7=>"RST $00".into(),
        0xC8=>"RET Z".into(),
        0xC9=>"RET".into(),
        0xCA=>format!("JP Z,${:04X}",nn()),
        0xCC=>format!("CALL Z,${:04X}",nn()),
        0xCD=>format!("CALL ${:04X}",nn()),
        0xCE=>format!("ADC A,${:02X}",n()),
        0xCF=>"RST $08".into(),
        0xD0=>"RET NC".into(),
        0xD1=>"POP DE".into(),
        0xD2=>format!("JP NC,${:04X}",nn()),
        0xD4=>format!("CALL NC,${:04X}",nn()),
        0xD5=>"PUSH DE".into(),
        0xD6=>format!("SUB ${:02X}",n()),
        0xD7=>"RST $10".into(),
        0xD8=>"RET C".into(),
        0xD9=>"RETI".into(),
        0xDA=>format!("JP C,${:04X}",nn()),
        0xDC=>format!("CALL C,${:04X}",nn()),
        0xDE=>format!("SBC A,${:02X}",n()),
        0xDF=>"RST $18".into(),
        0xE0=>format!("LDH ($FF00+${:02X}),A",n()),
        0xE1=>"POP HL".into(),
        0xE2=>"LD ($FF00+C),A".into(),
        0xE5=>"PUSH HL".into(),
        0xE6=>format!("AND ${:02X}",n()),
        0xE7=>"RST $20".into(),
        0xE8=>format!("ADD SP,{:+}",(n() as i8)),
        0xE9=>"JP (HL)".into(),
        0xEA=>format!("LD (${:04X}),A",nn()),
        0xEE=>format!("XOR ${:02X}",n()),
        0xEF=>"RST $28".into(),
        0xF0=>format!("LDH A,($FF00+${:02X})",n()),
        0xF1=>"POP AF".into(),
        0xF2=>"LD A,($FF00+C)".into(),
        0xF3=>"DI".into(),
        0xF5=>"PUSH AF".into(),
        0xF6=>format!("OR ${:02X}",n()),
        0xF7=>"RST $30".into(),
        0xF8=>format!("LD HL,SP{:+}",(n() as i8)),
        0xF9=>"LD SP,HL".into(),
        0xFA=>format!("LD A,(${:04X})",nn()),
        0xFB=>"EI".into(),
        0xFE=>format!("CP ${:02X}",n()),
        0xFF=>"RST $38".into(),
        _=>format!("DB ${:02X}",op),
    }
}
fn jr_text(pc:u16,o:u8)->String { 
    format!("JR ${:04X}",pc.wrapping_add(2).wrapping_add((o as i8) as i16 as u16)) 
}
fn jr_cc(c:&str,pc:u16,o:u8)->String { 
    format!("JR {},${:04X}",c,pc.wrapping_add(2).wrapping_add((o as i8) as i16 as u16)) 
}
fn cb_mnemonic(op:u8)->String { 
    let r=reg_name(op); 
    match op {
        0x00..=0x07=>format!("RLC {}",r),
        0x08..=0x0F=>format!("RRC {}",r),
        0x10..=0x17=>format!("RL {}",r),
        0x18..=0x1F=>format!("RR {}",r),
        0x20..=0x27=>format!("SLA {}",r),
        0x28..=0x2F=>format!("SRA {}",r),
        0x30..=0x37=>format!("SWAP {}",r),
        0x38..=0x3F=>format!("SRL {}",r),
        0x40..=0x7F=>format!("BIT {},{}",
        (op>>3)&7,r),
        0x80..=0xBF=>format!("RES {},{}",(op>>3)&7,r),
        _=>format!("SET {},{}",(op>>3)&7,r)
    }

