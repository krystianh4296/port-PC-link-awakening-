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
    cpu.sp = 0xC200;

    (cpu, memory)
}

#[test]
fn cpu_single_step_cycles_match_machine_cycle_lengths() {
    let cases = [
        (0x00, 4),  // NOP
        (0x06, 8),  // LD B,d8
        (0x36, 12), // LD (HL),d8
        (0xC3, 16), // JP a16
        (0xCD, 24), // CALL a16
        (0xC9, 16), // RET
        (0xE8, 16), // ADD SP,e8
        (0xF9, 8),  // LD SP,HL
        (0x76, 4),  // HALT
        (0xCB, 8),  // CB prefix + register operation
    ];

    for &(opcode, expected_cycles) in &cases {
        let (mut cpu, mut memory) = cpu_at(opcode);
        memory.write(0xC001, 0x00);
        memory.write(0xC002, 0x00);
        cpu.set_hl(0xC100);
        memory.write(0xC100, 0x00);

        if opcode == 0x36 {
            memory.write(0xC001, 0x42);
        }
        if opcode == 0xC3 || opcode == 0xCD {
            memory.write(0xC001, 0x34);
            memory.write(0xC002, 0x12);
        }
        if opcode == 0xE8 {
            memory.write(0xC001, 0x01);
            cpu.sp = 0x1000;
        }
        if opcode == 0xF9 {
            cpu.set_hl(0x1234);
        }
        if opcode == 0xCB {
            memory.write(0xC001, 0x00); // RLC B
            cpu.b = 0x80;
        }

        let cycles = cpu.step(&mut memory);
        assert_eq!(cycles, expected_cycles, "opcode {:02X}", opcode);
    }
}

#[test]
fn conditional_instruction_timing_differs_only_on_taken_path() {
    let cases = [
        (0x20, false, 8),
        (0x20, true, 12),
        (0xC2, false, 12),
        (0xC2, true, 16),
        (0xC4, false, 12),
        (0xC4, true, 24),
        (0xC0, false, 8),
        (0xC0, true, 20),
    ];

    for &(opcode, taken, expected_cycles) in &cases {
        let (mut cpu, mut memory) = cpu_at(opcode);
        memory.write(0xC001, 0x02);
        memory.write(0xC002, 0xC1);
        cpu.sp = 0xC1FE;

        match opcode {
            0x20 | 0xC2 | 0xC4 | 0xC0 => {
                cpu.f = if taken { 0x00 } else { 0x80 };
            }
            _ => unreachable!(),
        }

        if taken && opcode == 0xC0 {
            memory.write(0xC1FE, 0x78);
            memory.write(0xC1FF, 0x56);
        }

        let cycles = cpu.step(&mut memory);
        assert_eq!(cycles, expected_cycles, "opcode {:02X} taken={}", opcode, taken);
    }
}

#[test]
fn cpu_cycles_forward_exactly_to_timer() {
    let mut memory = memory();
    let mut cpu = Cpu::new();
    cpu.pc = 0xC000;

    // 64 NOPs = 64 * 4 T-cycles = 256 T-cycles.
    // DIV must therefore advance by exactly one.
    for offset in 0..64u16 {
        memory.write(0xC000 + offset, 0x00);
    }

    assert_eq!(memory.read(0xFF04), 0x00);

    let mut total_cycles = 0u32;
    for _ in 0..64 {
        let cycles = cpu.step(&mut memory);
        assert_eq!(cycles, 4);
        total_cycles += cycles;
        memory.step(cycles);
    }

    assert_eq!(total_cycles, 256);
    assert_eq!(memory.read(0xFF04), 0x01);
}

#[test]
fn cpu_cycles_forward_exactly_to_ppu_scanline_timing() {
    let mut memory = memory();
    let mut cpu = Cpu::new();
    cpu.pc = 0xC000;

    // 114 NOPs = 114 * 4 = 456 T-cycles, exactly one visible scanline.
    for offset in 0..114u16 {
        memory.write(0xC000 + offset, 0x00);
    }

    assert_eq!(memory.read(0xFF44), 0x00);

    let mut total_cycles = 0u32;
    for _ in 0..114 {
        let cycles = cpu.step(&mut memory);
        assert_eq!(cycles, 4);
        total_cycles += cycles;
        memory.step(cycles);
    }

    assert_eq!(total_cycles, 456);
    assert_eq!(memory.read(0xFF44), 0x01);
}

#[test]
fn interrupt_entry_has_fixed_20_cycle_latency() {
    let (mut cpu, mut memory) = cpu_at(0x00);
    cpu.ime = true;

    memory.write(0xFFFF, 0x01);
    memory.write(0xFF0F, 0x01);

    let cycles = cpu.step(&mut memory);

    assert_eq!(cycles, 20);
    assert_eq!(cpu.pc, 0x0040);
    assert_eq!(cpu.sp, 0xC1FE);
}

#[test]
fn cumulative_timing_of_call_and_return_is_exact() {
    let (mut cpu, mut memory) = cpu_at(0xCD);
    memory.write(0xC001, 0x00);
    memory.write(0xC002, 0xC3);
    memory.write(0xC300, 0xC9);
    cpu.sp = 0xC200;

    let call_cycles = cpu.step(&mut memory);
    assert_eq!(call_cycles, 24);
    assert_eq!(cpu.pc, 0xC300);

    let ret_cycles = cpu.step(&mut memory);
    assert_eq!(ret_cycles, 16);
    assert_eq!(cpu.pc, 0xC003);

    assert_eq!(call_cycles + ret_cycles, 40);
}
