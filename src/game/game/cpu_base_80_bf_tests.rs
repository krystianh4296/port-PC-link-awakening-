use crate::game::{Cpu, GameMemory};
use crate::rom::{Cartridge, Rom};

fn memory() -> GameMemory {
    let rom = Rom::load(
        "Legend of Zelda, The - Link's Awakening DX (USA, Europe) (Rev 2).gbc",
    )
    .expect("Nie można załadować ROM-u testowego");
    GameMemory::new(Cartridge::new(rom))
}

fn run(opcode: u8, a: u8, value: u8, flags: u8) -> (Cpu, GameMemory, u32) {
    let mut memory = memory();
    memory.write(0xC000, opcode);
    memory.write(0xC100, value);

    let mut cpu = Cpu::new();
    cpu.pc = 0xC000;
    cpu.set_hl(0xC100);
    cpu.a = a;
    cpu.b = value;
    cpu.c = value;
    cpu.d = value;
    cpu.e = value;
    cpu.h = 0xC1;
    cpu.l = 0x00;
    cpu.f = flags;

    let cycles = cpu.step(&mut memory);
    (cpu, memory, cycles)
}

#[test]
fn base_80_bf_all_opcodes_have_correct_cycles_and_pc() {
    for opcode in 0x80u8..=0xBF {
        let (cpu, _memory, cycles) = run(opcode, 0x53, 0x27, 0x10);
        let expected_cycles = if opcode & 7 == 6 { 8 } else { 4 };

        assert_eq!(cycles, expected_cycles, "opcode {:02X}", opcode);
        assert_eq!(cpu.pc, 0xC001, "opcode {:02X}", opcode);
    }
}

#[test]
fn base_80_bf_alu_operations_produce_correct_results() {
    let cases = [
        (0x80, 0x10, 0x22, 0x32), // ADD B
        (0x81, 0x10, 0x22, 0x32), // ADD C
        (0x82, 0x10, 0x22, 0x32), // ADD D
        (0x83, 0x10, 0x22, 0x32), // ADD E
        (0x84, 0x10, 0x22, 0xD1), // ADD H (H=0xC1)
        (0x85, 0x10, 0x22, 0x10), // ADD L (L=0x00)
        (0x87, 0x10, 0x22, 0x20), // ADD A (A=0x10)
        (0x88, 0x10, 0x22, 0x32), // ADC B, carry=0
        (0x90, 0x32, 0x10, 0x22), // SUB B
        (0x98, 0x32, 0x10, 0x22), // SBC B, carry=0
        (0xA0, 0xF0, 0x0F, 0x00), // AND B
        (0xA8, 0xF0, 0x0F, 0xFF), // XOR B
        (0xB0, 0xF0, 0x0F, 0xFF), // OR B
        (0xB8, 0x32, 0x10, 0x32), // CP B preserves A
    ];

    for &(opcode, a, value, expected) in &cases {
        let mut memory = memory();
        memory.write(0xC000, opcode);
        memory.write(0xC100, value);

        let mut cpu = Cpu::new();
        cpu.pc = 0xC000;
        cpu.set_hl(0xC100);
        cpu.a = a;
        cpu.b = value;
        cpu.c = value;
        cpu.d = value;
        cpu.e = value;
        cpu.h = 0xC1;
        cpu.l = 0x00;
        cpu.f = 0x00;

        cpu.step(&mut memory);

        if opcode == 0xB8 {
            assert_eq!(cpu.a, a, "opcode {:02X}", opcode);
        } else {
            assert_eq!(cpu.a, expected, "opcode {:02X}", opcode);
        }
    }
}

#[test]
fn base_80_bf_add_adc_flags_are_correct() {
    let (cpu, _, _) = run(0x80, 0x0F, 0x01, 0x00);
    assert_eq!(cpu.a, 0x10);
    assert_eq!(cpu.f, 0x20); // H

    let (cpu, _, _) = run(0x80, 0xFF, 0x01, 0x00);
    assert_eq!(cpu.a, 0x00);
    assert_eq!(cpu.f, 0xB0); // Z,H,C

    let (cpu, _, _) = run(0x88, 0x0F, 0x00, 0x10);
    assert_eq!(cpu.a, 0x10);
    assert_eq!(cpu.f, 0x20); // H

    let (cpu, _, _) = run(0x88, 0xFF, 0x00, 0x10);
    assert_eq!(cpu.a, 0x00);
    assert_eq!(cpu.f, 0xB0); // Z,H,C
}

#[test]
fn base_80_bf_sub_sbc_cp_flags_are_correct() {
    let (cpu, _, _) = run(0x90, 0x10, 0x01, 0x00);
    assert_eq!(cpu.a, 0x0F);
    assert_eq!(cpu.f, 0x60); // N,H

    let (cpu, _, _) = run(0x90, 0x00, 0x01, 0x00);
    assert_eq!(cpu.a, 0xFF);
    assert_eq!(cpu.f, 0x70); // N,H,C

    let (cpu, _, _) = run(0x98, 0x10, 0x00, 0x10);
    assert_eq!(cpu.a, 0x0F);
    assert_eq!(cpu.f, 0x60); // N,H

    let (cpu, _, _) = run(0x98, 0x00, 0x00, 0x10);
    assert_eq!(cpu.a, 0xFF);
    assert_eq!(cpu.f, 0x70); // N,H,C

    let (cpu, _, _) = run(0xB8, 0x42, 0x42, 0x00);
    assert_eq!(cpu.a, 0x42);
    assert_eq!(cpu.f, 0xC0); // Z,N
}

#[test]
fn base_80_bf_logic_flags_are_correct() {
    let (cpu, _, _) = run(0xA0, 0xF0, 0x0F, 0xF0);
    assert_eq!(cpu.a, 0x00);
    assert_eq!(cpu.f, 0xA0); // Z,H

    let (cpu, _, _) = run(0xA8, 0xFF, 0xFF, 0xF0);
    assert_eq!(cpu.a, 0x00);
    assert_eq!(cpu.f, 0x80); // Z

    let (cpu, _, _) = run(0xB0, 0xF0, 0x0F, 0xF0);
    assert_eq!(cpu.a, 0xFF);
    assert_eq!(cpu.f, 0x00);
}

#[test]
fn base_80_bf_hl_operand_is_used_and_flags_are_correct() {
    let mut memory = memory();
    memory.write(0xC000, 0x86); // ADD A,(HL)
    memory.write(0xC100, 0x01);

    let mut cpu = Cpu::new();
    cpu.pc = 0xC000;
    cpu.set_hl(0xC100);
    cpu.a = 0x0F;
    cpu.f = 0x10;

    assert_eq!(cpu.step(&mut memory), 8);
    assert_eq!(cpu.a, 0x10);
    assert_eq!(cpu.f, 0x20);

    memory.write(0xC000, 0xA6); // AND A,(HL)
    memory.write(0xC100, 0x0F);
    cpu.pc = 0xC000;
    cpu.a = 0xFF;
    cpu.f = 0x10;

    assert_eq!(cpu.step(&mut memory), 8);
    assert_eq!(cpu.a, 0x0F);
    assert_eq!(cpu.f, 0x20);
}

#[test]
fn base_80_bf_non_a_registers_are_not_modified() {
    let (cpu, _, _) = run(0x80, 0x10, 0x22, 0x00);

    assert_eq!(cpu.b, 0x22);
    assert_eq!(cpu.c, 0x22);
    assert_eq!(cpu.d, 0x22);
    assert_eq!(cpu.e, 0x22);
    assert_eq!(cpu.hl(), 0xC100);
}
