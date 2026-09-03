use crate::apu::Apu;
use crate::audio::Audio;
use crate::rom::{Mbc1, Rom};
use crate::save::{load_sram, save_sram};

pub struct Bus {
    pub rom: Rom,
    pub mbc1: Mbc1,

    pub vram: [u8; 0x2000],
    pub wram: [u8; 0x2000],
    pub oam: [u8; 0xA0],
    pub hram: [u8; 0x7F],
    pub io: [u8; 0x80],

    pub joyp: u8,
    pub buttons: u8,

    pub ly: u8,
    pub ie: u8,
    pub if_reg: u8,
    pub lyc: u8,
    pub ppu_mode: u8,
    pub lcd_cycles: u32,

    pub lcdc: u8,
    pub stat: u8,
    pub scy: u8,
    pub scx: u8,
    pub bgp: u8,
    pub obp0: u8,
    pub obp1: u8,
    pub wy: u8,
    pub wx: u8,
    pub dma: u8,

    pub div: u8,
    pub tima: u8,
    pub tma: u8,
    pub tac: u8,
    pub(crate) div_cycles: u32,
    pub(crate) tima_cycles: u32,

    pub apu: Apu,

    pub debug_frames: u64,
}

impl Bus {
    pub fn new(path: &str) -> Self {
        let rom = Rom::load(path).expect("Nie można wczytać ROM-u");
        let mbc1 = Mbc1::new(&rom);

        Self {
            rom,
            mbc1,

            vram: [0; 0x2000],
            wram: [0; 0x2000],
            oam: [0; 0xA0],
            hram: [0; 0x7F],
            io: [0; 0x80],

            joyp: 0x30,
            buttons: 0xFF,

            ly: 0,
            lyc: 0,
            ppu_mode: 2,

            ie: 0,
            if_reg: 0xE1,

            lcdc: 0x91,
            lcd_cycles: 0,
            stat: 0x85,
            scy: 0,
            scx: 0,
            bgp: 0xFC,
            obp0: 0xFF,
            obp1: 0xFF,
            wy: 0,
            wx: 0,
            dma: 0xFF,

            div: 0xAB,
            tima: 0,
            tma: 0,
            tac: 0xF8,
            div_cycles: 0,
            tima_cycles: 0,

            apu: Apu::new(),

            debug_frames: 0,
        }
    }

    pub fn step(&mut self, cycles: u32, buffer: &mut [u32]) -> bool {
        self.step_timer(cycles);
        self.apu.step(cycles);
        self.step_lcd(cycles, buffer)
    }

    fn step_timer(&mut self, cycles: u32) {
        self.div_cycles += cycles;
        while self.div_cycles >= 256 {
            self.div_cycles -= 256;
            self.div = self.div.wrapping_add(1);
        }

        if self.tac & 0x04 == 0 {
            return;
        }

        let period = match self.tac & 0x03 {
            0 => 1024,
            1 => 16,
            2 => 64,
            _ => 256,
        };

        self.tima_cycles += cycles;
        while self.tima_cycles >= period {
            self.tima_cycles -= period;
            let (new_tima, overflow) = self.tima.overflowing_add(1);
            if overflow {
                self.tima = self.tma;
                self.if_reg |= 0x04;
            } else {
                self.tima = new_tima;
            }
        }
    }

    pub fn step_lcd(&mut self, cycles: u32, buffer: &mut [u32]) -> bool {
        if self.lcdc & 0x80 == 0 {
            self.ly = 0;
            self.lcd_cycles = 0;
            self.ppu_mode = 0;
            self.update_stat();
            return false;
        }

        self.lcd_cycles += cycles;

        let mut frame_ready = false;

        loop {
            let mode_length = match self.ppu_mode {
                2 => 80,
                3 => 172,
                0 => 204,
                1 => 456,
                _ => 456,
            };

            if self.lcd_cycles < mode_length {
                break;
            }

            self.lcd_cycles -= mode_length;

            match self.ppu_mode {
                // OAM scan
                2 => {
                    self.ppu_mode = 3;
                }

                // Drawing
                3 => {
                    if self.ly < 144 {
                        self.render_scanline(self.ly as usize, buffer);
                    }

                    self.ppu_mode = 0;
                }

                // H-Blank
                0 => {
                    self.ly = self.ly.wrapping_add(1);

                    if self.ly == 144 {
                        // V-Blank
                        self.ppu_mode = 1;
                        self.if_reg |= 0x01;

                        frame_ready = true;

                        self.debug_frames += 1;
                    } else {
                        self.ppu_mode = 2;
                    }
                }

                // V-Blank
                1 => {
                    self.ly = self.ly.wrapping_add(1);

                    if self.ly > 153 {
                        self.ly = 0;
                        self.ppu_mode = 2;
                    }
                }

                _ => {
                    self.ly = 0;
                    self.ppu_mode = 2;
                }
            }

            self.update_stat();
        }

        frame_ready
    }

    fn update_stat(&mut self) {
        // Zapamiętujemy poprzedni stan źródła przerwania STAT.
        let old_stat_irq = {
            let coincidence = self.ly == self.lyc;

            (self.stat & 0x40 != 0 && coincidence)
                || (self.stat & 0x20 != 0 && self.ppu_mode == 2)
                || (self.stat & 0x10 != 0 && self.ppu_mode == 1)
                || (self.stat & 0x08 != 0 && self.ppu_mode == 0)
        };

        // Zachowujemy tylko bity konfiguracyjne STAT:
        // bit 6 = LYC=LY interrupt
        // bit 5 = Mode 2 interrupt
        // bit 4 = Mode 1 interrupt
        // bit 3 = Mode 0 interrupt
        let interrupt_enable = self.stat & 0x78;

        let coincidence = self.ly == self.lyc;

        self.stat = 0x80 | interrupt_enable | (self.ppu_mode & 0x03);

        if coincidence {
            self.stat |= 0x04;
        }

        // Obliczamy nowy stan źródła IRQ STAT.
        let new_stat_irq = {
            (self.stat & 0x40 != 0 && coincidence)
                || (self.stat & 0x20 != 0 && self.ppu_mode == 2)
                || (self.stat & 0x10 != 0 && self.ppu_mode == 1)
                || (self.stat & 0x08 != 0 && self.ppu_mode == 0)
        };

        // STAT IRQ generuje się przy przejściu:
        // false -> true
        if new_stat_irq && !old_stat_irq {
            self.if_reg |= 0x02;
        }
    }

    fn joyp_value(&self) -> u8 {
        let mut low = 0x0F;
        let action = (self.buttons >> 4) & 0x0F;
        let direction = self.buttons & 0x0F;

        // P14 (bit 4) = 0 -> kierunki
        if self.joyp & 0x10 == 0 {
            low &= direction;
        }

        // P15 (bit 5) = 0 -> A/B/Select/Start
        if self.joyp & 0x20 == 0 {
            low &= action;
        }

        0xC0 | (self.joyp & 0x30) | low
    }

    pub fn set_buttons(&mut self, buttons: u8) {
        let pressed = self.buttons & !buttons;
        self.buttons = buttons;
        if pressed != 0 {
            self.if_reg |= 0x10;
        }
    }

    fn do_dma(&mut self, value: u8) {
        self.dma = value;
        let src = (value as u16) << 8;
        for i in 0..0xA0u16 {
            let byte = self.read_raw(src.wrapping_add(i));
            self.oam[i as usize] = byte;
        }
    }

    fn read_raw(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x7FFF => self.mbc1.read(&self.rom, address),
            0x8000..=0x9FFF => self.vram[(address - 0x8000) as usize],
            0xA000..=0xBFFF => self.mbc1.read_ram(address),
            0xC000..=0xDFFF => self.wram[(address - 0xC000) as usize],
            0xE000..=0xFDFF => self.wram[(address - 0xE000) as usize],
            0xFE00..=0xFE9F => self.oam[(address - 0xFE00) as usize],
            0xFF80..=0xFFFE => self.hram[(address - 0xFF80) as usize],
            _ => 0xFF,
        }
    }

    pub fn read(&mut self, address: u16) -> u8 {
        match address {
            0x0000..=0x7FFF => self.mbc1.read(&self.rom, address),
            0x8000..=0x9FFF => self.vram[(address - 0x8000) as usize],
            0xA000..=0xBFFF => self.mbc1.read_ram(address),
            0xC000..=0xDFFF => self.wram[(address - 0xC000) as usize],
            0xE000..=0xFDFF => self.wram[(address - 0xE000) as usize],
            0xFE00..=0xFE9F => self.oam[(address - 0xFE00) as usize],
            0xFF00 => self.joyp_value(),
            0xFF04 => self.div,
            0xFF05 => self.tima,
            0xFF06 => self.tma,
            0xFF07 => self.tac | 0xF8,
            0xFF0F => self.if_reg | 0xE0,
            0xFF10..=0xFF3F => self.apu.read(address),
            0xFF40 => self.lcdc,
            0xFF41 => {
                self.update_stat();
                self.stat | 0x80
            }
            0xFF42 => self.scy,
            0xFF43 => self.scx,
            0xFF44 => self.ly,
            0xFF45 => self.lyc,
            0xFF46 => self.dma,
            0xFF47 => self.bgp,
            0xFF48 => self.obp0,
            0xFF49 => self.obp1,
            0xFF4A => self.wy,
            0xFF4B => self.wx,
            0xFF01..=0xFF7F => self.io[(address - 0xFF00) as usize],
            0xFF80..=0xFFFE => self.hram[(address - 0xFF80) as usize],
            0xFFFF => self.ie,
            _ => 0xFF,
        }
    }

    pub fn read_debug(&self, address: u16) -> u8 {
        match address {
            // ROM
            0x0000..=0x7FFF => {
                self.mbc1.read(&self.rom, address)
            }

            // VRAM
            0x8000..=0x9FFF => {
                self.vram[(address - 0x8000) as usize]
            }

            // Cartridge RAM
            0xA000..=0xBFFF => {
                self.mbc1.read_ram(address)
            }

            // WRAM
            0xC000..=0xDFFF => {
                self.wram[(address - 0xC000) as usize]
            }

            // Echo RAM
            0xE000..=0xFDFF => {
                self.wram[(address - 0xE000) as usize]
            }

            // OAM
            0xFE00..=0xFE9F => {
                self.oam[(address - 0xFE00) as usize]
            }

            // Unusable
            0xFEA0..=0xFEFF => 0xFF,

            // I/O
            0xFF00..=0xFF7F => {
                match address {
                    0xFF00 => self.joyp_value(),

                    0xFF04 => self.div,
                    0xFF05 => self.tima,
                    0xFF06 => self.tma,
                    0xFF07 => self.tac | 0xF8,
                    0xFF0F => self.if_reg | 0xE0,

                    0xFF10..=0xFF3F => {
                        self.apu.read(address)
                    }

                    0xFF40 => self.lcdc,
                    0xFF41 => self.stat | 0x80,
                    0xFF42 => self.scy,
                    0xFF43 => self.scx,
                    0xFF44 => self.ly,
                    0xFF45 => self.lyc,
                    0xFF46 => self.dma,
                    0xFF47 => self.bgp,
                    0xFF48 => self.obp0,
                    0xFF49 => self.obp1,
                    0xFF4A => self.wy,
                    0xFF4B => self.wx,

                    _ => self.io[(address - 0xFF00) as usize],
                }
            }

            // HRAM
            0xFF80..=0xFFFE => {
                self.hram[(address - 0xFF80) as usize]
            }

            // IE
            0xFFFF => self.ie,

            _ => 0xFF,
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x1FFF => self.mbc1.write_ram_enable(value),
            0x2000..=0x3FFF => self.mbc1.select_rom_bank(value),
            0x4000..=0x5FFF => self.mbc1.select_ram_bank(value),
            0x6000..=0x7FFF => self.mbc1.select_banking_mode(value),
            0x8000..=0x9FFF => {
                self.vram[(address - 0x8000) as usize] = value;
            }
            0xA000..=0xBFFF => self.mbc1.write_ram(address, value),
            0xC000..=0xDFFF => {
                self.wram[(address - 0xC000) as usize] = value;
            }
            0xE000..=0xFDFF => {
                self.wram[(address - 0xE000) as usize] = value;
            }
            0xFE00..=0xFE9F => {
                self.oam[(address - 0xFE00) as usize] = value;
            }
            0xFEA0..=0xFEFF => {}
            0xFF00 => {
                self.joyp = value & 0x30;
            }
            0xFF04 => {
                self.div = 0;
                self.div_cycles = 0;
            }
            0xFF05 => {
                self.tima = value;
            }
            0xFF06 => {
                self.tma = value;
            }
            0xFF07 => {
                self.tac = value | 0xF8;
            }
            0xFF0F => {
                self.if_reg = value & 0x1F;
            }
            0xFF10..=0xFF3F => self.apu.write(address, value),
            0xFF40 => {
                let old_lcd_on = self.lcdc & 0x80 != 0;
                let new_lcd_on = value & 0x80 != 0;
                self.lcdc = value;

                if !old_lcd_on && new_lcd_on {
                    self.ly = 0;
                    self.lcd_cycles = 0;
                    self.ppu_mode = 2;
                    self.update_stat();
                }

                if old_lcd_on && !new_lcd_on {
                    self.ly = 0;
                    self.lcd_cycles = 0;
                    self.ppu_mode = 0;
                    self.update_stat();
                }
            }
            0xFF41 => {
                self.stat = 0x80 | (value & 0x78) | (self.stat & 0x07);
                self.update_stat();
            }
            0xFF42 => self.scy = value,
            0xFF43 => self.scx = value,
            0xFF44 => {}
            0xFF45 => {
                self.lyc = value;
                self.update_stat();
            }
            0xFF46 => self.do_dma(value),
            0xFF47 => self.bgp = value,
            0xFF48 => self.obp0 = value,
            0xFF49 => self.obp1 = value,
            0xFF4A => self.wy = value,
            0xFF4B => self.wx = value,
            0xFF01..=0xFF7F => {
                self.io[(address - 0xFF00) as usize] = value;
            }
            0xFF80..=0xFFFE => {
                self.hram[(address - 0xFF80) as usize] = value;
            }
            0xFFFF => {
                self.ie = value;
            }
        }
    }

    fn palette_gray(palette: u8, color_id: u8) -> u8 {
        match (palette >> (color_id * 2)) & 0x03 {
            0 => 0xFF,
            1 => 0xAA,
            2 => 0x55,
            _ => 0x00,
        }
    }

    fn rgb(gray: u8) -> u32 {
        ((gray as u32) << 16) | ((gray as u32) << 8) | gray as u32
    }

    fn tile_pixel(&self, tile_number: u8, row: usize, bit: usize, unsigned_tile_data: bool) -> u8 {
        let tile_address = if unsigned_tile_data {
            (tile_number as usize) * 16
        } else {
            let tile = tile_number as i8;
            (0x1000isize + (tile as isize) * 16) as usize
        };

        let address = tile_address + row * 2;
        if address + 1 >= self.vram.len() {
            return 0;
        }

        let low = self.vram[address];
        let high = self.vram[address + 1];
        ((high >> bit) & 1) << 1 | ((low >> bit) & 1)
    }

    fn render_scanline(&self, screen_y: usize, buffer: &mut [u32]) {
        const WIDTH: usize = 160;
        let mut color_ids = [0u8; WIDTH];

        let bg_enabled = self.lcdc & 0x01 != 0;
        let unsigned_tile_data = self.lcdc & 0x10 != 0;

        if bg_enabled {
            let tile_map_base = if self.lcdc & 0x08 != 0 {
                0x1C00
            } else {
                0x1800
            };

            for screen_x in 0..WIDTH {
                let bg_x = (screen_x + self.scx as usize) & 0xFF;
                let bg_y = (screen_y + self.scy as usize) & 0xFF;
                let tile_number = self.vram[tile_map_base + (bg_y / 8) * 32 + (bg_x / 8)];
                let bit = 7 - (bg_x & 7);
                color_ids[screen_x] =
                    self.tile_pixel(tile_number, bg_y & 7, bit, unsigned_tile_data);
            }
        }

        let window_enabled = self.lcdc & 0x20 != 0 && bg_enabled && screen_y as u8 >= self.wy;
        if window_enabled {
            let tile_map_base = if self.lcdc & 0x40 != 0 {
                0x1C00
            } else {
                0x1800
            };
            let window_y = screen_y - self.wy as usize;
            let window_x_start = self.wx as i32 - 7;

            for screen_x in 0..WIDTH {
                let window_x = screen_x as i32 - window_x_start;
                if window_x < 0 {
                    continue;
                }
                let window_x = window_x as usize;
                let tile_number =
                    self.vram[tile_map_base + ((window_y / 8) & 31) * 32 + ((window_x / 8) & 31)];
                let bit = 7 - (window_x & 7);
                color_ids[screen_x] =
                    self.tile_pixel(tile_number, window_y & 7, bit, unsigned_tile_data);
            }
        }

        for screen_x in 0..WIDTH {
            let gray = if bg_enabled {
                Self::palette_gray(self.bgp, color_ids[screen_x])
            } else {
                0xFF
            };
            buffer[screen_y * WIDTH + screen_x] = Self::rgb(gray);
        }

        if self.lcdc & 0x02 != 0 {
            self.render_sprites_line(screen_y, buffer, &color_ids);
        }
    }

    fn render_sprites_line(&self, line: usize, buffer: &mut [u32], bg_colors: &[u8; 160]) {
        const WIDTH: usize = 160;
        let sprite_height = if self.lcdc & 0x04 != 0 { 16 } else { 8 };

        for index in (0..40).rev() {
            let base = index * 4;
            let y = self.oam[base] as i32;
            let x = self.oam[base + 1] as i32;
            let tile_index = self.oam[base + 2];
            let attrs = self.oam[base + 3];

            let sprite_y = y - 16;
            let sprite_x = x - 8;
            if sprite_y > line as i32 || sprite_y + sprite_height as i32 <= line as i32 {
                continue;
            }

            let palette = if attrs & 0x10 != 0 {
                self.obp1
            } else {
                self.obp0
            };
            let flip_y = attrs & 0x40 != 0;
            let flip_x = attrs & 0x20 != 0;
            let behind_bg = attrs & 0x80 != 0;

            let row = (line as i32 - sprite_y) as usize;
            let sprite_row = if flip_y { sprite_height - 1 - row } else { row };

            if sprite_row >= sprite_height {
                continue;
            }

            let tile_id = if sprite_height == 16 {
                (tile_index & 0xFE).wrapping_add(if sprite_row >= 8 { 1 } else { 0 })
            } else {
                tile_index
            };

            let tile_offset = (tile_id as usize) * 16;
            let pixel_row = sprite_row % 8;
            let low = self.vram[tile_offset + pixel_row * 2];
            let high = self.vram[tile_offset + pixel_row * 2 + 1];

            for col in 0..8 {
                let screen_x = sprite_x + col as i32;
                if screen_x < 0 || screen_x >= WIDTH as i32 {
                    continue;
                }

                let sprite_col = if flip_x { 7 - col } else { col };
                let bit = 7 - sprite_col;
                let color_id = ((high >> bit) & 1) << 1 | ((low >> bit) & 1);
                if color_id == 0 {
                    continue;
                }

                let pixel_index = line * WIDTH + screen_x as usize;
                if behind_bg && bg_colors[screen_x as usize] != 0 {
                    continue;
                }

                let gray = Self::palette_gray(palette, color_id);
                buffer[pixel_index] = Self::rgb(gray);
            }
        }
    }

    pub fn render_tile_debug(&self, buffer: &mut [u32]) {
        const TILES_PER_ROW: usize = 16;
        const TILE_SIZE: usize = 8;
        const TILE_COUNT: usize = 384;
        const WIDTH: usize = TILES_PER_ROW * TILE_SIZE;
        const HEIGHT: usize = (TILE_COUNT / TILES_PER_ROW) * TILE_SIZE;

        // Wyczyść cały bufor.
        buffer.fill(0x00FFFFFF);

        for tile in 0..TILE_COUNT {
            let tile_addr = tile * 16;

            // 384 tile × 16 bajtów = dokładnie 0x1800 bajtów VRAM.
            if tile_addr + 15 >= 0x1800 {
                break;
            }

            let tile_x = tile % TILES_PER_ROW;
            let tile_y = tile / TILES_PER_ROW;

            for row in 0..8 {
                let low = self.vram[tile_addr + row * 2];
                let high = self.vram[tile_addr + row * 2 + 1];

                for col in 0..8 {
                    let bit = 7 - col;

                    let color_id =
                        (((high >> bit) & 1) << 1) |
                        ((low >> bit) & 1);

                    // Game Boy:
                    // 0 = biały
                    // 1 = jasnoszary
                    // 2 = ciemnoszary
                    // 3 = czarny
                    let gray = match color_id {
                        0 => 0xFF,
                        1 => 0xAA,
                        2 => 0x55,
                        3 => 0x00,
                        _ => unreachable!(),
                    };

                    let pixel =
                        ((gray as u32) << 16) |
                        ((gray as u32) << 8) |
                        gray as u32;

                    let x = tile_x * TILE_SIZE + col;
                    let y = tile_y * TILE_SIZE + row;

                    if x < WIDTH && y < HEIGHT {
                        buffer[y * WIDTH + x] = pixel;
                    }
                }
            }
        }
    }

    pub fn save_game(&self) {
        save_sram(self);
    }

    pub fn load_game(&mut self) {
        load_sram(self);
    }

    pub fn set_audio(&mut self, audio: Audio) {
        self.apu.set_audio(audio);
    }
}
