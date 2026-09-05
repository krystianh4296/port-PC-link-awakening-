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
fn halt_with_ime_enabled_services_pending_interrupt() {
    let (mut cpu, mut memory) = cpu_at(0x76);
    cpu.ime = true;

    assert_eq!(cpu.step(&mut memory), 4);
    assert!(cpu.halted);
    assert_eq!(cpu.pc, 0xC001);

    memory.write(0xFFFF, 0x01); // VBlank enable
    memory.write(0xFF0F, 0x01); // VBlank request

    assert_eq!(cpu.step(&mut memory), 20);
    assert!(!cpu.halted);
    assert!(!cpu.ime);
    assert_eq!(cpu.pc, 0x0040);
    assert_eq!(cpu.sp, 0xC1FE);
    assert_eq!(memory.read(0xC1FE), 0x01);
    assert_eq!(memory.read(0xC1FF), 0xC0);
    assert_eq!(memory.read(0xFF0F) & 0x01, 0);
}

#[test]
fn halt_with_ime_disabled_wakes_without_servicing_interrupt() {
    let (mut cpu, mut memory) = cpu_at(0x76);
    cpu.ime = false;

    assert_eq!(cpu.step(&mut memory), 4);
    assert!(cpu.halted);
    assert_eq!(cpu.pc, 0xC001);

    memory.write(0xFFFF, 0x01);
    memory.write(0xFF0F, 0x01);

    assert_eq!(cpu.step(&mut memory), 4);
    assert!(!cpu.halted);
    assert!(!cpu.ime);
    assert_eq!(cpu.pc, 0xC001);
    assert_eq!(memory.read(0xFF0F) & 0x01, 0x01);
}

#[test]
fn halt_bug_does_not_advance_pc_before_next_instruction() {
    let (mut cpu, mut memory) = cpu_at(0x76);
    cpu.ime = false;
    memory.write(0xC001, 0x3E); // LD A,d8
    memory.write(0xFFFF, 0x01);
    memory.write(0xFF0F, 0x01);

    assert_eq!(cpu.step(&mut memory), 4);
    assert!(cpu.halt_bug);
    assert!(!cpu.halted);
    assert_eq!(cpu.pc, 0xC001);

    // HALT bug: the opcode at C001 is fetched without advancing PC first.
    // LD A,d8 therefore reads its immediate byte from C001 as well.
    assert_eq!(cpu.step(&mut memory), 8);
    assert_eq!(cpu.a, 0x3E);
    assert_eq!(cpu.pc, 0xC002);
    assert!(!cpu.halt_bug);
}

#[test]
fn interrupt_priority_selects_lowest_pending_bit() {
    let (mut cpu, mut memory) = cpu_at(0x00);
    cpu.ime = true;

    memory.write(0xFFFF, 0x1F);
    memory.write(0xFF0F, 0x1F);

    assert_eq!(cpu.step(&mut memory), 20);
    assert_eq!(cpu.pc, 0x0040);
    assert_eq!(memory.read(0xFF0F), 0x1E);
    assert!(!cpu.ime);
}

#[test]
fn interrupt_service_clears_only_selected_if_bit() {
    let (mut cpu, mut memory) = cpu_at(0x00);
    cpu.ime = true;

    memory.write(0xFFFF, 0x1F);
    memory.write(0xFF0F, 0x06); // STAT + Timer

    assert_eq!(cpu.step(&mut memory), 20);
    assert_eq!(cpu.pc, 0x0048); // STAT has priority over Timer
    assert_eq!(memory.read(0xFF0F), 0x04);
}

#[test]
fn ei_enables_ime_after_exactly_one_following_instruction() {
    let (mut cpu, mut memory) = cpu_at(0xFB); // EI
    cpu.ime = false;

    memory.write(0xC001, 0x00); // NOP
    memory.write(0xC002, 0x00); // NOP

    assert_eq!(cpu.step(&mut memory), 4);
    assert!(!cpu.ime);
    assert!(cpu.ime_pending);
    assert_eq!(cpu.pc, 0xC001);

    assert_eq!(cpu.step(&mut memory), 4);
    assert!(cpu.ime);
    assert!(!cpu.ime_pending);
    assert_eq!(cpu.pc, 0xC002);

    assert_eq!(cpu.step(&mut memory), 4);
    assert!(cpu.ime);
    assert_eq!(cpu.pc, 0xC003);
}

#[test]
fn di_cancels_pending_ei_and_disables_ime() {
    let (mut cpu, mut memory) = cpu_at(0xFB); // EI
    cpu.ime = false;
    memory.write(0xC001, 0xF3); // DI
    memory.write(0xC002, 0x00); // NOP

    assert_eq!(cpu.step(&mut memory), 4);
    assert!(cpu.ime_pending);

    assert_eq!(cpu.step(&mut memory), 4);
    assert!(!cpu.ime);
    assert!(!cpu.ime_pending);

    assert_eq!(cpu.step(&mut memory), 4);
    assert!(!cpu.ime);
    assert_eq!(cpu.pc, 0xC003);
}

#[test]
fn reti_restores_pc_and_enables_ime_immediately() {
    let (mut cpu, mut memory) = cpu_at(0xD9); // RETI
    cpu.ime = false;
    cpu.ime_pending = true;
    memory.write(0xC200, 0x34);
    memory.write(0xC201, 0x12);

    assert_eq!(cpu.step(&mut memory), 16);
    assert_eq!(cpu.pc, 0x1234);
    assert_eq!(cpu.sp, 0xC202);
    assert!(cpu.ime);
    assert!(!cpu.ime_pending);
}

#[test]
fn stop_consumes_second_byte_and_enters_stopped_state() {
    let (mut cpu, mut memory) = cpu_at(0x10); // STOP 00
    memory.write(0xC001, 0x00);

    assert_eq!(cpu.step(&mut memory), 4);
    assert!(cpu.halted);
    assert_eq!(cpu.pc, 0xC002);
}
