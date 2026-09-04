use crate::game::GameMemory;
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
    pub halt_bug: bool,

    pub opcode_counts: [u64; 256],
}

impl Cpu {
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
            halt_bug: false,
            opcode_counts: [0; 256],
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
        self.halt_bug = false;
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

    fn read_imm8(&mut self, memory: &mut GameMemory) -> u8 {
        let value = memory.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        value
    }

    fn read_imm16(&mut self, memory: &mut GameMemory) -> u16 {
        let low = self.read_imm8(memory);
        let high = self.read_imm8(memory);

        u16::from_le_bytes([low, high])
    }

    fn push(&mut self, memory: &mut GameMemory, value: u16) {
        let _old_sp = self.sp;

        self.sp = self.sp.wrapping_sub(1);
        let high_addr = self.sp;
        memory.write(high_addr, (value >> 8) as u8);

        self.sp = self.sp.wrapping_sub(1);
        let low_addr = self.sp;
        memory.write(low_addr, value as u8);
    }

    fn pop(&mut self, memory: &mut GameMemory) -> u16 {
        let _old_sp = self.sp;

        let _low_addr = self.sp;
        let low = memory.read(self.sp);
        self.sp = self.sp.wrapping_add(1);

        let _high_addr = self.sp;
        let high = memory.read(self.sp);
        self.sp = self.sp.wrapping_add(1);

        let value = u16::from_le_bytes([low, high]);

        value
    }

    fn read_r8(&self, memory: &mut GameMemory, index: u8) -> u8 {
        match index {
            0 => self.b,
            1 => self.c,
            2 => self.d,
            3 => self.e,
            4 => self.h,
            5 => self.l,
            6 => memory.read(self.hl()),
            7 => self.a,
            _ => unreachable!(),
        }
    }

    fn write_r8(&mut self, memory: &mut GameMemory, index: u8, value: u8) {
        match index {
            0 => self.b = value,
            1 => self.c = value,
            2 => self.d = value,
            3 => self.e = value,
            4 => self.h = value,
            5 => self.l = value,
            6 => {
                let address = self.hl();
                memory.write(address, value);
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

    fn inc8(&mut self, memory: &mut GameMemory, index: u8) {
    let old = self.read_r8(memory, index);
    let result = old.wrapping_add(1);

    // INC nie zmienia Carry.
    let carry = self.flag_c();

    self.write_r8(memory, index, result);

    self.set_flags(
        result == 0,
        false,
        (old & 0x0F) == 0x0F,
        carry,
    );
}

fn dec8(&mut self, memory: &mut GameMemory, index: u8) {
    let old = self.read_r8(memory, index);
    let result = old.wrapping_sub(1);

    // DEC nie zmienia Carry.
    let carry = self.flag_c();

    self.write_r8(memory, index, result);

    self.set_flags(
        result == 0,
        true,
        (old & 0x0F) == 0,
        carry,
    );
}

    fn execute_cb(&mut self, memory: &mut GameMemory, opcode: u8) -> u32 {
        let x = opcode >> 6;
        let y = (opcode >> 3) & 0x07;
        let z = opcode & 0x07;

        match x {
            // Rotate / shift / SWAP
            0 => {
                let old = self.read_r8(memory, z);

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

                self.write_r8(memory, z, result);

                self.set_flags(result == 0, false, false, carry);

                if z == 6 { 16 } else { 8 }
            }

            // BIT
            1 => {
                let value = self.read_r8(memory, z);
                let bit_set = value & (1 << y) != 0;
                let carry = self.flag_c();

                self.set_flags(!bit_set, false, true, carry);

                if z == 6 { 12 } else { 8 }
            }

            // RES
            2 => {
                let value = self.read_r8(memory, z);
                let result = value & !(1 << y);

                self.write_r8(memory, z, result);

                if z == 6 { 16 } else { 8 }
            }

            // SET
            3 => {
                let value = self.read_r8(memory, z);
                let result = value | (1 << y);

                self.write_r8(memory, z, result);

                if z == 6 { 16 } else { 8 }
            }

            _ => unreachable!(),
        }
    }

    fn execute_interrupt(&mut self, memory: &mut GameMemory, pending: u8) -> u32 {
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

        let if_reg = memory.read(0xFF0F);
        let _ie = memory.read(0xFFFF);
        self.ime = false;
        self.halted = false;

        let new_if = if_reg & !interrupt_bit;
        memory.write(0xFF0F, new_if);

        self.push(memory, self.pc);
        self.pc = vector;
        20
    }

    pub fn step(&mut self, memory: &mut GameMemory) -> u32 {
        let enable_ime = self.ime_pending;
        self.ime_pending = false;

        let if_reg = memory.read(0xFF0F);
        let ie = memory.read(0xFFFF);
        let pending = if_reg & ie & 0x1F;
        if self.halted {
            if pending != 0 {
                self.halted = false;

                if self.ime {
                    return self.execute_interrupt(memory, pending);
                }
            }

            if enable_ime {
                self.ime = true;
            }
            return 4;
        }

        if self.ime && pending != 0 {
            return self.execute_interrupt(memory, pending);
        }

        let opcode = memory.read(self.pc);

        if self.halt_bug {
            self.halt_bug = false;
        } else {
            self.pc = self.pc.wrapping_add(1);
        }

        let cycles = self.execute(memory, opcode);

        if self.pc == 0x0100 {
            println!("CPU WRÓCIŁ DO 0100");
        }

        if enable_ime && opcode != 0xF3 {
            self.ime = true;
        }

        cycles
    }

    fn execute(&mut self, memory: &mut GameMemory, opcode: u8) -> u32 {
        self.opcode_counts[opcode as usize] += 1;

        // HALT
        if opcode == 0x76 {
            let if_reg = memory.read(0xFF0F);
            let ie = memory.read(0xFFFF);
            let pending = if_reg & ie & 0x1F;

            if !self.ime && pending != 0 {
                self.halt_bug = true;
                self.halted = false;
            } else {
                self.halted = true;
            }

            return 4;
        }

        if opcode == 0xCB {
            let cb_opcode = self.read_imm8(memory);
            return self.execute_cb(memory, cb_opcode);
        }

        // -------------------------------------------------
        // LD r8,r8
        // -------------------------------------------------

        if (0x40..=0x7F).contains(&opcode) {
            let dst = (opcode >> 3) & 7;
            let src = opcode & 7;
            let value = self.read_r8(memory, src);

            self.write_r8(memory, dst, value);

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
            let value = self.read_r8(memory, src);

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
            let value = self.read_imm8(memory);

            self.write_r8(memory, dst, value);

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

            self.inc8(memory, index);

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

            self.dec8(memory, index);

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
                let _ = self.read_imm8(memory);
                self.halted = true;
                4
            }

            // LD BC,d16
            0x01 => {
                let value = self.read_imm16(memory);
                self.set_bc(value);
                12
            }

            // LD (BC),A
            0x02 => {
                memory.write(self.bc(), self.a);
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
                self.a = memory.read(self.bc());
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
                let value = self.read_imm16(memory);
                self.set_de(value);
                12
            }

            // LD (DE),A
            0x12 => {
                memory.write(self.de(), self.a);
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
                let value = memory.read(address);
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
                let value = self.read_imm16(memory);
                self.set_hl(value);
                12
            }

            // LD (HL+),A
            0x22 => {
                let address = self.hl();

                memory.write(address, self.a);
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
                self.a = memory.read(address);
                self.set_hl(address.wrapping_add(1));
                8
            }

            // LD (HL-),A
            0x32 => {
                let address = self.hl();
                memory.write(address, self.a);
                self.set_hl(address.wrapping_sub(1));
                8
            }

            // LD A,(HL-)
            0x3A => {
                let address = self.hl();
                self.a = memory.read(address);
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
                self.sp = self.read_imm16(memory);
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
                let address = self.read_imm16(memory);

                memory.write(address, self.sp as u8);
                memory.write(address.wrapping_add(1), (self.sp >> 8) as u8);

                20
            }

            // ADD SP,e8
            0xE8 => {
                let offset = self.read_imm8(memory);
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
                let offset = self.read_imm8(memory);
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
                let offset = self.read_imm8(memory) as i8;
                self.pc = (self.pc as i32 + offset as i32) as u16;

                12
            }

            // JR NZ/Z/NC/C
            0x20 | 0x28 | 0x30 | 0x38 => {
                let cc = (opcode >> 3) & 0x03;
                let raw_offset = self.read_imm8(memory);
                let offset = raw_offset as i8;
                let condition = self.condition(cc);
                let target = (self.pc as i32 + offset as i32) as u16;

                if condition {
                    self.pc = target;
                    12
                } else {
                    8
                }
            }

            // JP NZ/Z/NC/C
            0xC2 | 0xCA | 0xD2 | 0xDA => {
                let cc = (opcode >> 3) & 0x03;
                let address = self.read_imm16(memory);

                if self.condition(cc) {
                    self.pc = address;
                    16
                } else {
                    12
                }
            }

            // JP a16
            0xC3 => {
                let address = self.read_imm16(memory);

                self.pc = address;
                16
            }

            0xE9 => {
                self.pc = self.hl();
                4
            }

            // CALL NZ/Z/NC/C
            0xC4 | 0xCC | 0xD4 | 0xDC => {
                let cc = (opcode >> 3) & 0x03;
                let address = self.read_imm16(memory);

                if self.condition(cc) {
                    self.push(memory, self.pc);
                    self.pc = address;
                    24
                } else {
                    12
                }
            }

            // CALL a16
            0xCD => {
                let lo = memory.read(self.pc);
                let hi = memory.read(self.pc.wrapping_add(1));

                let target = u16::from_le_bytes([lo, hi]);
                let return_addr = self.pc.wrapping_add(2);

                self.push(memory, return_addr);

                self.pc = target;

                24
            }
            // RET NZ/Z/NC/C
            0xC0 | 0xC8 | 0xD0 | 0xD8 => {
                let cc = (opcode >> 3) & 0x03;

                if self.condition(cc) {
                    self.pc = self.pop(memory);
                    20
                } else {
                    8
                }
            }

            // RET
            0xC9 => {
                self.pc = self.pop(memory);
                16
            }

            // RETI
            0xD9 => {
                let return_address = self.pop(memory);

                self.pc = return_address;
                self.ime = true;

                16
            }

            // PUSH
            0xC5 => {
                let value = self.bc();
                self.push(memory, value);
                16
            }

            0xD5 => {
                let value = self.de();
                self.push(memory, value);
                16
            }

            0xE5 => {
                self.push(memory, self.hl());
                16
            }

            0xF5 => {
                let value = self.af();
                self.push(memory, value);
                16
            }

            // POP
            0xC1 => {
                let value = self.pop(memory);
                self.set_bc(value);
                12
            }

            0xD1 => {
                let value = self.pop(memory);
                self.set_de(value);
                12
            }

            0xE1 => {
                let value = self.pop(memory);
                self.set_hl(value);
                12
            }

            0xF1 => {
                let value = self.pop(memory);
                self.a = (value >> 8) as u8;
                self.f = (value as u8) & 0xF0;
                12
            }

            // RST 00H / 08H / 10H / 18H / 20H / 28H / 30H
            0xC7 | 0xCF | 0xD7 | 0xDF | 0xE7 | 0xEF | 0xF7 => {
                let return_address = self.pc;

                self.push(memory, return_address);

                self.pc = (opcode & 0x38) as u16;

                16
            }

            // RST 38H
            0xFF => {
                self.push(memory, self.pc);
                self.pc = 0x0038;
                16
            }

            // ALU d8
            0xC6 | 0xCE | 0xD6 | 0xDE | 0xE6 | 0xEE | 0xF6 | 0xFE => {
                let op = (opcode >> 3) & 7;
                let value = self.read_imm8(memory);

                self.alu_a(op, value);

                8
            }

            // LD (HL),A
            0x77 => {
                memory.write(self.hl(), self.a);
                8
            }

            // LD A,(HL)
            0x7E => {
                self.a = memory.read(self.hl());
                8
            }

            // LDH (a8),A
            0xE0 => {
                let offset = self.read_imm8(memory);
                let address = 0xFF00 | offset as u16;

                memory.write(address, self.a);

                12
            }

            // LDH A,(a8)
            0xF0 => {
                let offset = self.read_imm8(memory);
                let address = 0xFF00 | offset as u16;
                let value = memory.read(address);

                self.a = value;

                12
            }

            // LD (C),A
            0xE2 => {
                let address = 0xFF00 | self.c as u16;

                memory.write(address, self.a);

                8
            }

            // LD A,(C)
            0xF2 => {
                let address = 0xFF00 | self.c as u16;

                self.a = memory.read(address);

                8
            }

            // LD (a16),A
            0xEA => {
                let address = self.read_imm16(memory);

                memory.write(address, self.a);

                16
            }

            // LD A,(a16)
            0xFA => {
                let address = self.read_imm16(memory);
                let value = memory.read(address);

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

            // Invalid opcodes
            0xD3 | 0xDB | 0xDD | 0xE3 | 0xE4 | 0xEB | 0xEC | 0xED | 0xF4 | 0xFC | 0xFD => {
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
#[cfg(test)]
mod tests {
    use super::*;

    fn cpu() -> Cpu {
        Cpu::new()
    }

    fn flags(cpu: &Cpu) -> (bool, bool, bool, bool) {
        (
            cpu.f & 0x80 != 0, // Z
            cpu.f & 0x40 != 0, // N
            cpu.f & 0x20 != 0, // H
            cpu.f & 0x10 != 0, // C
        )
    }

    #[test]
    fn add_zero_sets_z() {
        let mut cpu = cpu();

        cpu.a = 0x00;
        cpu.f = 0x00;
        cpu.alu_a(0, 0x00);

        assert_eq!(cpu.a, 0x00);
        assert_eq!(flags(&cpu), (true, false, false, false));
    }

    #[test]
    fn add_simple() {
        let mut cpu = cpu();

        cpu.a = 0x01;
        cpu.alu_a(0, 0x01);

        assert_eq!(cpu.a, 0x02);
        assert_eq!(flags(&cpu), (false, false, false, false));
    }

    #[test]
    fn add_half_carry() {
        let mut cpu = cpu();

        cpu.a = 0x0F;
        cpu.alu_a(0, 0x01);

        assert_eq!(cpu.a, 0x10);
        assert_eq!(flags(&cpu), (false, false, true, false));
    }

    #[test]
    fn add_half_carry_another_case() {
        let mut cpu = cpu();

        cpu.a = 0x07;
        cpu.alu_a(0, 0x09);

        assert_eq!(cpu.a, 0x10);
        assert_eq!(flags(&cpu), (false, false, true, false));
    }

    #[test]
    fn add_full_carry_and_zero() {
        let mut cpu = cpu();

        cpu.a = 0xFF;
        cpu.alu_a(0, 0x01);

        assert_eq!(cpu.a, 0x00);
        assert_eq!(flags(&cpu), (true, false, true, true));
    }

    #[test]
    fn add_carry_without_half_carry() {
        let mut cpu = cpu();

        cpu.a = 0x80;
        cpu.alu_a(0, 0x80);

        assert_eq!(cpu.a, 0x00);
        assert_eq!(flags(&cpu), (true, false, false, true));
    }

    #[test]
    fn add_upper_nibble_carry_only() {
        let mut cpu = cpu();

        cpu.a = 0xF0;
        cpu.alu_a(0, 0x10);

        assert_eq!(cpu.a, 0x00);
        assert_eq!(flags(&cpu), (true, false, false, true));
    }
    #[test]
fn adc_without_initial_carry() {
    let mut cpu = cpu();

    cpu.a = 0x01;
    cpu.f = 0x00; // C=0

    cpu.alu_a(1, 0x01);

    assert_eq!(cpu.a, 0x02);
    assert_eq!(flags(&cpu), (false, false, false, false));
}

#[test]
fn adc_with_initial_carry() {
    let mut cpu = cpu();

    cpu.a = 0x01;
    cpu.f = 0x10; // C=1

    cpu.alu_a(1, 0x01);

    assert_eq!(cpu.a, 0x03);
    assert_eq!(flags(&cpu), (false, false, false, false));
}

#[test]
fn adc_half_carry_from_carry_flag() {
    let mut cpu = cpu();

    cpu.a = 0x0F;
    cpu.f = 0x10;

    cpu.alu_a(1, 0x00);

    assert_eq!(cpu.a, 0x10);
    assert_eq!(flags(&cpu), (false, false, true, false));
}

#[test]
fn adc_zero_and_full_carry() {
    let mut cpu = cpu();

    cpu.a = 0xFF;
    cpu.f = 0x10;

    cpu.alu_a(1, 0x00);

    assert_eq!(cpu.a, 0x00);
    assert_eq!(flags(&cpu), (true, false, true, true));
}

#[test]
fn adc_half_and_full_carry() {
    let mut cpu = cpu();

    cpu.a = 0xFF;
    cpu.f = 0x00;

    cpu.alu_a(1, 0x01);

    assert_eq!(cpu.a, 0x00);
    assert_eq!(flags(&cpu), (true, false, true, true));
}

#[test]
fn adc_carry_without_half_carry() {
    let mut cpu = cpu();

    cpu.a = 0x80;
    cpu.f = 0x10;

    cpu.alu_a(1, 0x7F);

    assert_eq!(cpu.a, 0x00);
    assert_eq!(flags(&cpu), (true, false, true, true));
}

#[test]
fn adc_half_carry_only() {
    let mut cpu = cpu();

    cpu.a = 0x08;
    cpu.f = 0x10;

    cpu.alu_a(1, 0x07);

    assert_eq!(cpu.a, 0x10);
    assert_eq!(flags(&cpu), (false, false, true, false));
}

#[test]
fn adc_preserves_no_flags_when_not_needed() {
    let mut cpu = cpu();

    cpu.a = 0x22;
    cpu.f = 0x10;

    cpu.alu_a(1, 0x11);

    assert_eq!(cpu.a, 0x34);
    assert_eq!(flags(&cpu), (false, false, false, false));
}
#[test]
fn sub_simple() {
    let mut cpu = cpu();

    cpu.a = 0x02;
    cpu.alu_a(2, 0x01);

    assert_eq!(cpu.a, 0x01);
    assert_eq!(flags(&cpu), (false, true, false, false));
}

#[test]
fn sub_zero() {
    let mut cpu = cpu();

    cpu.a = 0x01;
    cpu.alu_a(2, 0x01);

    assert_eq!(cpu.a, 0x00);
    assert_eq!(flags(&cpu), (true, true, false, false));
}

#[test]
fn sub_half_borrow() {
    let mut cpu = cpu();

    cpu.a = 0x10;
    cpu.alu_a(2, 0x01);

    assert_eq!(cpu.a, 0x0F);
    assert_eq!(flags(&cpu), (false, true, true, false));
}

#[test]
fn sub_half_borrow_lower_nibble() {
    let mut cpu = cpu();

    cpu.a = 0x20;
    cpu.alu_a(2, 0x11);

    assert_eq!(cpu.a, 0x0F);
    assert_eq!(flags(&cpu), (false, true, true, false));
}

#[test]
fn sub_full_borrow() {
    let mut cpu = cpu();

    cpu.a = 0x00;
    cpu.alu_a(2, 0x01);

    assert_eq!(cpu.a, 0xFF);
    assert_eq!(flags(&cpu), (false, true, true, true));
}

#[test]
fn sub_full_borrow_without_half_borrow() {
    let mut cpu = cpu();

    cpu.a = 0xF0;
    cpu.alu_a(2, 0xF1);

    assert_eq!(cpu.a, 0xFF);
    assert_eq!(flags(&cpu), (false, true, true, true));
}

#[test]
fn sub_no_half_no_carry() {
    let mut cpu = cpu();

    cpu.a = 0x7F;
    cpu.alu_a(2, 0x01);

    assert_eq!(cpu.a, 0x7E);
    assert_eq!(flags(&cpu), (false, true, false, false));
}

#[test]
fn sub_half_borrow_boundary() {
    let mut cpu = cpu();

    cpu.a = 0x11;
    cpu.alu_a(2, 0x02);

    assert_eq!(cpu.a, 0x0F);
    assert_eq!(flags(&cpu), (false, true, true, false));
}

#[test]
fn sub_zero_from_ff() {
    let mut cpu = cpu();

    cpu.a = 0xFF;
    cpu.alu_a(2, 0xFF);

    assert_eq!(cpu.a, 0x00);
    assert_eq!(flags(&cpu), (true, true, false, false));
}
#[test]
fn sbc_without_carry() {
    let mut cpu = cpu();

    cpu.a = 0x02;
    cpu.f = 0x00;

    cpu.alu_a(3, 0x01);

    assert_eq!(cpu.a, 0x01);
    assert_eq!(flags(&cpu), (false, true, false, false));
}

#[test]
fn sbc_with_input_carry() {
    let mut cpu = cpu();

    cpu.a = 0x02;
    cpu.f = 0x10;

    cpu.alu_a(3, 0x01);

    assert_eq!(cpu.a, 0x00);
    assert_eq!(flags(&cpu), (true, true, false, false));
}

#[test]
fn sbc_half_borrow_from_carry() {
    let mut cpu = cpu();

    cpu.a = 0x10;
    cpu.f = 0x10;

    cpu.alu_a(3, 0x00);

    assert_eq!(cpu.a, 0x0F);
    assert_eq!(flags(&cpu), (false, true, true, false));
}

#[test]
fn sbc_full_borrow_from_carry() {
    let mut cpu = cpu();

    cpu.a = 0x00;
    cpu.f = 0x10;

    cpu.alu_a(3, 0x00);

    assert_eq!(cpu.a, 0xFF);
    assert_eq!(flags(&cpu), (false, true, true, true));
}

#[test]
fn sbc_full_borrow_operand() {
    let mut cpu = cpu();

    cpu.a = 0x00;
    cpu.f = 0x00;

    cpu.alu_a(3, 0x01);

    assert_eq!(cpu.a, 0xFF);
    assert_eq!(flags(&cpu), (false, true, true, true));
}

#[test]
fn sbc_zero_with_operand_and_carry() {
    let mut cpu = cpu();

    cpu.a = 0x02;
    cpu.f = 0x10;

    cpu.alu_a(3, 0x01);

    assert_eq!(cpu.a, 0x00);
    assert_eq!(flags(&cpu), (true, true, false, false));
}

#[test]
fn sbc_half_borrow_boundary() {
    let mut cpu = cpu();

    cpu.a = 0x11;
    cpu.f = 0x10;

    cpu.alu_a(3, 0x01);

    assert_eq!(cpu.a, 0x0F);
    assert_eq!(flags(&cpu), (false, true, true, false));
}

#[test]
fn sbc_borrow_with_ff() {
    let mut cpu = cpu();

    cpu.a = 0xFF;
    cpu.f = 0x10;

    cpu.alu_a(3, 0xFF);

    assert_eq!(cpu.a, 0xFF);
    assert_eq!(flags(&cpu), (false, true, true, true));
}

#[test]
fn sbc_zero_without_borrow() {
    let mut cpu = cpu();

    cpu.a = 0xFF;
    cpu.f = 0x00;

    cpu.alu_a(3, 0xFF);

    assert_eq!(cpu.a, 0x00);
    assert_eq!(flags(&cpu), (true, true, false, false));
}

#[test]
fn sbc_carry_and_half_carry_together() {
    let mut cpu = cpu();

    cpu.a = 0x80;
    cpu.f = 0x10;

    cpu.alu_a(3, 0x7F);

    assert_eq!(cpu.a, 0x00);
    assert_eq!(flags(&cpu), (true, true, true, false));
}

#[test]
fn sbc_result_fe() {
    let mut cpu = cpu();

    cpu.a = 0x00;
    cpu.f = 0x10;

    cpu.alu_a(3, 0x01);

    assert_eq!(cpu.a, 0xFE);
    assert_eq!(flags(&cpu), (false, true, true, true));
}
#[test]
fn cp_equal() {
    let mut cpu = cpu();

    cpu.a = 0x05;
    cpu.alu_a(7, 0x05);

    assert_eq!(cpu.a, 0x05);
    assert_eq!(flags(&cpu), (true, true, false, false));
}

#[test]
fn cp_less_than_operand() {
    let mut cpu = cpu();

    cpu.a = 0x00;
    cpu.alu_a(7, 0x01);

    assert_eq!(cpu.a, 0x00);
    assert_eq!(flags(&cpu), (false, true, true, true));
}

#[test]
fn cp_greater_than_operand() {
    let mut cpu = cpu();

    cpu.a = 0x05;
    cpu.alu_a(7, 0x01);

    assert_eq!(cpu.a, 0x05);
    assert_eq!(flags(&cpu), (false, true, false, false));
}

#[test]
fn cp_half_borrow() {
    let mut cpu = cpu();

    cpu.a = 0x10;
    cpu.alu_a(7, 0x01);

    assert_eq!(cpu.a, 0x10);
    assert_eq!(flags(&cpu), (false, true, true, false));
}

#[test]
fn cp_full_borrow() {
    let mut cpu = cpu();

    cpu.a = 0x00;
    cpu.alu_a(7, 0x01);

    assert_eq!(cpu.a, 0x00);
    assert_eq!(flags(&cpu), (false, true, true, true));
}

#[test]
fn cp_ff_equal_ff() {
    let mut cpu = cpu();

    cpu.a = 0xFF;
    cpu.alu_a(7, 0xFF);

    assert_eq!(cpu.a, 0xFF);
    assert_eq!(flags(&cpu), (true, true, false, false));
}

#[test]
fn cp_does_not_modify_a() {
    let mut cpu = cpu();

    cpu.a = 0x42;
    cpu.alu_a(7, 0x99);

    assert_eq!(cpu.a, 0x42);
}

#[test]
fn cp_clears_old_flags() {
    let mut cpu = cpu();

    cpu.a = 0x20;
    cpu.f = 0xB0; // Z N H C = 1,1,1,0

    cpu.alu_a(7, 0x10);

    assert_eq!(cpu.a, 0x20);
    assert_eq!(flags(&cpu), (false, true, false, false));
}
#[test]
fn push_bc_pop_bc() {
    use crate::game::memory::GameMemory;
    use crate::rom::{Cartridge, Rom};

    let rom = Rom::load("Legend of Zelda, The - Link's Awakening DX (USA, Europe) (Rev 2).gbc")
        .expect("Nie można załadować ROM-u testowego");

    let cartridge = Cartridge::new(rom);
    let mut memory = GameMemory::new(cartridge);

    let mut cpu = Cpu::new();

    // Program testowy w WRAM:
    // C000: PUSH BC
    // C001: POP BC
    memory.write(0xC000, 0xC5);
    memory.write(0xC001, 0xC1);

    cpu.pc = 0xC000;
    cpu.sp = 0xC100;

    cpu.b = 0x12;
    cpu.c = 0x34;

    // PUSH BC
    let cycles = cpu.step(&mut memory);

    assert_eq!(cycles, 16);
    assert_eq!(cpu.pc, 0xC001);
    assert_eq!(cpu.sp, 0xC0FE);

    // Game Boy stack:
    // SP -> low byte
    // SP+1 -> high byte
    assert_eq!(memory.read(0xC0FE), 0x34);
    assert_eq!(memory.read(0xC0FF), 0x12);

    // POP BC
    let cycles = cpu.step(&mut memory);

    assert_eq!(cycles, 12);
    assert_eq!(cpu.pc, 0xC002);
    assert_eq!(cpu.sp, 0xC100);

    assert_eq!(cpu.b, 0x12);
    assert_eq!(cpu.c, 0x34);
}
#[test]
fn push_af_pop_af() {
    use crate::game::memory::GameMemory;
    use crate::rom::{Cartridge, Rom};

    let rom = Rom::load(
        "Legend of Zelda, The - Link's Awakening DX (USA, Europe) (Rev 2).gbc"
    )
    .expect("Nie można załadować ROM-u testowego");

    let cartridge = Cartridge::new(rom);
    let mut memory = GameMemory::new(cartridge);

    let mut cpu = Cpu::new();

    // C000: PUSH AF
    // C001: POP AF
    memory.write(0xC000, 0xF5);
    memory.write(0xC001, 0xF1);

    cpu.pc = 0xC000;
    cpu.sp = 0xC100;

    cpu.a = 0xAB;
    cpu.f = 0xF0;

    // PUSH AF
    let cycles = cpu.step(&mut memory);

    assert_eq!(cycles, 16);
    assert_eq!(cpu.pc, 0xC001);
    assert_eq!(cpu.sp, 0xC0FE);

    assert_eq!(memory.read(0xC0FE), 0xF0);
    assert_eq!(memory.read(0xC0FF), 0xAB);

    // POP AF
    let cycles = cpu.step(&mut memory);

    assert_eq!(cycles, 12);
    assert_eq!(cpu.pc, 0xC002);
    assert_eq!(cpu.sp, 0xC100);

    assert_eq!(cpu.a, 0xAB);
    assert_eq!(cpu.f, 0xF0);
}
#[test]
fn call_ret() {
    use crate::game::memory::GameMemory;
    use crate::rom::{Cartridge, Rom};

    let rom = Rom::load(
        "Legend of Zelda, The - Link's Awakening DX (USA, Europe) (Rev 2).gbc"
    )
    .expect("Nie można załadować ROM-u testowego");

    let cartridge = Cartridge::new(rom);
    let mut memory = GameMemory::new(cartridge);

    let mut cpu = Cpu::new();

    // C000: CALL C010
    // C003: następna instrukcja po CALL
    // C010: RET

    memory.write(0xC000, 0xCD);
    memory.write(0xC001, 0x10);
    memory.write(0xC002, 0xC0);

    memory.write(0xC010, 0xC9);

    cpu.pc = 0xC000;
    cpu.sp = 0xC100;

    // CALL C010
    let cycles = cpu.step(&mut memory);

    assert_eq!(cycles, 24);
    assert_eq!(cpu.pc, 0xC010);
    assert_eq!(cpu.sp, 0xC0FE);

    // Adres powrotu = C003
    // SP -> low byte
    // SP+1 -> high byte
    assert_eq!(memory.read(0xC0FE), 0x03);
    assert_eq!(memory.read(0xC0FF), 0xC0);

    // RET
    let cycles = cpu.step(&mut memory);

    assert_eq!(cycles, 16);
    assert_eq!(cpu.pc, 0xC003);
    assert_eq!(cpu.sp, 0xC100);
}
#[test]
fn conditional_call_ret() {
    use crate::game::memory::GameMemory;
    use crate::rom::{Cartridge, Rom};

    let rom = Rom::load(
        "Legend of Zelda, The - Link's Awakening DX (USA, Europe) (Rev 2).gbc"
    )
    .expect("Nie można załadować ROM-u testowego");

    let cartridge = Cartridge::new(rom);
    let mut memory = GameMemory::new(cartridge);

    let mut cpu = Cpu::new();

    // C000: CALL NZ,C010
    memory.write(0xC000, 0xC4);
    memory.write(0xC001, 0x10);
    memory.write(0xC002, 0xC0);

    // C010: RET NZ
    memory.write(0xC010, 0xC0);

    cpu.pc = 0xC000;
    cpu.sp = 0xC100;

    // Z = 0 -> CALL NZ powinien zostać wykonany.
    cpu.f = 0x00;

    let cycles = cpu.step(&mut memory);

    assert_eq!(cycles, 24);
    assert_eq!(cpu.pc, 0xC010);
    assert_eq!(cpu.sp, 0xC0FE);

    // Na stosie powinien być adres C003.
    assert_eq!(memory.read(0xC0FE), 0x03);
    assert_eq!(memory.read(0xC0FF), 0xC0);

    // RET NZ -> Z nadal 0, więc wykonany.
    let cycles = cpu.step(&mut memory);

    assert_eq!(cycles, 20);
    assert_eq!(cpu.pc, 0xC003);
    assert_eq!(cpu.sp, 0xC100);

    // Teraz sprawdzamy CALL NZ, gdy Z = 1.
    cpu.pc = 0xC000;
    cpu.sp = 0xC100;
    cpu.f = 0x80;

    let cycles = cpu.step(&mut memory);

    // CALL NZ nie powinien zostać wykonany.
    assert_eq!(cycles, 12);
    assert_eq!(cpu.pc, 0xC003);
    assert_eq!(cpu.sp, 0xC100);
}
#[test]
fn conditional_call_all_conditions() {
    use crate::game::memory::GameMemory;
    use crate::rom::{Cartridge, Rom};

    let rom = Rom::load(
        "Legend of Zelda, The - Link's Awakening DX (USA, Europe) (Rev 2).gbc"
    )
    .expect("Nie można załadować ROM-u testowego");

    let cartridge = Cartridge::new(rom);
    let mut memory = GameMemory::new(cartridge);

    let mut cpu = Cpu::new();

    // C000: CALL NZ,C010
    memory.write(0xC000, 0xC4);
    memory.write(0xC001, 0x10);
    memory.write(0xC002, 0xC0);

    // C010: RET
    memory.write(0xC010, 0xC9);

    // --------------------------------------------------
    // NZ: Z = 0 -> wykonany
    // --------------------------------------------------

    cpu.pc = 0xC000;
    cpu.sp = 0xC100;
    cpu.f = 0x00;

    assert_eq!(cpu.step(&mut memory), 24);
    assert_eq!(cpu.pc, 0xC010);
    assert_eq!(cpu.sp, 0xC0FE);

    // --------------------------------------------------
    // NZ: Z = 1 -> niewykonany
    // --------------------------------------------------

    cpu.pc = 0xC000;
    cpu.sp = 0xC100;
    cpu.f = 0x80;

    assert_eq!(cpu.step(&mut memory), 12);
    assert_eq!(cpu.pc, 0xC003);
    assert_eq!(cpu.sp, 0xC100);

    // --------------------------------------------------
    // Z: Z = 1 -> wykonany
    // --------------------------------------------------

    memory.write(0xC000, 0xCC);

    cpu.pc = 0xC000;
    cpu.sp = 0xC100;
    cpu.f = 0x80;

    assert_eq!(cpu.step(&mut memory), 24);
    assert_eq!(cpu.pc, 0xC010);
    assert_eq!(cpu.sp, 0xC0FE);

    // --------------------------------------------------
    // Z: Z = 0 -> niewykonany
    // --------------------------------------------------

    cpu.pc = 0xC000;
    cpu.sp = 0xC100;
    cpu.f = 0x00;

    assert_eq!(cpu.step(&mut memory), 12);
    assert_eq!(cpu.pc, 0xC003);
    assert_eq!(cpu.sp, 0xC100);

    // --------------------------------------------------
    // NC: C = 0 -> wykonany
    // --------------------------------------------------

    memory.write(0xC000, 0xD4);

    cpu.pc = 0xC000;
    cpu.sp = 0xC100;
    cpu.f = 0x00;

    assert_eq!(cpu.step(&mut memory), 24);
    assert_eq!(cpu.pc, 0xC010);
    assert_eq!(cpu.sp, 0xC0FE);

    // --------------------------------------------------
    // NC: C = 1 -> niewykonany
    // --------------------------------------------------

    cpu.pc = 0xC000;
    cpu.sp = 0xC100;
    cpu.f = 0x10;

    assert_eq!(cpu.step(&mut memory), 12);
    assert_eq!(cpu.pc, 0xC003);
    assert_eq!(cpu.sp, 0xC100);

    // --------------------------------------------------
    // C: C = 1 -> wykonany
    // --------------------------------------------------

    memory.write(0xC000, 0xDC);

    cpu.pc = 0xC000;
    cpu.sp = 0xC100;
    cpu.f = 0x10;

    assert_eq!(cpu.step(&mut memory), 24);
    assert_eq!(cpu.pc, 0xC010);
    assert_eq!(cpu.sp, 0xC0FE);

    // --------------------------------------------------
    // C: C = 0 -> niewykonany
    // --------------------------------------------------

    cpu.pc = 0xC000;
    cpu.sp = 0xC100;
    cpu.f = 0x00;

    assert_eq!(cpu.step(&mut memory), 12);
    assert_eq!(cpu.pc, 0xC003);
    assert_eq!(cpu.sp, 0xC100);
}
#[test]
fn rst_all_vectors() {
    use crate::game::memory::GameMemory;
    use crate::rom::{Cartridge, Rom};

    let rom = Rom::load(
        "Legend of Zelda, The - Link's Awakening DX (USA, Europe) (Rev 2).gbc"
    )
    .expect("Nie można załadować ROM-u testowego");

    let cartridge = Cartridge::new(rom);
    let mut memory = GameMemory::new(cartridge);

    let vectors = [
        (0xC7, 0x00),
        (0xCF, 0x08),
        (0xD7, 0x10),
        (0xDF, 0x18),
        (0xE7, 0x20),
        (0xEF, 0x28),
        (0xF7, 0x30),
        (0xFF, 0x38),
    ];

    for &(opcode, vector) in &vectors {
        let mut cpu = Cpu::new();

        // C000: RST xx
        memory.write(0xC000, opcode);

        cpu.pc = 0xC000;
        cpu.sp = 0xC100;

        let cycles = cpu.step(&mut memory);

        assert_eq!(cycles, 16);
        assert_eq!(cpu.pc, vector);
        assert_eq!(cpu.sp, 0xC0FE);

        // RST musi zapisać adres następnej instrukcji: C001
        assert_eq!(memory.read(0xC0FE), 0x01);
        assert_eq!(memory.read(0xC0FF), 0xC0);
    }
}
#[test]
fn halt_stops_cpu() {
    use crate::game::memory::GameMemory;
    use crate::rom::{Cartridge, Rom};

    let rom = Rom::load(
        "Legend of Zelda, The - Link's Awakening DX (USA, Europe) (Rev 2).gbc"
    )
    .expect("Nie można załadować ROM-u testowego");

    let cartridge = Cartridge::new(rom);
    let mut memory = GameMemory::new(cartridge);

    let mut cpu = Cpu::new();

    // C000: HALT
    // C001: NOP
    memory.write(0xC000, 0x76);
    memory.write(0xC001, 0x00);

    cpu.pc = 0xC000;
    memory.write(0xFF0F, 0x00);
memory.write(0xFFFF, 0x00);
    // HALT
    let cycles = cpu.step(&mut memory);

    assert_eq!(cycles, 4);
    assert_eq!(cpu.pc, 0xC001);
    assert!(cpu.halted);

    // CPU jest zatrzymany.
    // Kolejny step nie powinien wykonać NOP-a.
    let cycles = cpu.step(&mut memory);

    assert_eq!(cycles, 4);
    assert_eq!(cpu.pc, 0xC001);
    assert!(cpu.halted);
}
#[test]
fn halt_wakes_on_pending_interrupt() {
    use crate::game::memory::GameMemory;
    use crate::rom::{Cartridge, Rom};

    let rom = Rom::load(
        "Legend of Zelda, The - Link's Awakening DX (USA, Europe) (Rev 2).gbc"
    )
    .expect("Nie można załadować ROM-u testowego");

    let cartridge = Cartridge::new(rom);
    let mut memory = GameMemory::new(cartridge);

    let mut cpu = Cpu::new();

    // C000: HALT
    memory.write(0xC000, 0x76);

    cpu.pc = 0xC000;
    cpu.sp = 0xC100;
    memory.write(0xFF0F, 0x00);
    memory.write(0xFFFF, 0x00);
    // HALT
    let cycles = cpu.step(&mut memory);

    assert_eq!(cycles, 4);
    assert_eq!(cpu.pc, 0xC001);
    assert!(cpu.halted);

    memory.write(0xFF0F, 0x04); // IF: TIMER interrupt requested
    memory.write(0xFFFF, 0x04); // IE: TIMER interrupt enabled
    cpu.ime = false;

    let cycles = cpu.step(&mut memory);

    // CPU powinien wybudzić się z HALT,
    // ale ponieważ IME = false, nie obsługuje jeszcze przerwania.
    assert_eq!(cycles, 4);
    assert!(!cpu.halted);
    assert_eq!(cpu.pc, 0xC001);
}
#[test]
fn interrupt_services_timer_when_ime_enabled() {
    use crate::rom::{Cartridge, Rom};
    let mut cpu = cpu();

    let rom = Rom::load(
        "Legend of Zelda, The - Link's Awakening DX (USA, Europe) (Rev 2).gbc"
    )
    .expect("Nie można załadować ROM-u testowego");

    let cartridge = Cartridge::new(rom);
    let mut memory = GameMemory::new(cartridge);

    cpu.pc = 0xC001;
    cpu.sp = 0xC100;
    cpu.ime = true;
    cpu.halted = false;

    // Timer interrupt:
    // IF bit 2 = request
    // IE bit 2 = enabled
    memory.write(0xFF0F, 0x04);
    memory.write(0xFFFF, 0x04);

    let cycles = cpu.step(&mut memory);

    assert_eq!(cycles, 20);

    // Timer interrupt vector
    assert_eq!(cpu.pc, 0x0050);

    // Interrupt disables IME
    assert!(!cpu.ime);

    // CPU is no longer halted
    assert!(!cpu.halted);

    // TIMER bit in IF cleared
    assert_eq!(memory.read(0xFF0F) & 0x04, 0);

    // SP: C100 -> C0FE
    assert_eq!(cpu.sp, 0xC0FE);

    // Saved PC = C001, little endian
    assert_eq!(memory.read(0xC0FE), 0x01);
    assert_eq!(memory.read(0xC0FF), 0xC0);
}
#[test]
fn interrupt_priority_vblank_over_timer() {
    use crate::game::memory::GameMemory;
    use crate::rom::{Cartridge, Rom};

    let rom = Rom::load(
        "Legend of Zelda, The - Link's Awakening DX (USA, Europe) (Rev 2).gbc"
    )
    .expect("Nie można załadować ROM-u testowego");

    let cartridge = Cartridge::new(rom);
    let mut memory = GameMemory::new(cartridge);

    let mut cpu = Cpu::new();

    cpu.pc = 0xC001;
    cpu.sp = 0xC100;
    cpu.ime = true;
    cpu.halted = false;

    // VBlank + Timer jednocześnie pending
    memory.write(0xFF0F, 0x05);

    // VBlank + Timer jednocześnie enabled
    memory.write(0xFFFF, 0x05);

    let cycles = cpu.step(&mut memory);

    // Obsłużony powinien zostać VBlank, bo ma wyższy priorytet.
    assert_eq!(cycles, 20);
    assert_eq!(cpu.pc, 0x0040);

    // IME zostaje wyłączone.
    assert!(!cpu.ime);

    // VBlank bit 0 został wyczyszczony.
    assert_eq!(memory.read(0xFF0F) & 0x01, 0);

    // Timer bit 2 nadal pozostaje pending.
    assert_eq!(memory.read(0xFF0F) & 0x04, 0x04);

    // PC zapisany na stosie.
    assert_eq!(cpu.sp, 0xC0FE);
    assert_eq!(memory.read(0xC0FE), 0x01);
    assert_eq!(memory.read(0xC0FF), 0xC0);
}
#[test]
fn interrupt_priority_all_vectors() {
    use crate::game::memory::GameMemory;
    use crate::rom::{Cartridge, Rom};

    let rom = Rom::load(
        "Legend of Zelda, The - Link's Awakening DX (USA, Europe) (Rev 2).gbc"
    )
    .expect("Nie można załadować ROM-u testowego");

    let cartridge = Cartridge::new(rom);
    let mut memory = GameMemory::new(cartridge);

    let mut cpu = Cpu::new();

    cpu.pc = 0xC001;
    cpu.sp = 0xC100;
    cpu.ime = true;

    // Wszystkie 5 przerwań jednocześnie pending.
    memory.write(0xFF0F, 0x1F);

    // Wszystkie 5 przerwań włączone.
    memory.write(0xFFFF, 0x1F);

    let cycles = cpu.step(&mut memory);

    // Najwyższy priorytet ma VBlank.
    assert_eq!(cycles, 20);
    assert_eq!(cpu.pc, 0x0040);

    // IME wyłączone po wejściu w interrupt.
    assert!(!cpu.ime);

    // Tylko VBlank został skasowany.
    assert_eq!(memory.read(0xFF0F), 0x1E);

    // Powrót zapisany na stosie.
    assert_eq!(cpu.sp, 0xC0FE);
    assert_eq!(memory.read(0xC0FE), 0x01);
    assert_eq!(memory.read(0xC0FF), 0xC0);
}
#[test]
fn ei_enables_ime_after_next_instruction() {
    use crate::game::memory::GameMemory;
    use crate::rom::{Cartridge, Rom};

    let rom = Rom::load(
        "Legend of Zelda, The - Link's Awakening DX (USA, Europe) (Rev 2).gbc"
    )
    .expect("Nie można załadować ROM-u testowego");

    let cartridge = Cartridge::new(rom);
    let mut memory = GameMemory::new(cartridge);

    let mut cpu = Cpu::new();

    // C000: EI
    // C001: NOP
    memory.write(0xC000, 0xFB);
    memory.write(0xC001, 0x00);

    cpu.pc = 0xC000;
    cpu.ime = false;
    cpu.ime_pending = false;

    // EI
    let cycles = cpu.step(&mut memory);

    assert_eq!(cycles, 4);
    assert_eq!(cpu.pc, 0xC001);

    // EI nie włącza IME natychmiast.
    assert!(!cpu.ime);

    // IME oczekuje na następny krok.
    assert!(cpu.ime_pending);

    // NOP
    let cycles = cpu.step(&mut memory);

    assert_eq!(cycles, 4);
    assert_eq!(cpu.pc, 0xC002);

    // Dopiero po wykonaniu następnej instrukcji IME = true.
    assert!(cpu.ime);
    assert!(!cpu.ime_pending);
}
#[test]
fn ei_delays_interrupt_until_after_next_instruction() {
    use crate::game::memory::GameMemory;
    use crate::rom::{Cartridge, Rom};

    let rom = Rom::load(
        "Legend of Zelda, The - Link's Awakening DX (USA, Europe) (Rev 2).gbc"
    )
    .expect("Nie można załadować ROM-u testowego");

    let cartridge = Cartridge::new(rom);
    let mut memory = GameMemory::new(cartridge);

    let mut cpu = Cpu::new();

    // C000: EI
    // C001: NOP
    memory.write(0xC000, 0xFB);
    memory.write(0xC001, 0x00);

    cpu.pc = 0xC000;
    cpu.sp = 0xC100;
    cpu.ime = false;

    // Timer interrupt pending + enabled.
    memory.write(0xFF0F, 0x04);
    memory.write(0xFFFF, 0x04);

    // EI
    let cycles = cpu.step(&mut memory);

    assert_eq!(cycles, 4);
    assert_eq!(cpu.pc, 0xC001);

    // Interrupt nie może zostać obsłużony jeszcze teraz.
    assert!(!cpu.ime);
    assert!(cpu.ime_pending);
    assert_eq!(cpu.pc, 0xC001);

    // Następny krok wykonuje NOP.
    let cycles = cpu.step(&mut memory);

    assert_eq!(cycles, 4);

    // Dopiero teraz IME zostaje włączone.
    assert!(cpu.ime);
    assert!(!cpu.ime_pending);

    // Interrupt nadal nie powinien zostać obsłużony
    // w tym samym kroku co NOP.
    assert_eq!(cpu.pc, 0xC002);

    // Kolejny step powinien już obsłużyć Timer interrupt.
    let cycles = cpu.step(&mut memory);

    assert_eq!(cycles, 20);
    assert_eq!(cpu.pc, 0x0050);
    assert!(!cpu.ime);
}
#[test]
fn di_disables_ime_and_cancels_pending_ei() {
    use crate::game::memory::GameMemory;
    use crate::rom::{Cartridge, Rom};

    let rom = Rom::load(
        "Legend of Zelda, The - Link's Awakening DX (USA, Europe) (Rev 2).gbc"
    )
    .expect("Nie można załadować ROM-u testowego");

    let cartridge = Cartridge::new(rom);
    let mut memory = GameMemory::new(cartridge);

    let mut cpu = Cpu::new();

    // C000: EI
    // C001: DI
    memory.write(0xC000, 0xFB);
    memory.write(0xC001, 0xF3);

    cpu.pc = 0xC000;
    cpu.ime = false;
    cpu.ime_pending = false;

    // EI
    let cycles = cpu.step(&mut memory);

    assert_eq!(cycles, 4);
    assert_eq!(cpu.pc, 0xC001);
    assert!(!cpu.ime);
    assert!(cpu.ime_pending);

    // DI
    let cycles = cpu.step(&mut memory);

    assert_eq!(cycles, 4);
    assert_eq!(cpu.pc, 0xC002);

    // DI musi anulować oczekujące EI.
    assert!(!cpu.ime);
    assert!(!cpu.ime_pending);
}
#[test]
fn reti_restores_pc_and_enables_ime() {
    use crate::game::memory::GameMemory;
    use crate::rom::{Cartridge, Rom};

    let rom = Rom::load(
        "Legend of Zelda, The - Link's Awakening DX (USA, Europe) (Rev 2).gbc"
    )
    .expect("Nie można załadować ROM-u testowego");

    let cartridge = Cartridge::new(rom);
    let mut memory = GameMemory::new(cartridge);

    let mut cpu = Cpu::new();

    // RETI znajduje się w WRAM.
    cpu.pc = 0xC001;
    cpu.sp = 0xC0FE;
    cpu.ime = false;

    // Na stosie znajduje się adres powrotu C001.
    memory.write(0xC0FE, 0x01);
    memory.write(0xC0FF, 0xC0);

    // C001: RETI
    memory.write(0xC001, 0xD9);

    let cycles = cpu.step(&mut memory);

    assert_eq!(cycles, 16);

    // Przywrócony adres powrotu.
    assert_eq!(cpu.pc, 0xC001);

    // SP zwiększony po POP.
    assert_eq!(cpu.sp, 0xC100);

    // RETI ponownie włącza IME.
    assert!(cpu.ime);
}
#[test]
fn interrupt_then_reti_restores_cpu_state() {
    use crate::game::memory::GameMemory;
    use crate::rom::{Cartridge, Rom};

    let rom = Rom::load(
        "Legend of Zelda, The - Link's Awakening DX (USA, Europe) (Rev 2).gbc"
    )
    .expect("Nie można załadować ROM-u testowego");

    let cartridge = Cartridge::new(rom);
    let mut memory = GameMemory::new(cartridge);

    let mut cpu = Cpu::new();

    // CPU przed przerwaniem
    cpu.pc = 0xC001;
    cpu.sp = 0xC100;
    cpu.ime = true;
    cpu.halted = false;

    // Timer interrupt
    memory.write(0xFF0F, 0x04);
    memory.write(0xFFFF, 0x04);

    // ISR testujemy w WRAM, ponieważ ROM 0x0050 jest tylko do odczytu.
    memory.write(0xC002, 0xD9); // RETI

    // 1. Obsługa przerwania
    let cycles = cpu.step(&mut memory);

    assert_eq!(cycles, 20);
    assert_eq!(cpu.pc, 0x0050);
    assert_eq!(cpu.sp, 0xC0FE);
    assert!(!cpu.ime);
    assert!(!cpu.halted);

    // CPU powinien odłożyć adres powrotu C001 na stos.
    assert_eq!(memory.read(0xC0FE), 0x01);
    assert_eq!(memory.read(0xC0FF), 0xC0);

    // 2. Symulujemy wejście do ISR.
    // Właściwy wektor 0050 znajduje się w ROM,
    // dlatego test przenosi PC do przygotowanego RETI w WRAM.
    cpu.pc = 0xC002;

    // 3. RETI
    let cycles = cpu.step(&mut memory);

    assert_eq!(cycles, 16);
    assert_eq!(cpu.pc, 0xC001);
    assert_eq!(cpu.sp, 0xC100);

    // RETI musi ponownie włączyć IME.
    assert!(cpu.ime);
}
#[test]
fn interrupt_all_vectors_individually() {
    use crate::game::memory::GameMemory;
    use crate::rom::{Cartridge, Rom};

    let tests = [
        (0x01u8, 0x0040u16), // VBlank
        (0x02u8, 0x0048u16), // STAT
        (0x04u8, 0x0050u16), // Timer
        (0x08u8, 0x0058u16), // Serial
        (0x10u8, 0x0060u16), // Joypad
    ];

    for (interrupt_bit, vector) in tests {
        let rom = Rom::load(
            "Legend of Zelda, The - Link's Awakening DX (USA, Europe) (Rev 2).gbc"
        )
        .expect("Nie można załadować ROM-u testowego");

        let cartridge = Cartridge::new(rom);
        let mut memory = GameMemory::new(cartridge);

        let mut cpu = Cpu::new();

        cpu.pc = 0xC001;
        cpu.sp = 0xC100;
        cpu.ime = true;
        cpu.halted = false;

        memory.write(0xFF0F, interrupt_bit);
        memory.write(0xFFFF, interrupt_bit);

        let cycles = cpu.step(&mut memory);

        assert_eq!(
            cycles,
            20,
            "Zła liczba cykli dla interrupt {:02X}",
            interrupt_bit
        );

        assert_eq!(
            cpu.pc,
            vector,
            "Zły wektor dla interrupt {:02X}",
            interrupt_bit
        );

        assert_eq!(cpu.sp, 0xC0FE);
        assert!(!cpu.ime);
        assert!(!cpu.halted);

        // Adres powrotu C001 zapisany na stosie.
        assert_eq!(memory.read(0xC0FE), 0x01);
        assert_eq!(memory.read(0xC0FF), 0xC0);

        // Obsłużony bit IF musi zostać wyzerowany.
        assert_eq!(memory.read(0xFF0F) & interrupt_bit, 0);
    }
}
#[test]
fn halt_wakes_on_pending_interrupt_without_ime() {
    use crate::game::memory::GameMemory;
    use crate::rom::{Cartridge, Rom};

    let rom = Rom::load(
        "Legend of Zelda, The - Link's Awakening DX (USA, Europe) (Rev 2).gbc"
    )
    .expect("Nie można załadować ROM-u testowego");

    let cartridge = Cartridge::new(rom);
    let mut memory = GameMemory::new(cartridge);

    let mut cpu = Cpu::new();

    cpu.pc = 0xC000;
    cpu.sp = 0xC100;
    cpu.ime = false;
    cpu.halted = false;
    cpu.halt_bug = false;

    // Brak pending interrupt podczas wykonywania HALT.
    memory.write(0xFF0F, 0x00);
    memory.write(0xFFFF, 0x00);

    // C000: HALT
    memory.write(0xC000, 0x76);

    // C001: NOP
    memory.write(0xC001, 0x00);

    // 1. Wykonanie HALT
    let cycles = cpu.step(&mut memory);

    assert_eq!(cycles, 4);
    assert_eq!(cpu.pc, 0xC001);
    assert!(cpu.halted);
    assert!(!cpu.halt_bug);
    assert!(!cpu.ime);

    // Dopiero teraz pojawia się pending interrupt.
    memory.write(0xFF0F, 0x04);
    memory.write(0xFFFF, 0x04);

    // 2. Pending interrupt budzi CPU.
    // IME=0 -> interrupt nie jest obsługiwany.
    let cycles = cpu.step(&mut memory);

    assert_eq!(cycles, 4);
    assert!(!cpu.halted);

    // CPU nie skacze do 0050.
    assert_eq!(cpu.pc, 0xC001);

    // IME nadal wyłączone.
    assert!(!cpu.ime);

    // Interrupt nadal pending.
    assert_eq!(memory.read(0xFF0F) & 0x04, 0x04);

    // Stos bez zmian.
    assert_eq!(cpu.sp, 0xC100);
}
#[test]
fn halt_bug_reuses_next_opcode_byte() {
    use crate::game::memory::GameMemory;
    use crate::rom::{Cartridge, Rom};

    let rom = Rom::load(
        "Legend of Zelda, The - Link's Awakening DX (USA, Europe) (Rev 2).gbc"
    )
    .expect("Nie można załadować ROM-u testowego");

    let cartridge = Cartridge::new(rom);
    let mut memory = GameMemory::new(cartridge);

    let mut cpu = Cpu::new();

    cpu.pc = 0xC000;
    cpu.sp = 0xC100;
    cpu.ime = false;
    cpu.halted = false;
    cpu.halt_bug = false;

    // HALT
    memory.write(0xC000, 0x76);

    // LD A,d8
    memory.write(0xC001, 0x3E);

    // wartość, którą normalnie powinno załadować LD A,d8
    memory.write(0xC002, 0x42);

    // Timer interrupt pending
    memory.write(0xFF0F, 0x04);
    memory.write(0xFFFF, 0x04);

    // HALT
    assert_eq!(cpu.step(&mut memory), 4);

    assert!(!cpu.halted);
    assert!(cpu.halt_bug);
    assert_eq!(cpu.pc, 0xC001);
    assert!(!cpu.ime);

    // HALT bug:
    // opcode 3E z C001 zostaje pobrany,
    // ale PC nie zostaje zwiększony przed execute().
    //
    // read_imm8() ponownie odczyta więc C001,
    // czyli 3E zamiast 42.
    assert_eq!(cpu.step(&mut memory), 8);

    assert_eq!(cpu.a, 0x3E);
    assert_eq!(cpu.pc, 0xC002);
    assert!(!cpu.halt_bug);

    // Interrupt nadal pozostaje pending.
    assert_eq!(memory.read(0xFF0F) & 0x04, 0x04);
}
}
