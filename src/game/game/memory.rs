use crate::game::hardware::ppu::Ppu;
use crate::game::hardware::timer::Timer;
use crate::game::hardware::interrupt::InterruptController;
use crate::rom::Cartridge;
use crate::game::hardware::serial::Serial;
use crate::game::hardware::joypad::Joypad;

/// CPU-visible memory map for the native Link's Awakening DX port.
///
/// Cartridge-owned regions are delegated to `Cartridge` while the internal
/// Game Boy memory regions are kept here. This gives the future CPU one
/// stable read/write interface instead of exposing individual memory arrays.
#[derive(Debug)]
pub struct GameMemory {
    cartridge: Cartridge,
    vram: [u8; 0x2000],
    wram: [u8; 0x8000],
    oam: [u8; 0x00A0],
    io: [u8; 0x0080],
    hram: [u8; 0x007F],
    interrupt: InterruptController,
    timer: Timer,
    ppu: Ppu,
    serial: Serial,
    joypad: Joypad,
}

impl GameMemory {
    pub fn new(cartridge: Cartridge) -> Self {
        Self {
            cartridge,
            vram: [0; 0x2000],
            wram: [0; 0x8000],
            oam: [0; 0x00A0],
            io: [0; 0x0080],
            hram: [0; 0x007F],
            interrupt: InterruptController::new(),
            timer: Timer::new(),
            ppu: Ppu::new(),
            serial: Serial::new(),
            joypad: Joypad::new(),
        }
    }

    pub fn cartridge(&self) -> &Cartridge {
        &self.cartridge
    }

    pub fn cartridge_mut(&mut self) -> &mut Cartridge {
        &mut self.cartridge
    }

    /// Reads one byte using the Game Boy CPU address map.
    pub fn read(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x7FFF | 0xA000..=0xBFFF => self.cartridge.read(address),
            0x8000..=0x9FFF => self.vram[(address - 0x8000) as usize],
            0xC000..=0xDFFF => self.wram[(address - 0xC000) as usize],
            0xE000..=0xFDFF => self.wram[(address - 0xE000) as usize],
            0xFE00..=0xFE9F => self.oam[(address - 0xFE00) as usize],
            0xFEA0..=0xFEFF => 0xFF,
            0xFF00 => self.joypad.read(),
            0xFF01..=0xFF02 => self.serial.read(address),
            0xFF40 | 0xFF41 | 0xFF42 | 0xFF43 | 0xFF44 | 0xFF45 | 0xFF47 => {
                self.ppu.read(address)
            }
            0xFF04..=0xFF07 => self.timer.read(address),
            0xFF0F => self.interrupt.read_if(),
            0xFF00..=0xFF03 | 0xFF08..=0xFF0E | 0xFF10..=0xFF7F => {
                self.io[(address - 0xFF00) as usize]
            }
            0xFF80..=0xFFFE => self.hram[(address - 0xFF80) as usize],
            0xFFFF => self.interrupt.read_ie(),
        }
    }

    /// Writes one byte using the Game Boy CPU address map.
    pub fn write(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x7FFF | 0xA000..=0xBFFF => self.cartridge.write(address, value),
            0x8000..=0x9FFF => self.vram[(address - 0x8000) as usize] = value,
            0xC000..=0xDFFF => self.wram[(address - 0xC000) as usize] = value,
            0xE000..=0xFDFF => self.wram[(address - 0xE000) as usize] = value,
            0xFE00..=0xFE9F => self.oam[(address - 0xFE00) as usize] = value,
            0xFEA0..=0xFEFF => {},
            0xFF40 | 0xFF41 | 0xFF42 | 0xFF43 | 0xFF45 | 0xFF47 => {
                self.ppu.write(address, value)
            }
            0xFF00 => self.joypad.write(value),
            0xFF01..=0xFF02 => {
                self.serial.write(address, value);
            }
            0xFF04..=0xFF07 => {
                self.timer.write(address, value);

                if self.timer.take_interrupt() {
                    self.interrupt.request(2);
                }
            }
            0xFF0F => self.interrupt.write_if(value),
            0xFF00..=0xFF03 | 0xFF08..=0xFF0E | 0xFF10..=0xFF7F => {
                self.io[(address - 0xFF00) as usize] = value;
            }
            0xFF80..=0xFFFE => self.hram[(address - 0xFF80) as usize] = value,
            0xFFFF => self.interrupt.write_ie(value),
        }
    }

    pub fn read_word(&self, address: u16) -> u16 {
        let lo = self.read(address);
        let hi = self.read(address.wrapping_add(1));
        u16::from_le_bytes([lo, hi])
    }

    pub fn write_word(&mut self, address: u16, value: u16) {
        let [lo, hi] = value.to_le_bytes();
        self.write(address, lo);
        self.write(address.wrapping_add(1), hi);
    }
    pub fn joypad_button_pressed(&mut self, button: u8) {
        self.joypad.button_pressed(button);
    }
    pub fn step(&mut self, cycles: u32) {
        self.timer.step(cycles);

        if self.timer.take_interrupt() {
            self.interrupt.request(2);
        }

        self.ppu.step(cycles);

        if self.ppu.take_vblank_interrupt() {
            self.interrupt.request(0);
        }

        if self.ppu.take_stat_interrupt() {
            self.interrupt.request(1);
        }

        self.serial.step(cycles);

        if self.serial.take_interrupt() {
            self.interrupt.request(3);
        }
        if self.joypad.take_interrupt() {
    self.interrupt.request(4);
}
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::cpu::Cpu;
    use super::GameMemory;
    use crate::rom::{Cartridge, Rom};

    fn test_memory() -> GameMemory {
    use crate::rom::{Cartridge, Rom};

    let rom = Rom::load(
        "Legend of Zelda, The - Link's Awakening DX (USA, Europe) (Rev 2).gbc",
    )
    .expect("Nie można załadować ROM-u testowego");

    GameMemory::new(Cartridge::new(rom))
}

    #[test]
    fn timer_overflow_sets_timer_interrupt_flag() {
        let mut memory = test_memory();

        memory.write(0xFF05, 0xFF);
        memory.write(0xFF06, 0x42);
        memory.write(0xFF07, 0x05);

        memory.step(16);

        assert_eq!(memory.read(0xFF05), 0x00);

        memory.step(4);

        assert_eq!(memory.read(0xFF05), 0x42);
        assert_eq!(memory.read(0xFF0F) & 0x04, 0x04);
    }
    #[test]
fn ppu_vblank_sets_vblank_interrupt_flag() {
    let mut memory = test_memory();

    memory.step(456 * 144);

    assert_eq!(memory.read(0xFF44), 144);
    assert_eq!(memory.read(0xFF0F) & 0x01, 0x01);
}
#[test]
fn interrupt_controller_handles_ie_and_if() {
    let mut memory = test_memory();

    memory.write(0xFFFF, 0x05);
    memory.write(0xFF0F, 0x05);

    assert_eq!(memory.read(0xFFFF), 0x05);
    assert_eq!(memory.read(0xFF0F) & 0x1F, 0x05);
}
#[test]
fn timer_requests_interrupt_through_interrupt_controller() {
    let mut memory = test_memory();

    memory.write(0xFF05, 0xFF);
    memory.write(0xFF06, 0x42);
    memory.write(0xFF07, 0x05);

    memory.step(16);
    memory.step(4);

    assert_eq!(memory.read(0xFF05), 0x42);
    assert_eq!(memory.read(0xFF0F) & 0x04, 0x04);
}
#[test]
fn ppu_stat_sets_stat_interrupt_flag() {
    let mut memory = test_memory();

    // Włącz przerwanie STAT dla OAM (Mode 2).
    memory.write(0xFF41, 0x20);

    assert_eq!(memory.read(0xFF41) & 0x20, 0x20);

    // PPU startuje w Mode 2, więc STAT IRQ powinno zostać zgłoszone.
    memory.step(1);

    assert_eq!(memory.read(0xFF0F) & 0x02, 0x02);
}
#[test]
fn stat_interrupt_jumps_to_0048() {
    let mut memory = test_memory();
    let mut cpu = Cpu::new();

    // Włącz LCD STAT interrupt w IE.
    memory.write(0xFFFF, 0x02);

    // Włącz OAM STAT interrupt (mode 2).
    memory.write(0xFF41, 0x20);

    // PPU startuje w mode 2, więc połączenie powinno
    // wygenerować żądanie STAT.
    memory.step(1);

    assert_eq!(memory.read(0xFF0F) & 0x02, 0x02);

    cpu.ime = true;
    cpu.pc = 0x1234;
    cpu.sp = 0xFFFE;

    let cycles = cpu.step(&mut memory);

    assert_eq!(cycles, 20);
    assert_eq!(cpu.pc, 0x0048);
    assert!(!cpu.ime);

    // STAT IF.1 powinien zostać wyczyszczony.
    assert_eq!(memory.read(0xFF0F) & 0x02, 0);

    // CPU powinien zapisać poprzedni PC na stosie.
    assert_eq!(memory.read(0xFFFC), 0x34);
    assert_eq!(memory.read(0xFFFD), 0x12);
}
#[test]
fn lcdc_read_write() {
    let mut ppu = Ppu::new();

    assert_eq!(ppu.read(0xFF40), 0x91);

    ppu.write(0xFF40, 0xC7);

    assert_eq!(ppu.read(0xFF40), 0xC7);
}
#[test]
fn lcdc_bits_are_preserved() {
    let mut ppu = Ppu::new();

    ppu.write(0xFF40, 0x00);
    assert_eq!(ppu.read(0xFF40), 0x00);

    ppu.write(0xFF40, 0xFF);
    assert_eq!(ppu.read(0xFF40), 0xFF);
}
#[test]
fn scroll_registers_affect_ppu_state() {
    let mut memory = test_memory();

    memory.write(0xFF42, 32);
    memory.write(0xFF43, 64);

    assert_eq!(memory.read(0xFF42), 32);
    assert_eq!(memory.read(0xFF43), 64);
}
}