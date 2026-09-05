use crate::game::hardware::interrupt::InterruptController;
use crate::game::hardware::joypad::Joypad;
use crate::game::hardware::ppu::Ppu;
use crate::game::hardware::serial::Serial;
use crate::game::hardware::timer::Timer;
use crate::rom::Cartridge;

#[derive(Debug)]
pub struct GameMemory {
    cartridge: Cartridge,
    vram_bank: u8,
    vram: [[u8; 0x2000]; 2],
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
            vram_bank: 0,
            vram: [[0; 0x2000]; 2],
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

    pub fn cartridge(&self) -> &Cartridge { &self.cartridge }
    pub fn cartridge_mut(&mut self) -> &mut Cartridge { &mut self.cartridge }

    pub fn framebuffer(&self) -> &[u32; 160 * 144] {
        self.ppu.framebuffer()
    }

    pub fn frame_ready(&self) -> bool {
        self.ppu.frame_ready()
    }

    pub fn take_frame_ready(&mut self) -> bool {
        self.ppu.take_frame_ready()
    }

    pub fn read(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x7FFF | 0xA000..=0xBFFF => self.cartridge.read(address),
            0x8000..=0x9FFF => {
                let bank = (self.vram_bank & 0x01) as usize;
                self.vram[bank][(address - 0x8000) as usize]
            }
            0xC000..=0xDFFF => self.wram[(address - 0xC000) as usize],
            0xE000..=0xFDFF => self.wram[(address - 0xE000) as usize],
            0xFE00..=0xFE9F => self.oam[(address - 0xFE00) as usize],
            0xFEA0..=0xFEFF => 0xFF,
            0xFF00 => self.joypad.read(),
            0xFF01..=0xFF02 => self.serial.read(address),
            0xFF04..=0xFF07 => self.timer.read(address),
            0xFF0F => self.interrupt.read_if(),
            0xFF40..=0xFF47 | 0xFF68 | 0xFF69 | 0xFF6A | 0xFF6B => self.ppu.read(address),
            0xFF4F => 0xFE | (self.vram_bank & 0x01),
            0xFF00..=0xFF03 | 0xFF08..=0xFF0E | 0xFF10..=0xFF3F |
            0xFF48..=0xFF4E | 0xFF50..=0xFF67 | 0xFF6C..=0xFF6F |
            0xFF70..=0xFF7F => self.io[(address - 0xFF00) as usize],
            0xFF80..=0xFFFE => self.hram[(address - 0xFF80) as usize],
            0xFFFF => self.interrupt.read_ie(),
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x7FFF | 0xA000..=0xBFFF => self.cartridge.write(address, value),
            0x8000..=0x9FFF => {
                let bank = (self.vram_bank & 0x01) as usize;
                self.vram[bank][(address - 0x8000) as usize] = value;
            }
            0xC000..=0xDFFF => self.wram[(address - 0xC000) as usize] = value,
            0xE000..=0xFDFF => self.wram[(address - 0xE000) as usize] = value,
            0xFE00..=0xFE9F => self.oam[(address - 0xFE00) as usize] = value,
            0xFEA0..=0xFEFF => {}
            0xFF00 => self.joypad.write(value),
            0xFF01..=0xFF02 => self.serial.write(address, value),
            0xFF04..=0xFF07 => {
                self.timer.write(address, value);
                if self.timer.take_interrupt() { self.interrupt.request(2); }
            }
            0xFF0F => self.interrupt.write_if(value),
            0xFF40..=0xFF47 | 0xFF68 | 0xFF69 | 0xFF6A | 0xFF6B => self.ppu.write(address, value),
            0xFF4F => self.vram_bank = value & 0x01,
            0xFF00..=0xFF03 | 0xFF08..=0xFF0E | 0xFF10..=0xFF3F |
            0xFF48..=0xFF4E | 0xFF50..=0xFF67 | 0xFF6C..=0xFF6F |
            0xFF70..=0xFF7F => self.io[(address - 0xFF00) as usize] = value,
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

    pub fn joypad_button_pressed(&mut self, button: u8) { self.joypad.button_pressed(button); }

    pub fn step(&mut self, cycles: u32) {
        self.timer.step(cycles);
        if self.timer.take_interrupt() { self.interrupt.request(2); }

        self.ppu.step(cycles, &self.oam, &self.vram[0], &self.vram[1]);
        if self.ppu.take_vblank_interrupt() { self.interrupt.request(0); }
        if self.ppu.take_stat_interrupt() { self.interrupt.request(1); }

        self.serial.step(cycles);
        if self.serial.take_interrupt() { self.interrupt.request(3); }
        if self.joypad.take_interrupt() { self.interrupt.request(4); }
    }

    pub fn background_tile_attributes(
        vram_bank_1: &[u8; 0x2000],
        bg_x: u8,
        bg_y: u8,
        map_base: u16,
    ) -> u8 {
        let tile_x = (bg_x / 8) as usize;
        let tile_y = (bg_y / 8) as usize;
        let map_offset = (map_base - 0x8000) as usize;
        vram_bank_1[map_offset + tile_y * 32 + tile_x]
    }
}
