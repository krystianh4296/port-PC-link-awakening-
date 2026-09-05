use crate::game::hardware::interrupt::InterruptController;
use crate::game::hardware::joypad::Joypad;
use crate::game::hardware::ppu::Ppu;
use crate::game::hardware::serial::Serial;
use crate::game::hardware::timer::Timer;
use crate::apu::Apu;
use crate::audio::Audio;
use crate::rom::Cartridge;

pub struct GameMemory {
    cartridge: Cartridge,
    vram_bank: u8,
    vram: [[u8; 0x2000]; 2],
    wram: [u8; 0x8000],
    wram_bank: u8,
    oam: [u8; 0x00A0],
    io: [u8; 0x0080],
    hram: [u8; 0x007F],
    interrupt: InterruptController,
    timer: Timer,
    ppu: Ppu,
    serial: Serial,
    joypad: Joypad,
    apu: Apu,
    cpu_pc: u16,
    vram_write_count: u64,
    vram_nonzero_write_count: u64,
    bg_map_write_count: u64,
    oam_write_count: u64,
    ppu_register_write_count: u64,
    vram_bank_write_count: u64,
    key1_prepare: bool,
    double_speed: bool,
    hdma: [u8; 5],
    graphics_write_trace_count: u16,
    vram_first_writes_logged: u8,
    last_vram_write: Option<(u16, u8, u8)>,
}

impl GameMemory {
    pub fn new(cartridge: Cartridge) -> Self {
        Self {
            cartridge,
            vram_bank: 0,
            vram: [[0; 0x2000]; 2],
            wram: [0; 0x8000],
            wram_bank: 1,
            oam: [0; 0x00A0],
            io: [0; 0x0080],
            hram: [0; 0x007F],
            interrupt: InterruptController::new(),
            timer: Timer::new(),
            ppu: Ppu::new(),
            serial: Serial::new(),
            joypad: Joypad::new(),
            apu: Apu::new(),
            cpu_pc: 0x0100,
            vram_write_count: 0,
            vram_nonzero_write_count: 0,
            bg_map_write_count: 0,
            oam_write_count: 0,
            ppu_register_write_count: 0,
            vram_bank_write_count: 0,
            key1_prepare: false,
            double_speed: false,
            hdma: [0; 5],
            graphics_write_trace_count: 0,
            vram_first_writes_logged: 0,
            last_vram_write: None,
        }
    }

    pub fn set_cpu_pc(&mut self, pc: u16) {
        self.cpu_pc = pc;
    }

    pub fn cartridge(&self) -> &Cartridge { &self.cartridge }
    pub fn cartridge_mut(&mut self) -> &mut Cartridge { &mut self.cartridge }
    pub fn framebuffer(&self) -> &[u32; 160 * 144] { self.ppu.framebuffer() }
    pub fn frame_ready(&self) -> bool { self.ppu.frame_ready() }
    pub fn take_frame_ready(&mut self) -> bool { self.ppu.take_frame_ready() }
    pub fn set_audio(&mut self, audio: Audio) { self.apu.set_audio(audio); }

    fn wram_index(&self, address: u16) -> usize {
        match address {
            0xC000..=0xCFFF => (address - 0xC000) as usize,
            0xD000..=0xDFFF => self.wram_bank as usize * 0x1000 + (address - 0xD000) as usize,
            0xE000..=0xEFFF => (address - 0xE000) as usize,
            0xF000..=0xFDFF => self.wram_bank as usize * 0x1000 + (address - 0xF000) as usize,
            _ => unreachable!("Adres poza WRAM: {address:04X}"),
        }
    }

    fn oam_dma(&mut self, source_high: u8) {
        let source = (source_high as u16) << 8;
        for offset in 0..0xA0u16 {
            self.oam[offset as usize] = self.read(source.wrapping_add(offset));
        }
    }

    fn run_vram_dma(&mut self, control: u8) {
        let source = u16::from_be_bytes([self.hdma[0], self.hdma[1] & 0xF0]);
        let destination = 0x8000 | (u16::from_be_bytes([self.hdma[2] & 0x1F, self.hdma[3] & 0xF0]) & 0x1FF0);
        let length = ((control & 0x7F) as usize + 1) * 0x10;
        let bank = (self.vram_bank & 1) as usize;
        for offset in 0..length {
            let value = self.read(source.wrapping_add(offset as u16));
            self.vram[bank][(destination as usize - 0x8000 + offset) & 0x1FFF] = value;
        }
        self.hdma[4] = 0xFF; // completed; HBlank transfers are completed eagerly for now
    }

    pub fn try_speed_switch(&mut self) -> bool {
        if !self.key1_prepare { return false; }
        self.double_speed = !self.double_speed;
        self.key1_prepare = false;
        true
    }

    pub fn read(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x7FFF | 0xA000..=0xBFFF => self.cartridge.read(address),
            0x8000..=0x9FFF => {
                let bank = (self.vram_bank & 0x01) as usize;
                self.vram[bank][(address - 0x8000) as usize]
            }
            0xC000..=0xDFFF | 0xE000..=0xFDFF => self.wram[self.wram_index(address)],
            0xFE00..=0xFE9F => self.oam[(address - 0xFE00) as usize],
            0xFEA0..=0xFEFF => 0xFF,
            0xFF00 => self.joypad.read(),
            0xFF01..=0xFF02 => self.serial.read(address),
            0xFF10..=0xFF3F => self.apu.read(address),
            0xFF04..=0xFF07 => self.timer.read(address),
            0xFF0F => self.interrupt.read_if(),
            0xFF40..=0xFF45 | 0xFF47 | 0xFF68 | 0xFF69 | 0xFF6A | 0xFF6B => self.ppu.read(address),
            0xFF4F => 0xFE | (self.vram_bank & 0x01),
            0xFF4D => 0x7E | (self.double_speed as u8) << 7 | self.key1_prepare as u8,
            0xFF46 => 0xFF,
            0xFF51..=0xFF55 => self.hdma[(address - 0xFF51) as usize],
            0xFF70 => 0xF8 | self.wram_bank,
            0xFF00..=0xFF03 | 0xFF08..=0xFF0E |
            0xFF48..=0xFF4E | 0xFF50..=0xFF67 | 0xFF6C..=0xFF6F |
            0xFF71..=0xFF7F => self.io[(address - 0xFF00) as usize],
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
                self.vram_write_count += 1;
                if value != 0 { self.vram_nonzero_write_count += 1; }
                if address >= 0x9800 { self.bg_map_write_count += 1; }
                self.last_vram_write = Some((address, value, bank as u8));

                self.graphics_write_trace_count = self.graphics_write_trace_count.saturating_add(1);
                self.vram_first_writes_logged = self.vram_first_writes_logged.saturating_add(1);
            }
            0xC000..=0xDFFF | 0xE000..=0xFDFF => { let index = self.wram_index(address); self.wram[index] = value; }
            0xFE00..=0xFE9F => {
                self.oam[(address - 0xFE00) as usize] = value;
                self.oam_write_count += 1;
                self.graphics_write_trace_count = self.graphics_write_trace_count.saturating_add(1);
            }
            0xFEA0..=0xFEFF => {}
            0xFF00 => self.joypad.write(value),
            0xFF01..=0xFF02 => self.serial.write(address, value),
            0xFF10..=0xFF3F => self.apu.write(address, value),
            0xFF04..=0xFF07 => {
                self.timer.write(address, value);
                if self.timer.take_interrupt() { self.interrupt.request(2); }
            }
            0xFF0F => self.interrupt.write_if(value),
            0xFF40..=0xFF45 | 0xFF47 | 0xFF68 | 0xFF69 | 0xFF6A | 0xFF6B => {
                self.ppu_register_write_count += 1;
                self.graphics_write_trace_count = self.graphics_write_trace_count.saturating_add(1);
                self.ppu.write(address, value);
            }
            0xFF4F => {
                self.vram_bank_write_count += 1;
                self.graphics_write_trace_count = self.graphics_write_trace_count.saturating_add(1);
                self.vram_bank = value & 0x01;
            }
            0xFF4D => self.key1_prepare = value & 1 != 0,
            0xFF46 => self.oam_dma(value),
            0xFF51..=0xFF54 => self.hdma[(address - 0xFF51) as usize] = value,
            0xFF55 => self.run_vram_dma(value),
            0xFF70 => self.wram_bank = (value & 7).max(1),
            0xFF00..=0xFF03 | 0xFF08..=0xFF0E |
            0xFF48..=0xFF4E | 0xFF50..=0xFF67 | 0xFF6C..=0xFF6F |
            0xFF71..=0xFF7F => self.io[(address - 0xFF00) as usize] = value,
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
    pub fn set_joypad_button(&mut self, button: u8, pressed: bool) { self.joypad.set_button(button, pressed); }

    pub fn step(&mut self, cycles: u32) {
        self.timer.step(cycles);
        self.apu.step(cycles);
        if self.timer.take_interrupt() { self.interrupt.request(2); }
        self.ppu.step(cycles, &self.oam, &self.vram[0], &self.vram[1]);
        if self.ppu.take_vblank_interrupt() { self.interrupt.request(0); }
        if self.ppu.take_stat_interrupt() { self.interrupt.request(1); }
        self.serial.step(cycles);
        if self.serial.take_interrupt() { self.interrupt.request(3); }
        if self.joypad.take_interrupt() { self.interrupt.request(4); }
    }

    pub fn background_tile_attributes(vram_bank_1: &[u8; 0x2000], bg_x: u8, bg_y: u8, map_base: u16) -> u8 {
        let tile_x = (bg_x / 8) as usize;
        let tile_y = (bg_y / 8) as usize;
        let map_offset = (map_base - 0x8000) as usize;
        vram_bank_1[map_offset + tile_y * 32 + tile_x]
    }

    pub fn print_vram_diagnostics(&self) {
        println!("=== VRAM DIAGNOSTICS ===");
        println!("VRAM WRITES: {}", self.vram_write_count);
        println!("VRAM NONZERO WRITES: {}", self.vram_nonzero_write_count);
        println!("BG MAP WRITES (9800-9FFF): {}", self.bg_map_write_count);
        println!("OAM WRITES: {}", self.oam_write_count);
        println!("PPU REGISTER WRITES (FF40-FF47, FF68-FF6B): {}", self.ppu_register_write_count);
        println!("VRAM BANK SELECT WRITES (FF4F): {}", self.vram_bank_write_count);
        match self.last_vram_write {
            Some((address, value, bank)) => println!("LAST VRAM WRITE: addr={:04X} value={:02X} bank={}", address, value, bank),
            None => println!("LAST VRAM WRITE: none"),
        }
        println!("========================");
    }
}
