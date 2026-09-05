use crate::game::{Cpu, GameMemory};
use crate::rom::{Cartridge, Rom};

fn memory() -> GameMemory {
    let rom = Rom::load(
        "Legend of Zelda, The - Link's Awakening DX (USA, Europe) (Rev 2).gbc",
    )
    .expect("Nie można załadować ROM-u testowego");
    GameMemory::new(Cartridge::new(rom))
}

fn cpu_at_cb(opcode: u8) -> (Cpu, GameMemory) {
    let mut memory = memory();
    memory.write(0xC000, 0xCB);
    memory.write(0xC001, opcode);

    let mut cpu = Cpu::new();
    cpu.pc = 0xC000;
    cpu.set_hl(0xC100);
    (cpu, memory)
}

fn read_target(cpu: &Cpu, memory: &mut GameMemory, z: u8) -> u8 {
    match z {
        0 => cpu.b,
        1 => cpu.c,
        2 => cpu.d,
        3 => cpu.e,
        4 => cpu.h,
        5 => cpu.l,
        6 => memory.read(0xC100),
        7 => cpu.a,
        _ => unreachable!(),
    }
}

fn write_target(cpu: &mut Cpu, memory: &mut GameMemory, z: u8, value: u8) {
    match z {
        0 => cpu.b = value,
        1 => cpu.c = value,
        2 => cpu.d = value,
        3 => cpu.e = value,
        4 => cpu.h = value,
        5 => cpu.l = value,
        6 => memory.write(0xC100, value),
        7 => cpu.a = value,
        _ => unreachable!(),
    }
}

fn expected_rotate_shift(op: u8, value: u8, carry_in: bool) -> (u8, bool) {
    match op {
        0 => (value.rotate_left(1), value & 0x80 != 0),
        1 => (value.rotate_right(1), value & 0x01 != 0),
        2 => (
            (value << 1) | u8::from(carry_in),
            value & 0x80 != 0,
        ),
        3 => (
            (value >> 1) | if carry_in { 0x80 } else { 0 },
            value & 0x01 != 0,
        ),
        4 => (value << 1, value & 0x80 != 0),
        5 => ((value >> 1) | (value & 0x80), value & 0x01 != 0),
        6 => (value.rotate_left(4), false),
        7 => (value >> 1, value & 0x01 != 0),
        _ => unreachable!(),
    }
}

#[test]
fn cb_all_256_opcodes_have_correct_operation_cycles_and_pc() {
    for opcode in 0u8..=0xFF {
        let (mut cpu, mut memory) = cpu_at_cb(opcode);
        let x = opcode >> 6;
        let y = (opcode >> 3) & 0x07;
        let z = opcode & 0x07;

        let initial = 0x96u8;
        write_target(&mut cpu, &mut memory, z, initial);
        cpu.f = 0x10 | 0x80;

        let carry_in = true;
        let cycles = cpu.step(&mut memory);

        assert_eq!(cpu.pc, 0xC002, "CB opcode {:02X} PC", opcode);

        let expected_cycles = if z == 6 {
            if x == 1 { 12 } else { 16 }
        } else {
            8
        };
        assert_eq!(cycles, expected_cycles, "CB opcode {:02X} cycles", opcode);

        match x {
            0 => {
                let (expected, carry) = expected_rotate_shift(y, initial, carry_in);
                assert_eq!(
                    read_target(&cpu, &mut memory, z),
                    expected,
                    "CB opcode {:02X} result",
                    opcode
                );
                assert_eq!(
                    cpu.f,
                    (if expected == 0 { 0x80 } else { 0 })
                        | (if carry { 0x10 } else { 0 }),
                    "CB opcode {:02X} flags",
                    opcode
                );
            }
            1 => {
                let bit_set = initial & (1 << y) != 0;
                assert_eq!(
                    read_target(&cpu, &mut memory, z),
                    initial,
                    "CB opcode {:02X} BIT must not modify operand",
                    opcode
                );
                assert_eq!(
                    cpu.f,
                    (if !bit_set { 0x80 } else { 0 }) | 0x20 | 0x10,
                    "CB opcode {:02X} flags",
                    opcode
                );
            }
            2 => {
                let expected = initial & !(1 << y);
                assert_eq!(
                    read_target(&cpu, &mut memory, z),
                    expected,
                    "CB opcode {:02X} RES result",
                    opcode
                );
                assert_eq!(cpu.f, 0x90, "CB opcode {:02X} RES flags", opcode);
            }
            3 => {
                let expected = initial | (1 << y);
                assert_eq!(
                    read_target(&cpu, &mut memory, z),
                    expected,
                    "CB opcode {:02X} SET result",
                    opcode
                );
                assert_eq!(cpu.f, 0x90, "CB opcode {:02X} SET flags", opcode);
            }
            _ => unreachable!(),
        }
    }
}

#[test]
fn cb_bit_res_set_cover_all_bits_and_all_register_targets() {
    for z in 0u8..=7 {
        for y in 0u8..=7 {
            for x in 1u8..=3 {
                let opcode = (x << 6) | (y << 3) | z;
                let (mut cpu, mut memory) = cpu_at_cb(opcode);
                let initial = 0x55u8;
                write_target(&mut cpu, &mut memory, z, initial);
                cpu.f = 0x10 | 0x40;

                let expected_cycles = if z == 6 {
                    if x == 1 { 12 } else { 16 }
                } else {
                    8
                };
                assert_eq!(cpu.step(&mut memory), expected_cycles, "CB {:02X} cycles", opcode);
                assert_eq!(cpu.pc, 0xC002, "CB {:02X} PC", opcode);

                match x {
                    1 => {
                        let expected_z = initial & (1 << y) == 0;
                        assert_eq!(read_target(&cpu, &mut memory, z), initial);
                        assert_eq!(cpu.f, (if expected_z { 0x80 } else { 0 }) | 0x20 | 0x10);
                    }
                    2 => {
                        assert_eq!(read_target(&cpu, &mut memory, z), initial & !(1 << y));
                        assert_eq!(cpu.f, 0x50);
                    }
                    3 => {
                        assert_eq!(read_target(&cpu, &mut memory, z), initial | (1 << y));
                        assert_eq!(cpu.f, 0x50);
                    }
                    _ => unreachable!(),
                }
            }
        }
    }
}

#[test]
fn cb_rotate_shift_group_covers_all_operations_with_carry_and_zero_flags() {
    for y in 0u8..=7 {
        for &initial in &[0x00u8, 0x01, 0x80, 0xFF, 0x96] {
            for &carry_in in &[false, true] {
                let opcode = (y << 3) | 0x07;
                let (mut cpu, mut memory) = cpu_at_cb(opcode);
                cpu.a = initial;
                cpu.f = if carry_in { 0x10 } else { 0 };

                let (expected, carry) = expected_rotate_shift(y, initial, carry_in);
                assert_eq!(cpu.step(&mut memory), 8, "CB {:02X}", opcode);
                assert_eq!(cpu.a, expected, "CB {:02X} result", opcode);
                assert_eq!(
                    cpu.f,
                    (if expected == 0 { 0x80 } else { 0 })
                        | (if carry { 0x10 } else { 0 }),
                    "CB {:02X} flags",
                    opcode
                );
            }
        }
    }
}
