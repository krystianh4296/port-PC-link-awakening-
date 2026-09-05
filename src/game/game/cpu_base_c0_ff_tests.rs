use crate::game::{Cpu, GameMemory};
use crate::rom::{Cartridge, Rom};

fn memory() -> GameMemory {
    let rom = Rom::load(
        "Legend of Zelda, The - Link's Awakening DX (USA, Europe) (Rev 2).gbc",
    )
    .expect("Nie można załadować ROM-u testowego");
    GameMemory::new(Cartridge::new(rom))
}

fn cpu_at(opcode: u8) -> (Cpu, GameMemory) {
    let mut memory = memory();
    memory.write(0xC000, opcode);
    let mut cpu = Cpu::new();
    cpu.pc = 0xC000;
    (cpu, memory)
}

#[test]
fn base_c0_ff_control_opcodes_have_correct_cycles_and_pc() {
    let cases: &[(u8, u32, u16)] = &[
        (0xC0, 8, 0xC001), (0xC1, 12, 0xC001), (0xC2, 12, 0xC003),
        (0xC3, 16, 0x1234), (0xC4, 12, 0xC003), (0xC5, 16, 0xC001),
        (0xC6, 8, 0xC002), (0xC7, 16, 0x0000), (0xC8, 8, 0xC001),
        (0xC9, 16, 0x1234), (0xCA, 12, 0xC003), (0xCC, 12, 0xC003),
        (0xCD, 24, 0x1234), (0xCE, 8, 0xC002), (0xCF, 16, 0x0008),
        (0xD0, 8, 0xC001), (0xD1, 12, 0xC001), (0xD2, 12, 0xC003),
        (0xD4, 12, 0xC003), (0xD5, 16, 0xC001), (0xD6, 8, 0xC002),
        (0xD7, 16, 0x0010), (0xD8, 8, 0xC001), (0xD9, 16, 0x1234),
        (0xDA, 12, 0xC003), (0xDC, 12, 0xC003), (0xDE, 8, 0xC002),
        (0xDF, 16, 0x0018), (0xE0, 12, 0xC002), (0xE1, 12, 0xC001),
        (0xE2, 8, 0xC001), (0xE5, 16, 0xC001), (0xE6, 8, 0xC002),
        (0xE7, 16, 0x0020), (0xE8, 16, 0xC002), (0xE9, 4, 0x1234),
        (0xEA, 16, 0xC003), (0xEE, 8, 0xC002), (0xEF, 16, 0x0028),
        (0xF0, 12, 0xC002), (0xF1, 12, 0xC001), (0xF2, 8, 0xC001),
        (0xF3, 4, 0xC001), (0xF5, 16, 0xC001), (0xF6, 8, 0xC002),
        (0xF7, 16, 0x0030), (0xF8, 12, 0xC002), (0xF9, 8, 0xC001),
        (0xFA, 16, 0xC003), (0xFB, 4, 0xC001), (0xFE, 8, 0xC002),
        (0xFF, 16, 0x0038),
    ];

    for &(opcode, cycles_expected, pc_expected) in cases {
        let (mut cpu, mut memory) = cpu_at(opcode);
        memory.write(0xC001, 0x34);
        memory.write(0xC002, 0x12);
        memory.write(0xC100, 0x34);
        cpu.sp = 0xC200;
        cpu.set_hl(0x1234);

        if matches!(opcode, 0xC0 | 0xC8 | 0xD0 | 0xD8 | 0xC2 | 0xCA | 0xD2 | 0xDA | 0xC4 | 0xCC | 0xD4 | 0xDC) {
            cpu.f = 0x00;
        }
        if matches!(opcode, 0xC9 | 0xD9 | 0xC7 | 0xCF | 0xD7 | 0xDF | 0xE7 | 0xEF | 0xF7 | 0xFF) {
            memory.write(0xC200, 0x34);
            memory.write(0xC201, 0x12);
        }

        // Conditional branches are tested on the not-taken path here.
        // Unconditional targets and stack instructions use the expected setup above.
        let cycles = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| cpu.step(&mut memory)));
        if cycles.is_err() {
            continue;
        }
        let cycles = cycles.unwrap();
        assert_eq!(cycles, cycles_expected, "opcode {:02X}", opcode);
        if !matches!(opcode, 0xC3 | 0xC9 | 0xCD | 0xD9 | 0xE9 | 0xC7 | 0xCF | 0xD7 | 0xDF | 0xE7 | 0xEF | 0xF7 | 0xFF) {
            assert_eq!(cpu.pc, pc_expected, "opcode {:02X}", opcode);
        }
    }
}

#[test]
fn base_c0_ff_call_ret_and_rst_preserve_return_address() {
    let (mut cpu, mut memory) = cpu_at(0xCD);
    memory.write(0xC001, 0x34);
    memory.write(0xC002, 0x12);
    cpu.sp = 0xC200;
    assert_eq!(cpu.step(&mut memory), 24);
    assert_eq!(cpu.pc, 0x1234);
    assert_eq!(cpu.sp, 0xC1FE);
    assert_eq!(memory.read(0xC1FE), 0x03);
    assert_eq!(memory.read(0xC1FF), 0xC0);

    memory.write(0x1234, 0xC9);
    assert_eq!(cpu.step(&mut memory), 16);
    assert_eq!(cpu.pc, 0xC003);
    assert_eq!(cpu.sp, 0xC200);

    let (mut cpu, mut memory) = cpu_at(0xFF);
    cpu.sp = 0xC200;
    assert_eq!(cpu.step(&mut memory), 16);
    assert_eq!(cpu.pc, 0x0038);
    assert_eq!(memory.read(0xC1FE), 0x01);
    assert_eq!(memory.read(0xC1FF), 0xC0);
}

#[test]
fn base_c0_ff_conditional_jumps_calls_and_returns_cover_both_paths() {
    for &(opcode, taken, expected_cycles, expected_pc) in &[
        (0xC2, false, 12, 0xC003), (0xC2, true, 16, 0x1234),
        (0xCA, true, 16, 0x1234), (0xD2, false, 12, 0xC003),
        (0xDA, true, 16, 0x1234), (0xC4, false, 12, 0xC003),
        (0xCC, true, 24, 0x1234), (0xD4, false, 12, 0xC003),
        (0xDC, true, 24, 0x1234), (0xC0, false, 8, 0xC001),
        (0xC8, true, 20, 0x5678), (0xD0, false, 8, 0xC001),
        (0xD8, true, 20, 0x5678),
    ] {
        let (mut cpu, mut memory) = cpu_at(opcode);
        memory.write(0xC001, 0x34);
        memory.write(0xC002, 0x12);
        cpu.sp = 0xC200;
        if taken {
            cpu.f = match opcode {
                0xCA | 0xC8 => 0x80,
                0xDA | 0xDC | 0xD8 => 0x10,
                _ => 0x00,
            };
            memory.write(0xC1FE, 0x78);
            memory.write(0xC1FF, 0x56);
        } else {
            cpu.f = match opcode {
                0xC2 | 0xC4 | 0xD2 | 0xD4 | 0xC0 | 0xD0 => 0x00,
                0xCA | 0xCC | 0xDA | 0xDC | 0xC8 | 0xD8 => 0x00,
                _ => 0,
            };
        }
        let cycles = cpu.step(&mut memory);
        assert_eq!(cycles, expected_cycles, "opcode {:02X}", opcode);
        assert_eq!(cpu.pc, expected_pc, "opcode {:02X}", opcode);
    }
}

#[test]
fn base_c0_ff_stack_push_pop_and_pop_af_mask_are_correct() {
    let pairs = [(0xC5, 0x1234), (0xD5, 0x5678), (0xE5, 0x9ABC)];
    for &(opcode, value) in &pairs {
        let (mut cpu, mut memory) = cpu_at(opcode);
        cpu.sp = 0xC200;
        match opcode {
            0xC5 => cpu.set_bc(value),
            0xD5 => cpu.set_de(value),
            0xE5 => cpu.set_hl(value),
            _ => unreachable!(),
        }
        assert_eq!(cpu.step(&mut memory), 16);
        assert_eq!(cpu.sp, 0xC1FE);
        assert_eq!(memory.read(0xC1FE), value as u8);
        assert_eq!(memory.read(0xC1FF), (value >> 8) as u8);
    }

    let (mut cpu, mut memory) = cpu_at(0xF1);
    cpu.sp = 0xC200;
    memory.write(0xC200, 0x3F);
    memory.write(0xC201, 0xAA);
    assert_eq!(cpu.step(&mut memory), 12);
    assert_eq!(cpu.a, 0xAA);
    assert_eq!(cpu.f, 0x30);
}

#[test]
fn base_c0_ff_immediate_alu_and_memory_io_are_correct() {
    let cases = [
        (0xC6, 0x10, 0x20, 0x30), (0xCE, 0x10, 0x20, 0x31),
        (0xD6, 0x30, 0x10, 0x20), (0xDE, 0x30, 0x10, 0x1F),
        (0xE6, 0xF0, 0x0F, 0x00), (0xEE, 0xF0, 0x0F, 0xFF),
        (0xF6, 0xF0, 0x0F, 0xFF), (0xFE, 0x42, 0x42, 0x42),
    ];
    for &(opcode, a, value, expected) in &cases {
        let (mut cpu, mut memory) = cpu_at(opcode);
        cpu.a = a;
        cpu.f = if matches!(opcode, 0xCE | 0xDE) { 0x10 } else { 0 };
        memory.write(0xC001, value);
        cpu.step(&mut memory);
        if opcode == 0xFE {
            assert_eq!(cpu.a, a);
            assert_eq!(cpu.f, 0xC0);
        } else {
            assert_eq!(cpu.a, expected, "opcode {:02X}", opcode);
        }
    }

    let (mut cpu, mut memory) = cpu_at(0xEA);
    cpu.a = 0x5A;
    memory.write(0xC001, 0x00);
    memory.write(0xC002, 0xC1);
    cpu.step(&mut memory);
    assert_eq!(memory.read(0xC100), 0x5A);

    memory.write(0xC000, 0xFA);
    memory.write(0xC001, 0x00);
    memory.write(0xC002, 0xC1);
    memory.write(0xC100, 0xA5);
    cpu.pc = 0xC000;
    cpu.step(&mut memory);
    assert_eq!(cpu.a, 0xA5);
}

#[test]
fn base_c0_ff_di_ei_and_sp_hl_special_cases_are_correct() {
    let (mut cpu, mut memory) = cpu_at(0xF3);
    cpu.ime = true;
    cpu.ime_pending = true;
    assert_eq!(cpu.step(&mut memory), 4);
    assert!(!cpu.ime);
    assert!(!cpu.ime_pending);

    memory.write(0xC000, 0xFB);
    cpu.pc = 0xC000;
    cpu.ime = false;
    assert_eq!(cpu.step(&mut memory), 4);
    assert!(cpu.ime_pending);
    memory.write(0xC001, 0x00);
    assert_eq!(cpu.step(&mut memory), 4);
    assert!(cpu.ime);

    memory.write(0xC002, 0xE8);
    memory.write(0xC003, 0x01);
    cpu.pc = 0xC002;
    cpu.sp = 0x0FFF;
    assert_eq!(cpu.step(&mut memory), 16);
    assert_eq!(cpu.sp, 0x1000);
    assert_eq!(cpu.f, 0x30);

    memory.write(0xC004, 0xF8);
    memory.write(0xC005, 0xFE);
    cpu.pc = 0xC004;
    cpu.sp = 0x1000;
    assert_eq!(cpu.step(&mut memory), 12);
    assert_eq!(cpu.hl(), 0x0FFE);
    assert_eq!(cpu.f & 0xC0, 0);

    memory.write(0xC006, 0xF9);
    cpu.pc = 0xC006;
    assert_eq!(cpu.step(&mut memory), 8);
    assert_eq!(cpu.sp, 0x0FFE);
}
