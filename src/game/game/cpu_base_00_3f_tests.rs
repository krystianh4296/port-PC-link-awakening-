use crate::game::{Cpu, GameMemory};
use crate::rom::{Cartridge, Rom};

fn memory() -> GameMemory {
    let rom = Rom::load(
        "Legend of Zelda, The - Link's Awakening DX (USA, Europe) (Rev 2).gbc",
    )
    .expect("Nie można załadować ROM-u testowego");
    GameMemory::new(Cartridge::new(rom))
}

fn run(memory: &mut GameMemory, opcode: u8) -> (Cpu, u32) {
    memory.write(0xC000, opcode);
    memory.write(0xC001, 0x02);
    memory.write(0xC002, 0xC1);
    memory.write(0xC100, 0x40);

    let mut cpu = Cpu::new();
    cpu.pc = 0xC000;
    cpu.sp = 0xC100;
    cpu.b = 0x12;
    cpu.c = 0x34;
    cpu.d = 0x56;
    cpu.e = 0x78;
    cpu.h = 0xC1;
    cpu.l = 0x00;
    cpu.a = 0x42;
    cpu.f = 0;

    let cycles = cpu.step(memory);
    (cpu, cycles)
}

#[test]
fn base_opcodes_00_3f_cycles_and_pc() {
    let mut memory = memory();
    let cases: &[(u8, u32, u16)] = &[
        (0x00,4,0xC001),(0x01,12,0xC003),(0x02,8,0xC001),(0x03,8,0xC001),
        (0x04,4,0xC001),(0x05,4,0xC001),(0x06,8,0xC002),(0x07,4,0xC001),
        (0x08,20,0xC003),(0x09,8,0xC001),(0x0A,8,0xC001),(0x0B,8,0xC001),
        (0x0C,4,0xC001),(0x0D,4,0xC001),(0x0E,8,0xC002),(0x0F,4,0xC001),
        (0x10,4,0xC002),(0x11,12,0xC003),(0x12,8,0xC001),(0x13,8,0xC001),
        (0x14,4,0xC001),(0x15,4,0xC001),(0x16,8,0xC002),(0x17,4,0xC001),
        (0x18,12,0xC004),(0x19,8,0xC001),(0x1A,8,0xC001),(0x1B,8,0xC001),
        (0x1C,4,0xC001),(0x1D,4,0xC001),(0x1E,8,0xC002),(0x1F,4,0xC001),
        (0x20,12,0xC004),(0x21,12,0xC003),(0x22,8,0xC001),(0x23,8,0xC001),
        (0x24,4,0xC001),(0x25,4,0xC001),(0x26,8,0xC002),(0x27,4,0xC001),
        (0x28,8,0xC002),(0x29,8,0xC001),(0x2A,8,0xC001),(0x2B,8,0xC001),
        (0x2C,4,0xC001),(0x2D,4,0xC001),(0x2E,8,0xC002),(0x2F,4,0xC001),
        (0x30,12,0xC004),(0x31,12,0xC003),(0x32,8,0xC001),(0x33,8,0xC001),
        (0x34,12,0xC001),(0x35,12,0xC001),(0x36,12,0xC002),(0x37,4,0xC001),
        (0x38,8,0xC002),(0x39,8,0xC001),(0x3A,8,0xC001),(0x3B,8,0xC001),
        (0x3C,4,0xC001),(0x3D,4,0xC001),(0x3E,8,0xC002),(0x3F,4,0xC001),
    ];

    for &(opcode, expected_cycles, expected_pc) in cases {
        let (cpu, cycles) = run(&mut memory, opcode);
        assert_eq!(cycles, expected_cycles, "opcode {:02X}", opcode);
        assert_eq!(cpu.pc, expected_pc, "opcode {:02X}", opcode);
    }
}

#[test]
fn base_00_3f_conditional_jr_both_paths() {
    let mut memory = memory();
    for &(opcode, taken_flags, not_taken_flags) in &[
        (0x20, 0x00, 0x80),
        (0x28, 0x80, 0x00),
        (0x30, 0x00, 0x10),
        (0x38, 0x10, 0x00),
    ] {
        memory.write(0xC000, opcode);
        memory.write(0xC001, 0x02);

        let mut cpu = Cpu::new();
        cpu.pc = 0xC000;
        cpu.f = taken_flags;
        assert_eq!(cpu.step(&mut memory), 12, "taken {:02X}", opcode);
        assert_eq!(cpu.pc, 0xC004, "taken {:02X}", opcode);

        let mut cpu = Cpu::new();
        cpu.pc = 0xC000;
        cpu.f = not_taken_flags;
        assert_eq!(cpu.step(&mut memory), 8, "not taken {:02X}", opcode);
        assert_eq!(cpu.pc, 0xC002, "not taken {:02X}", opcode);
    }
}

#[test]
fn base_00_3f_memory_and_pair_instructions() {
    let mut memory = memory();
    let mut cpu = Cpu::new();

    memory.write(0xC000, 0x01);
    memory.write(0xC001, 0x34);
    memory.write(0xC002, 0x12);
    cpu.pc = 0xC000;
    assert_eq!(cpu.step(&mut memory), 12);
    assert_eq!(cpu.bc(), 0x1234);

    memory.write(0xC000, 0x08);
    memory.write(0xC001, 0x00);
    memory.write(0xC002, 0xC1);
    cpu.pc = 0xC000;
    cpu.sp = 0xBEEF;
    assert_eq!(cpu.step(&mut memory), 20);
    assert_eq!(memory.read(0xC100), 0xEF);
    assert_eq!(memory.read(0xC101), 0xBE);

    memory.write(0xC000, 0x22);
    cpu.pc = 0xC000;
    cpu.set_hl(0xC100);
    cpu.a = 0x5A;
    assert_eq!(cpu.step(&mut memory), 8);
    assert_eq!(memory.read(0xC100), 0x5A);
    assert_eq!(cpu.hl(), 0xC101);

    memory.write(0xC000, 0x3A);
    memory.write(0xC101, 0xA5);
    cpu.pc = 0xC000;
    cpu.set_hl(0xC101);
    assert_eq!(cpu.step(&mut memory), 8);
    assert_eq!(cpu.a, 0xA5);
    assert_eq!(cpu.hl(), 0xC100);
}

#[test]
fn base_00_3f_flags_and_special_cases() {
    let mut memory = memory();

    memory.write(0xC000, 0x04);
    let mut cpu = Cpu::new();
    cpu.pc = 0xC000;
    cpu.b = 0x0F;
    cpu.f = 0x10;
    assert_eq!(cpu.step(&mut memory), 4);
    assert_eq!(cpu.b, 0x10);
    assert_eq!(cpu.f, 0x30);

    memory.write(0xC000, 0x05);
    let mut cpu = Cpu::new();
    cpu.pc = 0xC000;
    cpu.b = 0x10;
    cpu.f = 0x10;
    assert_eq!(cpu.step(&mut memory), 4);
    assert_eq!(cpu.b, 0x0F);
    assert_eq!(cpu.f, 0x70);

    memory.write(0xC000, 0x09);
    let mut cpu = Cpu::new();
    cpu.pc = 0xC000;
    cpu.set_hl(0x0FFF);
    cpu.set_bc(0x0001);
    cpu.f = 0xC0;
    assert_eq!(cpu.step(&mut memory), 8);
    assert_eq!(cpu.hl(), 0x1000);
    assert_eq!(cpu.f, 0xA0);

    memory.write(0xC000, 0x07);
    let mut cpu = Cpu::new();
    cpu.pc = 0xC000;
    cpu.a = 0x80;
    cpu.f = 0xF0;
    assert_eq!(cpu.step(&mut memory), 4);
    assert_eq!(cpu.a, 0x01);
    assert_eq!(cpu.f, 0x10);

    memory.write(0xC000, 0x37);
    let mut cpu = Cpu::new();
    cpu.pc = 0xC000;
    cpu.f = 0xE0;
    assert_eq!(cpu.step(&mut memory), 4);
    assert_eq!(cpu.f, 0x90);

    memory.write(0xC000, 0x3F);
    let mut cpu = Cpu::new();
    cpu.pc = 0xC000;
    cpu.f = 0x80;
    assert_eq!(cpu.step(&mut memory), 4);
    assert_eq!(cpu.f, 0x90);

    memory.write(0xC000, 0x2F);
    let mut cpu = Cpu::new();
    cpu.pc = 0xC000;
    cpu.a = 0x55;
    cpu.f = 0x90;
    assert_eq!(cpu.step(&mut memory), 4);
    assert_eq!(cpu.a, 0xAA);
    assert_eq!(cpu.f, 0xF0);
}
