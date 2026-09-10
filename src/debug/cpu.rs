pub struct Cpu {
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

    pub opcode_counts: [u64; 256],
    // Temporary one-shot tracer for debugging PC target hits
    pub debug_pc_traced: bool,
    // previous-instruction snapshot
    pub prev_pc: u16,
    pub prev_opcode: u8,
    pub prev_sp: u16,
    // recent instruction history (pc, opcode, sp) for diagnostics
    pub instr_history: Vec<(u16, u8, u16)>,
}



impl Cpu {
    fn maybe_trace_pc(&mut self, new_pc: u16, cause: &str, bus: &mut crate::bus::Bus, prev_pc: u16, prev_opcode: u8, sp_before: u16) {
        if self.debug_pc_traced || new_pc != 0x5C05 {
            return;
        }

        self.debug_pc_traced = true;

        // Stack bytes at SP
        let b0 = bus.read(sp_before);
        let b1 = bus.read(sp_before.wrapping_add(1));

        let low = bus.mbc1.rom_bank_low as usize;
        let high = bus.mbc1.rom_bank_high as usize;
        let banking = bus.mbc1.banking_mode as usize;
        let bank_4000 = if banking == 0 { (high << 5) | low } else { low };

        let mut surround = Vec::new();
        for i in 0..16u16 {
            let addr = prev_pc.wrapping_sub(8).wrapping_add(i);
            surround.push((addr, bus.read(addr)));
        }

        eprintln!("--- PC TARGET TRACE (one-shot) ---");
        eprintln!("cause={} prev_pc=${:04X} prev_opcode={:02X}", cause, prev_pc, prev_opcode);
        eprintln!("new_pc=${:04X}", new_pc);
        eprintln!("sp_before=${:04X} sp_bytes={:02X} {:02X}", sp_before, b0, b1);
        eprintln!("IME={} halted={}", self.ime, self.halted);
        eprintln!("mbc1.low={} high={} banking={} bank_4000={}", low, high, banking, bank_4000);
        eprintln!("Surrounding bytes around prev_pc:");
        for (a, v) in surround {
            eprint!("{:04X}:{:02X} ", a, v);
        }
        eprintln!("\nRecent instruction history (most recent last):");
        for (a, op, sp) in &self.instr_history {
            eprintln!("  {:04X}: {:02X} SP={:04X}", a, op, sp);
        }
        eprintln!("\n--- end trace ---");
    }
    pub fn new() -> Self {
        Self {
            a: 0,
            f: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            sp: 0,
            pc: 0,
            ime: false,
            ime_pending: false,
            halted: false,
            opcode_counts: [0; 256],
            debug_pc_traced: false,
            prev_pc: 0,
            prev_opcode: 0,
            prev_sp: 0,
            instr_history: Vec::new(),
        }
    }

    pub fn reset(&mut self) {
        self.a = 0x01;
        self.f = 0xB0;
        self.b = 0x00;
        self.c = 0x13;
        self.d = 0x00;
        self.e = 0xD8;
        self.h = 0x01;
        self.l = 0x4D;
        self.sp = 0xFFFE;
        self.pc = 0x0100;

        self.ime = false;
        self.ime_pending = false;
        self.halted = false;
        self.debug_pc_traced = false;
        self.prev_pc = 0;
        self.prev_opcode = 0;
        self.prev_sp = 0;
        self.instr_history.clear();
    }

    pub fn af(&self) -> u16 {
        ((self.a as u16) << 8) | self.f as u16
    }

    pub fn bc(&self) -> u16 {
        ((self.b as u16) << 8) | self.c as u16
    }

    pub fn de(&self) -> u16 {
        ((self.d as u16) << 8) | self.e as u16
    }

    pub fn hl(&self) -> u16 {
        ((self.h as u16) << 8) | self.l as u16
    }

    pub fn set_bc(&mut self, value: u16) {
        self.b = (value >> 8) as u8;
        self.c = value as u8;
    }

    pub fn set_de(&mut self, value: u16) {
        self.d = (value >> 8) as u8;
        self.e = value as u8;
    }

    pub fn set_hl(&mut self, value: u16) {
        self.h = (value >> 8) as u8;
        self.l = value as u8;
    }

    fn set_flags(&mut self, z: bool, n: bool, h: bool, c: bool) {
        self.f = 0;

        if z {
            self.f |= 0x80;
        }

        if n {
            self.f |= 0x40;
        }

        if h {
            self.f |= 0x20;
        }

        if c {
            self.f |= 0x10;
        }
    }

    fn flag_z(&self) -> bool {
        self.f & 0x80 != 0
    }

    fn flag_c(&self) -> bool {
        self.f & 0x10 != 0
    }

    fn condition(&self, cc: u8) -> bool {
        match cc {
            0 => !self.flag_z(), // NZ
            1 => self.flag_z(),  // Z
            2 => !self.flag_c(), // NC
            3 => self.flag_c(),  // C
            _ => false,
        }
    }

    fn read_imm8(&mut self, bus: &mut crate::bus::Bus) -> u8 {
        let value = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        value
    }

    fn read_imm16(&mut self, bus: &mut crate::bus::Bus) -> u16 {
        let low = self.read_imm8(bus);
        let high = self.read_imm8(bus);

        u16::from_le_bytes([low, high])
    }

    fn push(&mut self, bus: &mut crate::bus::Bus, value: u16) {
        let _old_sp = self.sp;

        self.sp = self.sp.wrapping_sub(1);
        let high_addr = self.sp;
        bus.write(high_addr, (value >> 8) as u8);

        self.sp = self.sp.wrapping_sub(1);
        let low_addr = self.sp;
        bus.write(low_addr, value as u8);

        // println!(
        //     "PUSH value={:04X} SP {:04X}->{:04X} [{}]={:02X} [{}]={:02X}",
        //     value,
        //     old_sp,
        //     self.sp,
        //     low_addr,
        //     bus.read(low_addr),
        //     high_addr,
        //     bus.read(high_addr),
        // );
    }

    fn pop(&mut self, bus: &mut crate::bus::Bus) -> u16 {
        let _old_sp = self.sp;

        let _low_addr = self.sp;
        let low = bus.read(self.sp);
        self.sp = self.sp.wrapping_add(1);

        let _high_addr = self.sp;
        let high = bus.read(self.sp);
        self.sp = self.sp.wrapping_add(1);

        let value = u16::from_le_bytes([low, high]);

        // println!(
        //     "POP value={:04X} SP {:04X}->{:04X} [{}]={:02X} [{}]={:02X}",
        //     value,
        //     old_sp,
        //     self.sp,
        //     low_addr,
        //     low,
        //     high_addr,
        //     high,
        // );

        value
    }

    fn read_r8(&self, bus: &mut crate::bus::Bus, index: u8) -> u8 {
        match index {
            0 => self.b,
            1 => self.c,
            2 => self.d,
            3 => self.e,
            4 => self.h,
            5 => self.l,
            6 => bus.read(self.hl()),
            7 => self.a,
            _ => unreachable!(),
        }
    }

    fn write_r8(&mut self, bus: &mut crate::bus::Bus, index: u8, value: u8) {
        match index {
            0 => self.b = value,
            1 => self.c = value,
            2 => self.d = value,
            3 => self.e = value,
            4 => self.h = value,
            5 => self.l = value,
            6 => {
                let address = self.hl();
                bus.write(address, value);
            }
            7 => self.a = value,
            _ => unreachable!(),
        }
    }

    fn read_rr(&self, index: u8) -> u16 {
        match index {
            0 => self.bc(),
            1 => self.de(),
            2 => self.hl(),
            3 => self.sp,
            _ => unreachable!(),
        }
    }

    fn alu_a(&mut self, op: u8, value: u8) {
        match op {
            // ADD
            0 => {
                let a = self.a;
                let result = a.wrapping_add(value);

                self.set_flags(
                    result == 0,
                    false,
                    ((a & 0x0F) + (value & 0x0F)) > 0x0F,
                    (a as u16 + value as u16) > 0xFF,
                );

                self.a = result;
            }

            // ADC
            1 => {
                let a = self.a;
                let carry = if self.flag_c() { 1u8 } else { 0 };
                let result = a.wrapping_add(value).wrapping_add(carry);

                self.set_flags(
                    result == 0,
                    false,
                    ((a & 0x0F) + (value & 0x0F) + carry) > 0x0F,
                    (a as u16 + value as u16 + carry as u16) > 0xFF,
                );

                self.a = result;
            }

            // SUB
            2 => {
                let a = self.a;
                let result = a.wrapping_sub(value);

                self.set_flags(result == 0, true, (a & 0x0F) < (value & 0x0F), a < value);

                self.a = result;
            }

            // SBC
            3 => {
                let a = self.a as u16;
                let value = value as u16;
                let carry = if self.flag_c() { 1u16 } else { 0 };
                let result = a.wrapping_sub(value).wrapping_sub(carry);

                self.set_flags(
                    result as u8 == 0,
                    true,
                    (a & 0x0F) < (value & 0x0F) + carry,
                    a < value + carry,
                );

                self.a = result as u8;
            }

            // AND
            4 => {
                self.a &= value;

                self.set_flags(self.a == 0, false, true, false);
            }

            // XOR
            5 => {
                self.a ^= value;

                self.set_flags(self.a == 0, false, false, false);
            }

            // OR
            6 => {
                self.a |= value;

                self.set_flags(self.a == 0, false, false, false);
            }

            // CP
            7 => {
                let a = self.a;
                let result = a.wrapping_sub(value);

                self.set_flags(result == 0, true, (a & 0x0F) < (value & 0x0F), a < value);
            }

            _ => unreachable!(),
        }
    }

    fn execute_cb(&mut self, bus: &mut crate::bus::Bus, opcode: u8) -> u32 {
        let x = opcode >> 6;
        let y = (opcode >> 3) & 0x07;
        let z = opcode & 0x07;

        match x {
            // Rotate / shift / SWAP
            0 => {
                let old = self.read_r8(bus, z);

                let (result, carry) = match y {
                    // RLC
                    0 => {
                        let carry = old & 0x80 != 0;
                        (old.rotate_left(1), carry)
                    }

                    // RRC
                    1 => {
                        let carry = old & 0x01 != 0;
                        (old.rotate_right(1), carry)
                    }

                    // RL
                    2 => {
                        let carry = old & 0x80 != 0;
                        let old_carry = if self.flag_c() { 1 } else { 0 };

                        ((old << 1) | old_carry, carry)
                    }

                    // RR
                    3 => {
                        let carry = old & 0x01 != 0;
                        let old_carry = if self.flag_c() { 0x80 } else { 0 };

                        ((old >> 1) | old_carry, carry)
                    }

                    // SLA
                    4 => {
                        let carry = old & 0x80 != 0;
                        (old << 1, carry)
                    }

                    // SRA
                    5 => {
                        let carry = old & 0x01 != 0;
                        ((old >> 1) | (old & 0x80), carry)
                    }

                    // SWAP
                    6 => (old.rotate_left(4), false),

                    // SRL
                    7 => {
                        let carry = old & 0x01 != 0;
                        (old >> 1, carry)
                    }

                    _ => unreachable!(),
                };

                self.write_r8(bus, z, result);

                self.set_flags(result == 0, false, false, carry);

                if z == 6 { 16 } else { 8 }
            }

            // BIT
            1 => {
                let value = self.read_r8(bus, z);
                let bit_set = value & (1 << y) != 0;
                let carry = self.flag_c();

                self.set_flags(!bit_set, false, true, carry);

                if z == 6 { 12 } else { 8 }
            }

            // RES
            2 => {
                let value = self.read_r8(bus, z);
                let result = value & !(1 << y);

                self.write_r8(bus, z, result);

                if z == 6 { 16 } else { 8 }
            }

            // SET
            3 => {
                let value = self.read_r8(bus, z);
                let result = value | (1 << y);

                self.write_r8(bus, z, result);

                if z == 6 { 16 } else { 8 }
            }

            _ => unreachable!(),
        }
    }

    fn execute_interrupt(&mut self, bus: &mut crate::bus::Bus, pending: u8) -> u32 {
        let interrupt_bit;
        let vector;

        if pending & 0x01 != 0 {
            interrupt_bit = 0x01;
            vector = 0x0040;
        } else if pending & 0x02 != 0 {
            interrupt_bit = 0x02;
            vector = 0x0048;
        } else if pending & 0x04 != 0 {
            interrupt_bit = 0x04;
            vector = 0x0050;
        } else if pending & 0x08 != 0 {
            interrupt_bit = 0x08;
            vector = 0x0058;
        } else {
            interrupt_bit = 0x10;
            vector = 0x0060;
        }

        let if_reg = bus.read(0xFF0F);
        let _ie = bus.read(0xFFFF);
        self.ime = false;
        self.halted = false;

        let new_if = if_reg & !interrupt_bit;
        bus.write(0xFF0F, new_if);

        self.push(bus, self.pc);
        self.pc = vector;
        20
    }

    pub fn step(&mut self, bus: &mut crate::bus::Bus) -> u32 {
        let enable_ime = self.ime_pending;
        self.ime_pending = false;

        let if_reg = bus.read(0xFF0F);
        let ie = bus.read(0xFFFF);
        let pending = if_reg & ie & 0x1F;

        if self.halted {
            if pending != 0 {
                self.halted = false;

                if self.ime {
                    return self.execute_interrupt(bus, pending);
                }
            }

            if enable_ime {
                self.ime = true;
            }
            return 4;
        }

        if self.ime && pending != 0 {
            return self.execute_interrupt(bus, pending);
        }

        let cycles = self.execute(bus);
        if self.pc == 0x0100 {
            println!("CPU WRÓCIŁ DO 0100");
        }

        if enable_ime {
            self.ime = true;
        }

        cycles
    }

    fn execute(&mut self, bus: &mut crate::bus::Bus) -> u32 {
        // Temporary diagnostic: inspect mapping when PC reaches 0x5C05
        if std::env::var("DEBUG_F4_TRACE").is_ok() && self.pc == 0x5C05 {
            let addr = self.pc;
            let low = bus.mbc1.rom_bank_low as usize;
            let high = bus.mbc1.rom_bank_high as usize;
            let banking = bus.mbc1.banking_mode as usize;

            let bank_4000 = if banking == 0 { (high << 5) | low } else { low };
            let bank_0000 = if banking == 0 { 0 } else { high << 5 };

            let mapped = bus.mbc1.read(&bus.rom, addr);
            let file_offset = bank_4000 * 0x4000 + (addr as usize - 0x4000);
            let file_byte = bus.rom.read(file_offset);

            eprintln!(
                "DEBUG_F4_TRACE PC={:04X} mbc1.low={} high={} banking={} bank_4000={} mapped={:02X} file_off={:06X} file_byte={:02X}",
                addr, low, high, banking, bank_4000, mapped, file_offset, file_byte
            );
        }

        let prev_pc = self.pc;
        let sp_before = self.sp;

        let opcode = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);

        // record previous-instruction snapshot for potential tracing
        self.prev_pc = prev_pc;
        self.prev_opcode = opcode;
        self.prev_sp = sp_before;
        // push into small history buffer
        if self.instr_history.len() >= 16 {
            self.instr_history.remove(0);
        }
        self.instr_history.push((prev_pc, opcode, sp_before));

        self.opcode_counts[opcode as usize] += 1;

        if opcode == 0xCB {
            let cb_opcode = self.read_imm8(bus);
            return self.execute_cb(bus, cb_opcode);
        }
        // -------------------------------------------------
        // LD r8,r8
        // -------------------------------------------------

        if (0x40..=0x7F).contains(&opcode) {
            // HALT
            if opcode == 0x76 {
                self.halted = true;
                return 4;
            }

            let dst = (opcode >> 3) & 7;
            let src = opcode & 7;
            let value = self.read_r8(bus, src);

            self.write_r8(bus, dst, value);

            if dst == 6 || src == 6 {
                return 8;
            }

            return 4;
        }

        // -------------------------------------------------
        // ALU A,r
        // -------------------------------------------------

        if (0x80..=0xBF).contains(&opcode) {
            let op = (opcode >> 3) & 7;
            let src = opcode & 7;
            let value = self.read_r8(bus, src);

            self.alu_a(op, value);

            if src == 6 {
                return 8;
            }

            return 4;
        }

        // -------------------------------------------------
        // LD r,d8
        // -------------------------------------------------

        if opcode & 0xC7 == 0x06 {
            let dst = (opcode >> 3) & 7;
            let value = self.read_imm8(bus);

            self.write_r8(bus, dst, value);

            if dst == 6 {
                return 12;
            }

            return 8;
        }

        // -------------------------------------------------
        // INC r8
        // -------------------------------------------------

        if opcode & 0xC7 == 0x04 {
            let index = (opcode >> 3) & 7;
            let old = self.read_r8(bus, index);
            let result = old.wrapping_add(1);

            let carry = self.flag_c();

            self.write_r8(bus, index, result);

            self.set_flags(result == 0, false, (old & 0x0F) == 0x0F, carry);

            if index == 6 {
                return 12;
            }

            return 4;
        }

        // -------------------------------------------------
        // DEC r8
        // -------------------------------------------------

        if opcode & 0xC7 == 0x05 {
            let index = (opcode >> 3) & 7;
            let old = self.read_r8(bus, index);
            let result = old.wrapping_sub(1);

            let carry = self.flag_c();

            self.write_r8(bus, index, result);

            self.set_flags(result == 0, true, (old & 0x0F) == 0, carry);

            if index == 6 {
                return 12;
            }

            return 4;
        }

        match opcode {
            // NOP
            0x00 => 4,

            // STOP 00
            0x10 => {
                let _ = self.read_imm8(bus);
                self.halted = true;
                4
            }

            // LD BC,d16
            0x01 => {
                let value = self.read_imm16(bus);
                self.set_bc(value);
                12
            }

            // LD (BC),A
            0x02 => {
                bus.write(self.bc(), self.a);
                8
            }

            // INC BC
            0x03 => {
                self.set_bc(self.bc().wrapping_add(1));
                8
            }

            // DEC BC
            0x0B => {
                let old_bc = self.bc();

                let new_bc = old_bc.wrapping_sub(1);
                self.set_bc(new_bc);

                8
            }

            // LD A,(BC)
            0x0A => {
                self.a = bus.read(self.bc());
                8
            }

            // RLCA
            0x07 => {
                let carry = self.a & 0x80 != 0;
                self.a = self.a.rotate_left(1);

                self.set_flags(false, false, false, carry);

                4
            }

            // RRCA
            0x0F => {
                let carry = self.a & 0x01 != 0;
                self.a = self.a.rotate_right(1);

                self.set_flags(false, false, false, carry);

                4
            }

            // LD DE,d16
            0x11 => {
                let value = self.read_imm16(bus);
                self.set_de(value);
                12
            }

            // LD (DE),A
            0x12 => {
                bus.write(self.de(), self.a);
                8
            }

            // INC DE
            0x13 => {
                self.set_de(self.de().wrapping_add(1));
                8
            }

            // DEC DE
            0x1B => {
                self.set_de(self.de().wrapping_sub(1));
                8
            }

            // LD A,(DE)
            0x1A => {
                let address = self.de();
                let value = bus.read(address);
                self.a = value;
                8
            }

            // RLA
            0x17 => {
                let carry = self.a & 0x80 != 0;
                let old_carry = if self.flag_c() { 1 } else { 0 };

                self.a = (self.a << 1) | old_carry;

                self.set_flags(false, false, false, carry);

                4
            }

            // RRA
            0x1F => {
                let carry = self.a & 0x01 != 0;
                let old_carry = if self.flag_c() { 0x80 } else { 0 };

                self.a = (self.a >> 1) | old_carry;

                self.set_flags(false, false, false, carry);

                4
            }

            // LD HL,d16
            0x21 => {
                let value = self.read_imm16(bus);
                self.set_hl(value);
                12
            }

            // LD (HL+),A
            0x22 => {
                let address = self.hl();

                bus.write(address, self.a);
                self.set_hl(address.wrapping_add(1));
                8
            }

            // INC HL
            0x23 => {
                self.set_hl(self.hl().wrapping_add(1));
                8
            }

            // INC SP
            0x33 => {
                self.sp = self.sp.wrapping_add(1);
                8
            }

            // DEC SP
            0x3B => {
                self.sp = self.sp.wrapping_sub(1);
                8
            }

            // LD A,(HL+)
            0x2A => {
                let address = self.hl();
                self.a = bus.read(address);
                self.set_hl(address.wrapping_add(1));
                8
            }

            // LD (HL-),A
            0x32 => {
                let address = self.hl();
                bus.write(address, self.a);
                self.set_hl(address.wrapping_sub(1));
                8
            }

            // LD A,(HL-)
            0x3A => {
                let address = self.hl();
                self.a = bus.read(address);
                self.set_hl(address.wrapping_sub(1));
                8
            }

            // DEC HL
            0x2B => {
                self.set_hl(self.hl().wrapping_sub(1));
                8
            }

            // LD SP,d16
            0x31 => {
                self.sp = self.read_imm16(bus);
                12
            }

            // ADD HL,BC/DE/HL/SP
            0x09 | 0x19 | 0x29 | 0x39 => {
                let index = (opcode >> 4) & 3;
                let hl_before = self.hl();
                let value = self.read_rr(index);
                let result = hl_before.wrapping_add(value);
                let carry16 = (hl_before as u32 + value as u32) > 0xFFFF;
                let half_carry = ((hl_before & 0x0FFF) + (value & 0x0FFF)) > 0x0FFF;
                let zero = self.flag_z();

                self.set_flags(zero, false, half_carry, carry16);
                self.set_hl(result);
                8
            }

            // LD (a16),SP
            0x08 => {
                let address = self.read_imm16(bus);

                bus.write(address, self.sp as u8);
                bus.write(address.wrapping_add(1), (self.sp >> 8) as u8);

                20
            }

            // ADD SP,e8
            0xE8 => {
                let offset = self.read_imm8(bus);
                let signed = offset as i8;

                let sp = self.sp;
                let result = sp.wrapping_add(signed as i16 as u16);

                let half_carry = ((sp & 0x000F) + (offset as u16 & 0x000F)) > 0x000F;

                let carry = ((sp & 0x00FF) + (offset as u16 & 0x00FF)) > 0x00FF;

                self.sp = result;

                self.set_flags(false, false, half_carry, carry);

                16
            }

            // LD HL,SP+e8
            0xF8 => {
                let offset = self.read_imm8(bus);
                let signed = offset as i8;

                let sp = self.sp;
                let result = sp.wrapping_add(signed as i16 as u16);

                let half_carry = ((sp & 0x000F) + (offset as u16 & 0x000F)) > 0x000F;

                let carry = ((sp & 0x00FF) + (offset as u16 & 0x00FF)) > 0x00FF;

                self.set_hl(result);

                self.set_flags(false, false, half_carry, carry);

                12
            }

            // LD SP,HL
            0xF9 => {
                self.sp = self.hl();
                8
            }

            // CPL
            0x2F => {
                self.a = !self.a;
                self.f |= 0x60;
                4
            }

            // SCF
            0x37 => {
                let z = self.flag_z();

                self.set_flags(z, false, false, true);

                4
            }

            // CCF
            0x3F => {
                let z = self.flag_z();
                let c = !self.flag_c();

                self.set_flags(z, false, false, c);

                4
            }

            // DAA
            0x27 => {
                let mut a = self.a;
                let old_n = self.f & 0x40 != 0;
                let old_h = self.f & 0x20 != 0;
                let mut carry = self.f & 0x10 != 0;

                if !old_n {
                    if carry || a > 0x99 {
                        a = a.wrapping_add(0x60);
                        carry = true;
                    }

                    if old_h || (a & 0x0F) > 0x09 {
                        a = a.wrapping_add(0x06);
                    }
                } else {
                    if carry {
                        a = a.wrapping_sub(0x60);
                    }

                    if old_h {
                        a = a.wrapping_sub(0x06);
                    }
                }

                self.a = a;

                self.set_flags(a == 0, old_n, false, carry);

                4
            }

            // JR r8
            0x18 => {
                let offset = self.read_imm8(bus) as i8;
                self.pc = (self.pc as i32 + offset as i32) as u16;
                self.maybe_trace_pc(self.pc, "JR", bus, self.prev_pc, self.prev_opcode, self.prev_sp);

                12
            }

            // JR NZ/Z/NC/C
            0x20 | 0x28 | 0x30 | 0x38 => {
                let cc = (opcode >> 3) & 0x03;
                let raw_offset = self.read_imm8(bus);
                let offset = raw_offset as i8;
                let condition = self.condition(cc);
                let target = (self.pc as i32 + offset as i32) as u16;

                if condition {
                    self.pc = target;
                    self.maybe_trace_pc(self.pc, "JR_CC", bus, self.prev_pc, self.prev_opcode, self.prev_sp);
                    12
                } else {
                    8
                }
            }

            // JP NZ/Z/NC/C
            0xC2 | 0xCA | 0xD2 | 0xDA => {
                let cc = (opcode >> 3) & 0x03;
                let address = self.read_imm16(bus);

                if self.condition(cc) {
                    self.pc = address;
                    self.maybe_trace_pc(self.pc, "JP_CC", bus, self.prev_pc, self.prev_opcode, self.prev_sp);
                    16
                } else {
                    12
                }
            }

            // JP a16
            0xC3 => {
                let address = self.read_imm16(bus);
                let prev_pc = prev_pc;
                let prev_opcode = opcode;
                let sp_before = sp_before;
                self.pc = address;
                self.maybe_trace_pc(self.pc, "JP", bus, prev_pc, prev_opcode, sp_before);
                16
            }

            0xE9 => {
                self.pc = self.hl();
                4
            }

            // CALL NZ/Z/NC/C
            0xC4 | 0xCC | 0xD4 | 0xDC => {
                let cc = (opcode >> 3) & 0x03;
                let address = self.read_imm16(bus);

                if self.condition(cc) {
                    self.push(bus, self.pc);
                    self.pc = address;
                    24
                } else {
                    12
                }
            }

            // CALL a16
            0xCD => {
                let lo = bus.read(self.pc);
                let hi = bus.read(self.pc.wrapping_add(1));

                let target = u16::from_le_bytes([lo, hi]);
                let return_addr = self.pc.wrapping_add(2);

                self.push(bus, return_addr);
                let prev_pc = prev_pc;
                let prev_opcode = opcode;
                let sp_before = sp_before;
                self.pc = target;
                self.maybe_trace_pc(self.pc, "CALL", bus, prev_pc, prev_opcode, sp_before);

                24
            }
            // RET NZ/Z/NC/C
            0xC0 | 0xC8 | 0xD0 | 0xD8 => {
                let cc = (opcode >> 3) & 0x03;

                if self.condition(cc) {
                    let prev_pc = prev_pc;
                    let prev_opcode = opcode;
                    let sp_before = sp_before;
                    let ret_addr = self.pop(bus);
                    self.pc = ret_addr;
                    self.maybe_trace_pc(self.pc, "RET", bus, prev_pc, prev_opcode, sp_before);
                    20
                } else {
                    8
                }
            }

            // RET
            0xC9 => {
                let prev_pc = prev_pc;
                let prev_opcode = opcode;
                let sp_before = sp_before;
                let ret_addr = self.pop(bus);
                self.pc = ret_addr;
                self.maybe_trace_pc(self.pc, "RET", bus, prev_pc, prev_opcode, sp_before);
                16
            }

            // RETI
            0xD9 => {
                let prev_pc = prev_pc;
                let prev_opcode = opcode;
                let sp_before = sp_before;
                let return_address = self.pop(bus);

                self.pc = return_address;
                self.ime = true;
                self.maybe_trace_pc(self.pc, "RETI", bus, prev_pc, prev_opcode, sp_before);

                16
            }

            // PUSH
            0xC5 => {
                let value = self.bc();
                self.push(bus, value);
                16
            }

            0xD5 => {
                let value = self.de();
                self.push(bus, value);
                16
            }

            0xE5 => {
                self.push(bus, self.hl());
                16
            }

            0xF5 => {
                let value = self.af();
                self.push(bus, value);
                16
            }

            // POP
            0xC1 => {
                let value = self.pop(bus);
                self.set_bc(value);
                12
            }

            0xD1 => {
                let value = self.pop(bus);
                self.set_de(value);
                12
            }

            0xE1 => {
                let value = self.pop(bus);
                self.set_hl(value);
                12
            }

            0xF1 => {
                let value = self.pop(bus);
                self.a = (value >> 8) as u8;
                self.f = (value as u8) & 0xF0;
                12
            }

            // RST 00H / 08H / 10H / 18H / 20H / 28H / 30H
            0xC7 | 0xCF | 0xD7 | 0xDF | 0xE7 | 0xEF | 0xF7 => {
                let return_address = self.pc;

                self.push(bus, return_address);

                self.pc = (opcode & 0x38) as u16;

                16
            }

            // RST 38H
            0xFF => {
                self.push(bus, self.pc);
                self.pc = 0x0038;
                16
            }

            // ALU d8
            0xC6 | 0xCE | 0xD6 | 0xDE | 0xE6 | 0xEE | 0xF6 | 0xFE => {
                let op = (opcode >> 3) & 7;
                let value = self.read_imm8(bus);

                self.alu_a(op, value);

                8
            }

            // HALT
            0x76 => {
                self.halted = true;
                4
            }

            // LD (HL),A
            0x77 => {
                bus.write(self.hl(), self.a);
                8
            }

            // LD A,(HL)
            0x7E => {
                self.a = bus.read(self.hl());
                8
            }

            // LDH (a8),A
            0xE0 => {
                let offset = self.read_imm8(bus);
                let address = 0xFF00 | offset as u16;

                bus.write(address, self.a);

                12
            }

            // LDH A,(a8)
            0xF0 => {
                let offset = self.read_imm8(bus);
                let address = 0xFF00 | offset as u16;
                let value = bus.read(address);

                self.a = value;

                12
            }

            // LD (C),A
            0xE2 => {
                let address = 0xFF00 | self.c as u16;

                bus.write(address, self.a);

                8
            }

            // LD A,(C)
            0xF2 => {
                let address = 0xFF00 | self.c as u16;

                self.a = bus.read(address);

                8
            }

            // LD (a16),A
            0xEA => {
                let address = self.read_imm16(bus);

                bus.write(address, self.a);

                16
            }

            // LD A,(a16)
            0xFA => {
                let address = self.read_imm16(bus);
                let value = bus.read(address);

                self.a = value;

                16
            }

            // DI
            0xF3 => {
                self.ime = false;
                self.ime_pending = false;
                4
            }

            // EI — IME włącza się po następnej instrukcji
            0xFB => {
                self.ime_pending = true;
                4
            }

            // XOR A
            0xAF => {
                self.a ^= self.a;

                self.set_flags(true, false, false, false);

                4
            }

        // ... (handling of opcodes continues)
            0xD3 | 0xDB | 0xDD | 0xE3 | 0xE4 | 0xEB | 0xEC | 0xED | 0xF4 | 0xFC | 0xFD => {
                // Before panicking, if this instruction caused PC to land on interesting target,
                // emit a one-time diagnostic.
                // instruction address is self.pc - 1
                if !self.debug_pc_traced && self.pc.wrapping_sub(1) == 0x5C05 {
                    self.debug_pc_traced = true;
                    // Determine operation type from opcode
                    let op_type = match opcode {
                        0xC3 | 0xC2 | 0xCA | 0xD2 | 0xDA => "JP",
                        0x18 | 0x20 | 0x28 | 0x30 | 0x38 => "JR",
                        0xCD | 0xC4 | 0xCC | 0xD4 | 0xDC => "CALL",
                        0xC9 | 0xC0 | 0xC8 | 0xD0 | 0xD8 => "RET",
                        0xD9 => "RETI",
                        0xC7 | 0xCF | 0xD7 | 0xDF | 0xE7 | 0xEF | 0xF7 | 0xFF => "RST",
                        _ => "OTHER",
                    };

                    // Stack bytes to inspect
                    let (stack_addr, b0, b1) = if op_type == "CALL" {
                        let addr = self.sp; // after CALL push, SP points to pushed low
                        (addr, bus.read(addr), bus.read(addr.wrapping_add(1)))
                    } else {
                        (sp_before, bus.read(sp_before), bus.read(sp_before.wrapping_add(1)))
                    };

                    // MBC1 mapping info
                    let low = bus.mbc1.rom_bank_low as usize;
                    let high = bus.mbc1.rom_bank_high as usize;
                    let banking = bus.mbc1.banking_mode as usize;
                    let bank_4000 = if banking == 0 { (high << 5) | low } else { low };

                    // bytes around previous PC
                    let mut surround = Vec::new();
                    for i in 0..16u16 {
                        let addr = prev_pc.wrapping_sub(8).wrapping_add(i);
                        surround.push((addr, bus.read(addr)));
                    }

                    eprintln!("--- PC TARGET TRACE (one-shot) ---");
                    eprintln!("prev_pc=${:04X}", prev_pc);
                    eprintln!("prev_opcode={:02X}", opcode);
                    eprintln!("op_type={}", op_type);
                    eprintln!("new_pc=${:04X}", self.pc);
                    eprintln!("sp_before=${:04X}", sp_before);
                    eprintln!("sp_after=${:04X}", self.sp);
                    eprintln!("stack_addr=${:04X} stack_bytes={:02X} {:02X}", stack_addr, b0, b1);
                    eprintln!("IME={} halted={}", self.ime, self.halted);
                    eprintln!("mbc1.low={} high={} banking={}", low, high, banking);
                    eprintln!("bank_4000={}", bank_4000);
                    eprintln!("Surrounding bytes around prev_pc:");
                    for (a, v) in surround {
                        eprint!("{:04X}:{:02X} ", a, v);
                    }
                    eprintln!("\n--- end trace ---");
                }

                panic!(
                    "Nieprawidłowa instrukcja {:02X} pod adresem ${:04X}",
                    opcode,
                    self.pc.wrapping_sub(1)
                );
            }

            _ => {
                panic!(
                    "Niezaimplementowana instrukcja {:02X} pod adresem ${:04X}",
                    opcode,
                    self.pc.wrapping_sub(1)
                );
            }
        }
    }
}
