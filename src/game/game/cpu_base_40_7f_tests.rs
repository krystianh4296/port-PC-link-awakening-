use crate::game::game::{Cpu, GameMemory};
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
    let mut cpu = Cpu::new();
    cpu.pc = 0xC000;
    cpu.set_hl(0xC100);
    cpu.b = 0x12;
    cpu.c = 0x34;
    cpu.d = 0x56;
    cpu.e = 0x78;
    cpu.h = 0xC1;
    cpu.l = 0x00;
    cpu.a = 0x9A;
    let cycles = cpu.step(memory);
    (cpu, cycles)
}

#[test]
fn base_40_7f_all_ld_opcodes_have_correct_cycles_and_pc() {
    let mut memory = memory();

    for opcode in 0x40u8..=0x7F {
        if opcode == 0x76 {
            continue;
        }
        memory.write(0xC100, 0xA5);
        let (cpu, cycles) = run(&mut memory, opcode);
        let expected_cycles = if (opcode & 7) == 6 || ((opcode >> 3) & 7) == 6 { 8 } else { 4 };
        assert_eq!(cycles, expected_cycles, "opcode {:02X}", opcode);
        assert_eq!(cpu.pc, 0xC001, "opcode {:02X}", opcode);
    }
}

#[test]
fn base_40_7f_ld_register_matrix_copies_correct_values() {
    let mut memory = memory();
    // B, C, D, E, H, L, (HL), A
    let values = [0x12u8, 0x34, 0x56, 0x78, 0xC1, 0x00, 0xA5, 0x9A];

    for opcode in 0x40u8..=0x7F {
        if opcode == 0x76 {
            continue;
        }
        let dst = ((opcode >> 3) & 7) as usize;
        let src = (opcode & 7) as usize;

        let mut cpu = Cpu::new();
        cpu.pc = 0xC000;
        cpu.set_hl(0xC100);
        cpu.b = values[0];
        cpu.c = values[1];
        cpu.d = values[2];
        cpu.e = values[3];
        cpu.h = values[4];
        cpu.l = values[5];
        cpu.a = values[7];
        memory.write(0xC000, opcode);
        memory.write(0xC100, values[6]);

        let old_flags = 0xF0;
        cpu.f = old_flags;
        cpu.step(&mut memory);

        let result = match dst {
            0 => cpu.b,
            1 => cpu.c,
            2 => cpu.d,
            3 => cpu.e,
            4 => cpu.h,
            5 => cpu.l,
            6 => memory.read(0xC100),
            7 => cpu.a,
            _ => unreachable!(),
        };
        let expected = values[src];
        assert_eq!(result, expected, "opcode {:02X}", opcode);
        assert_eq!(cpu.f, old_flags, "flags changed for opcode {:02X}", opcode);
    }
}

#[test]
fn base_40_7f_hl_loads_use_memory_operand() {
    let mut memory = memory();
    memory.write(0xC100, 0xA5);

    for &(opcode, expected_reg) in &[
        (0x46, 0), // LD B,(HL)
        (0x4E, 1), // LD C,(HL)
        (0x56, 2), // LD D,(HL)
        (0x5E, 3), // LD E,(HL)
        (0x66, 4), // LD H,(HL)
        (0x6E, 5), // LD L,(HL)
        (0x7E, 7), // LD A,(HL)
    ] {
        let mut cpu = Cpu::new();
        cpu.pc = 0xC000;
        cpu.set_hl(0xC100);
        memory.write(0xC000, opcode);
        memory.write(0xC100, 0xA5);
        assert_eq!(cpu.step(&mut memory), 8, "opcode {:02X}", opcode);
        let result = match expected_reg {
            0 => cpu.b,
            1 => cpu.c,
            2 => cpu.d,
            3 => cpu.e,
            4 => cpu.h,
            5 => cpu.l,
            7 => cpu.a,
            _ => unreachable!(),
        };
        assert_eq!(result, 0xA5, "opcode {:02X}", opcode);
    }
}

#[test]
fn base_40_7f_register_to_hl_stores_correct_values() {
    let mut memory = memory();
    let cases = [
        (0x70, 0x12), // LD (HL),B
        (0x71, 0x34), // LD (HL),C
        (0x72, 0x56), // LD (HL),D
        (0x73, 0x78), // LD (HL),E
        (0x74, 0xC1), // LD (HL),H
        (0x75, 0x00), // LD (HL),L
        (0x77, 0x9A), // LD (HL),A
    ];

    for &(opcode, expected) in &cases {
        let mut cpu = Cpu::new();
        cpu.pc = 0xC000;
        cpu.set_hl(0xC100);
        cpu.b = 0x12;
        cpu.c = 0x34;
        cpu.d = 0x56;
        cpu.e = 0x78;
        cpu.a = 0x9A;
        memory.write(0xC000, opcode);
        memory.write(0xC100, 0x00);
        assert_eq!(cpu.step(&mut memory), 8, "opcode {:02X}", opcode);
        assert_eq!(memory.read(0xC100), expected, "opcode {:02X}", opcode);
    }
}

#[test]
fn base_40_7f_ld_hl_hl_is_noop_and_halt_is_covered_by_existing_test() {
    let mut memory = memory();
    memory.write(0xC000, 0x76); // HALT
    let mut cpu = Cpu::new();
    cpu.pc = 0xC000;
    cpu.set_hl(0xC100);
    cpu.f = 0xF0;
    assert_eq!(cpu.step(&mut memory), 4);
    assert_eq!(cpu.pc, 0xC001);
    assert!(cpu.halted);
    assert_eq!(cpu.f, 0xF0);

    memory.write(0xC000, 0x76);
    let mut cpu = Cpu::new();
    cpu.pc = 0xC000;
    cpu.b = 0x12;
    cpu.f = 0xF0;
    assert_eq!(cpu.step(&mut memory), 4);
    assert_eq!(cpu.b, 0x12);
    assert_eq!(cpu.pc, 0xC001);
    assert_eq!(cpu.f, 0xF0);
}
