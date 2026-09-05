#[derive(Debug, Clone, Copy)]
pub struct WindowScanline {
    pub pixels: [u32; 160],
    pub color_index: [u8; 160],
    pub priority: [bool; 160],
    pub visible: [bool; 160],
}

#[derive(Debug, Clone, Copy)]
pub struct BgWindowScanline {
    pub pixels: [u32; 160],
    pub color_index: [u8; 160],
    pub priority: [bool; 160],
    pub visible: [bool; 160],
}

#[derive(Debug, Clone, Copy)]
pub struct Sprite {
    pub y: u8,
    pub x: u8,
    pub tile: u8,
    pub flags: u8,
}

impl Sprite {
    pub fn priority(&self) -> bool {
        self.flags & 0x80 != 0
    }

    pub fn y_flip(&self) -> bool {
        self.flags & 0x40 != 0
    }

    pub fn x_flip(&self) -> bool {
        self.flags & 0x20 != 0
    }

    pub fn vram_bank(&self) -> usize {
        if self.flags & 0x08 != 0 { 1 } else { 0 }
    }

    pub fn cgb_palette(&self) -> usize {
        (self.flags & 0x07) as usize
    }
}

#[derive(Debug)]
pub struct Ppu {
    ly: u8,
    lyc: u8,
    lcdc: u8,
    stat: u8,
    scx: u8,
    scy: u8,
    wx: u8,
    wy: u8,
    window_line: u8,

    framebuffer: [u32; 160 * 144],
    frame_ready: bool,

    bg_palette_ram: [u8; 64],
    obj_palette_ram: [u8; 64],
    bgpi: u8,
    obpi: u8,

    cycle_counter: u32,
    mode: u8,

    vblank_interrupt: bool,
    stat_interrupt: bool,
    stat_irq_line: bool,
}

impl Ppu {
    pub fn new() -> Self {
        Self {
            ly: 0,
            lyc: 0,
            lcdc: 0x91,
            stat: 0x80,
            scx: 0,
            scy: 0,
            wx: 0,
            wy: 0,
            window_line: 0,
            framebuffer: [0xFF000000; 160 * 144],
            frame_ready: false,
            bg_palette_ram: [0; 64],
            obj_palette_ram: [0; 64],
            bgpi: 0,
            obpi: 0,
            cycle_counter: 0,
            mode: 2,
            vblank_interrupt: false,
            stat_interrupt: false,
            stat_irq_line: false,
        }
    }

    pub fn step(
        &mut self,
        cycles: u32,
        oam: &[u8; 0xA0],
        vram_bank_0: &[u8; 0x2000],
        vram_bank_1: &[u8; 0x2000],
    ) {
        if self.lcdc & 0x80 == 0 {
            return;
        }

        for _ in 0..cycles {
            self.step_cycle(oam, vram_bank_0, vram_bank_1);
        }
    }

    fn step_cycle(
        &mut self,
        oam: &[u8; 0xA0],
        vram_bank_0: &[u8; 0x2000],
        vram_bank_1: &[u8; 0x2000],
    ) {
        let _ = oam;
        self.cycle_counter += 1;

        match self.mode {
            2 => {
                if self.cycle_counter >= 80 {
                    self.cycle_counter = 0;
                    self.set_mode(3);
                }
            }
            3 => {
                if self.cycle_counter >= 172 {
                    self.cycle_counter = 0;

                    // Mode 3 has completed for the current visible scanline.
                    // Commit that line to the real 160x144 framebuffer.
                    if self.ly < 144 {
                        let line = self.render_background_scanline_cgb(
                            vram_bank_0,
                            vram_bank_1,
                            self.ly,
                        );
                        let start = self.ly as usize * 160;
                        self.framebuffer[start..start + 160].copy_from_slice(&line);
                    }

                    self.set_mode(0);
                }
            }
            0 => {
                if self.cycle_counter >= 204 {
                    self.cycle_counter = 0;
                    self.advance_line();

                    // LY became 144: the 160x144 framebuffer is complete.
                    if self.ly == 144 {
                        self.frame_ready = true;
                    }
                }
            }
            1 => {
                if self.cycle_counter >= 456 {
                    self.cycle_counter = 0;
                    self.advance_line();
                }
            }
            _ => unreachable!(),
        }
    }

    fn advance_line(&mut self) {
        let previous_ly = self.ly;

        if previous_ly < 144 {
            self.advance_window_line(previous_ly);
        }

        self.ly = self.ly.wrapping_add(1);

        if self.ly == 144 {
            self.set_mode(1);
            self.vblank_interrupt = true;
        } else if self.ly > 153 {
            self.ly = 0;
            self.window_line = 0;
            self.set_mode(2);
        } else if self.ly >= 144 {
            self.set_mode(1);
        } else {
            self.set_mode(2);
        }

        self.update_lyc_flag();
        self.update_stat_interrupt();
    }

    fn set_mode(&mut self, mode: u8) {
        self.mode = mode;
        self.stat = (self.stat & !0x03) | mode;
        self.update_stat_interrupt();
    }

    fn update_lyc_flag(&mut self) {
        if self.ly == self.lyc {
            self.stat |= 0x04;
        } else {
            self.stat &= !0x04;
        }
    }

    fn update_stat_interrupt(&mut self) {
        let coincidence = self.ly == self.lyc;
        self.update_lyc_flag();

        let hblank_enabled = self.stat & 0x08 != 0;
        let vblank_enabled = self.stat & 0x10 != 0;
        let oam_enabled = self.stat & 0x20 != 0;
        let lyc_enabled = self.stat & 0x40 != 0;

        let hblank_condition = self.mode == 0 && hblank_enabled;
        let vblank_condition = self.mode == 1 && vblank_enabled;
        let oam_condition = self.mode == 2 && oam_enabled;
        let lyc_condition = coincidence && lyc_enabled;

        let irq_line = hblank_condition || vblank_condition || oam_condition || lyc_condition;

        if irq_line && !self.stat_irq_line {
            self.stat_interrupt = true;
        }

        self.stat_irq_line = irq_line;
    }

    pub fn read(&self, address: u16) -> u8 {
        match address {
            0xFF40 => self.lcdc,
            0xFF41 => self.stat | 0x80,
            0xFF42 => self.scy,
            0xFF43 => self.scx,
            0xFF44 => self.ly,
            0xFF45 => self.lyc,
            0xFF68 => self.bgpi,
            0xFF69 => {
                let index = (self.bgpi & 0x3F) as usize;
                self.bg_palette_ram[index]
            }
            0xFF4A => self.wy,
            0xFF4B => self.wx,
            0xFF6A => self.obpi,
            0xFF6B => {
                let index = (self.obpi & 0x3F) as usize;
                self.obj_palette_ram[index]
            }
            _ => 0xFF,
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        match address {
            0xFF40 => {
                let was_enabled = self.lcdc & 0x80 != 0;
                let now_enabled = value & 0x80 != 0;
                self.lcdc = value;

                if was_enabled && !now_enabled {
                    self.ly = 0;
                    self.cycle_counter = 0;
                    self.mode = 0;
                    self.window_line = 0;
                    self.frame_ready = false;
                    self.stat = (self.stat & !0x07) | 0;
                    self.vblank_interrupt = false;
                    self.stat_interrupt = false;
                    self.stat_irq_line = false;
                    self.update_lyc_flag();
                    self.update_stat_interrupt();
                } else if !was_enabled && now_enabled {
                    self.ly = 0;
                    self.cycle_counter = 0;
                    self.mode = 2;
                    self.window_line = 0;
                    self.frame_ready = false;
                    self.stat = (self.stat & !0x03) | 2;
                    self.update_lyc_flag();
                    self.update_stat_interrupt();
                }
            }
            0xFF41 => {
                self.stat = (self.stat & 0x07) | (value & 0x78) | 0x80;
                self.update_stat_interrupt();
            }
            0xFF42 => self.scy = value,
            0xFF43 => self.scx = value,
            0xFF45 => {
                self.lyc = value;
                self.update_lyc_flag();
                self.update_stat_interrupt();
            }
            0xFF44 => {}
            0xFF4A => self.wy = value,
            0xFF4B => self.wx = value,
            0xFF68 => self.bgpi = value,
            0xFF69 => {
                let index = (self.bgpi & 0x3F) as usize;
                self.bg_palette_ram[index] = value;
                if self.bgpi & 0x80 != 0 {
                    self.bgpi = (self.bgpi & 0x80) | ((index as u8 + 1) & 0x3F);
                }
            }
            0xFF6A => self.obpi = value,
            0xFF6B => {
                let index = (self.obpi & 0x3F) as usize;
                self.obj_palette_ram[index] = value;
                if self.obpi & 0x80 != 0 {
                    self.obpi = (self.obpi & 0x80) | ((index as u8 + 1) & 0x3F);
                }
            }
            _ => {}
        }
    }

    pub fn take_vblank_interrupt(&mut self) -> bool {
        let pending = self.vblank_interrupt;
        self.vblank_interrupt = false;
        pending
    }

    pub fn take_stat_interrupt(&mut self) -> bool {
        let pending = self.stat_interrupt;
        self.stat_interrupt = false;
        pending
    }

    pub fn framebuffer(&self) -> &[u32; 160 * 144] {
        &self.framebuffer
    }

    pub fn frame_ready(&self) -> bool {
        self.frame_ready
    }

    pub fn take_frame_ready(&mut self) -> bool {
        let ready = self.frame_ready;
        self.frame_ready = false;
        ready
    }

    #[cfg(test)]
    pub fn ly(&self) -> u8 {
        self.ly
    }

    #[cfg(test)]
    pub fn mode(&self) -> u8 {
        self.mode
    }

    pub fn background_pixel_position(&self, screen_x: u8) -> (u8, u8) {
        (
            screen_x.wrapping_add(self.scx),
            self.ly.wrapping_add(self.scy),
        )
    }

    pub fn decode_tile_row(tile: &[u8; 16], row: usize) -> [u8; 8] {
        let low = tile[row * 2];
        let high = tile[row * 2 + 1];
        let mut pixels = [0; 8];

        for x in 0..8 {
            let bit = 7 - x;
            let lo = (low >> bit) & 1;
            let hi = (high >> bit) & 1;
            pixels[x] = (hi << 1) | lo;
        }

        pixels
    }

    pub fn background_tile_index(
        vram: &[u8; 0x2000],
        bg_x: u8,
        bg_y: u8,
        map_base: u16,
    ) -> u8 {
        let tile_x = (bg_x / 8) as usize;
        let tile_y = (bg_y / 8) as usize;
        let map_offset = (map_base - 0x8000) as usize;
        let index = map_offset + tile_y * 32 + tile_x;
        vram[index]
    }

    pub fn background_tile_data(
        vram: &[u8; 0x2000],
        tile_index: u8,
        tile_data_base: u16,
    ) -> [u8; 16] {
        let tile_address = if tile_data_base == 0x8000 {
            tile_data_base.wrapping_add((tile_index as u16) * 16)
        } else {
            let signed_index = tile_index as i8 as i16;
            (tile_data_base as i16 + signed_index * 16) as u16
        };

        let offset = (tile_address - 0x8000) as usize;
        let mut tile = [0u8; 16];
        tile.copy_from_slice(&vram[offset..offset + 16]);
        tile
    }

    pub fn render_background_scanline(
        &self,
        vram_bank_0: &[u8; 0x2000],
        vram_bank_1: &[u8; 0x2000],
    ) -> [u8; 160] {
        self.render_background_scanline_at(vram_bank_0, vram_bank_1, self.ly)
    }

    pub fn render_background_scanline_at(
        &self,
        vram_bank_0: &[u8; 0x2000],
        vram_bank_1: &[u8; 0x2000],
        screen_y: u8,
    ) -> [u8; 160] {
        let mut pixels = [0u8; 160];

        let map_base = if self.lcdc & 0x08 != 0 { 0x9C00 } else { 0x9800 };
        let tile_data_base = if self.lcdc & 0x10 != 0 { 0x8000 } else { 0x9000 };
        let bg_y = screen_y.wrapping_add(self.scy);

        for screen_x in 0..160u16 {
            let screen_x = screen_x as u8;
            let bg_x = screen_x.wrapping_add(self.scx);

            let tile_index = Self::background_tile_index(vram_bank_0, bg_x, bg_y, map_base);
            let attributes = Self::background_tile_attributes(vram_bank_1, bg_x, bg_y, map_base);
            let (_palette, vram_bank, flip_x, flip_y, _priority) = Self::background_tile_attribute_info(attributes);
            let tile_vram = if vram_bank { vram_bank_1 } else { vram_bank_0 };
            let tile = Self::background_tile_data(tile_vram, tile_index, tile_data_base);

            let row = if flip_y { 7 - (bg_y & 0x07) as usize } else { (bg_y & 0x07) as usize };
            let pixel_x = if flip_x { 7 - (bg_x & 0x07) as usize } else { (bg_x & 0x07) as usize };
            let row_pixels = Self::decode_tile_row(&tile, row);
            pixels[screen_x as usize] = row_pixels[pixel_x];
        }

        pixels
    }

    pub fn render_background_frame(
        &self,
        vram_bank_0: &[u8; 0x2000],
        vram_bank_1: &[u8; 0x2000],
    ) -> [u8; 160 * 144] {
        let mut frame = [0u8; 160 * 144];
        for y in 0..144 {
            let line = self.render_background_scanline_at(vram_bank_0, vram_bank_1, y as u8);
            let start = y * 160;
            frame[start..start + 160].copy_from_slice(&line);
        }
        frame
    }

    pub fn render_background_frame_cgb(
        &self,
        vram_bank_0: &[u8; 0x2000],
        vram_bank_1: &[u8; 0x2000],
    ) -> [u32; 160 * 144] {
        let mut frame = [0u32; 160 * 144];
        for y in 0..144 {
            let line = self.render_background_scanline_cgb(vram_bank_0, vram_bank_1, y as u8);
            let start = y * 160;
            frame[start..start + 160].copy_from_slice(&line);
        }
        frame
    }

    pub fn render_background_scanline_cgb(
        &self,
        vram_bank_0: &[u8; 0x2000],
        vram_bank_1: &[u8; 0x2000],
        screen_y: u8,
    ) -> [u32; 160] {
        let mut pixels = [0u32; 160];
        let map_base = if self.lcdc & 0x08 != 0 { 0x9C00 } else { 0x9800 };
        let tile_data_base = if self.lcdc & 0x10 != 0 { 0x8000 } else { 0x9000 };
        let bg_y = screen_y.wrapping_add(self.scy);

        for screen_x in 0..160u16 {
            let screen_x = screen_x as u8;
            let bg_x = screen_x.wrapping_add(self.scx);

            let tile_index = Self::background_tile_index(vram_bank_0, bg_x, bg_y, map_base);
            let attributes = Self::background_tile_attributes(vram_bank_1, bg_x, bg_y, map_base);
            let (palette, vram_bank, flip_x, flip_y, _priority) = Self::background_tile_attribute_info(attributes);
            let tile_vram = if vram_bank { vram_bank_1 } else { vram_bank_0 };
            let tile = Self::background_tile_data(tile_vram, tile_index, tile_data_base);

            let row = if flip_y { 7 - (bg_y & 0x07) as usize } else { (bg_y & 0x07) as usize };
            let pixel_x = if flip_x { 7 - (bg_x & 0x07) as usize } else { (bg_x & 0x07) as usize };
            let row_pixels = Self::decode_tile_row(&tile, row);
            let color_index = row_pixels[pixel_x];

            pixels[screen_x as usize] = self.background_palette_color(palette, color_index);
        }

        pixels
    }

    pub fn render_background_scanline_cgb_with_priority(
        &self,
        vram_bank_0: &[u8; 0x2000],
        vram_bank_1: &[u8; 0x2000],
    ) -> ([u32; 160], [bool; 160]) {
        let mut pixels = [0u32; 160];
        let mut priority = [false; 160];
        let map_base = if self.lcdc & 0x08 != 0 { 0x9C00 } else { 0x9800 };
        let tile_data_base = if self.lcdc & 0x10 != 0 { 0x8000 } else { 0x9000 };
        let bg_y = self.ly.wrapping_add(self.scy);

        for screen_x in 0..160u16 {
            let screen_x = screen_x as u8;
            let bg_x = screen_x.wrapping_add(self.scx);
            let tile_index = Self::background_tile_index(vram_bank_0, bg_x, bg_y, map_base);
            let attributes = Self::background_tile_attributes(vram_bank_1, bg_x, bg_y, map_base);
            let (palette, vram_bank, flip_x, flip_y, bg_priority) = Self::background_tile_attribute_info(attributes);
            let tile_vram = if vram_bank { vram_bank_1 } else { vram_bank_0 };
            let tile = Self::background_tile_data(tile_vram, tile_index, tile_data_base);

            let row = if flip_y { 7 - (bg_y & 0x07) as usize } else { (bg_y & 0x07) as usize };
            let pixel_x = if flip_x { 7 - (bg_x & 0x07) as usize } else { (bg_x & 0x07) as usize };
            let row_pixels = Self::decode_tile_row(&tile, row);
            let color_index = row_pixels[pixel_x];

            pixels[screen_x as usize] = self.background_palette_color(palette, color_index);
            priority[screen_x as usize] = bg_priority && color_index != 0;
        }

        (pixels, priority)
    }

    pub fn render_background_frame_cgb_with_priority(
        &self,
        vram_bank_0: &[u8; 0x2000],
        vram_bank_1: &[u8; 0x2000],
    ) -> ([u32; 160 * 144], [bool; 160 * 144]) {
        let mut frame = [0u32; 160 * 144];
        let mut priority = [false; 160 * 144];
        for y in 0..144 {
            let (line_pixels, line_priority) = self.render_background_scanline_cgb_with_priority(vram_bank_0, vram_bank_1);
            let start = y * 160;
            frame[start..start + 160].copy_from_slice(&line_pixels);
            priority[start..start + 160].copy_from_slice(&line_priority);
        }
        (frame, priority)
    }

    pub fn cgb_rgb555_to_argb(color: u16) -> u32 {
        let r = ((color >> 0) & 0x1F) as u32;
        let g = ((color >> 5) & 0x1F) as u32;
        let b = ((color >> 10) & 0x1F) as u32;
        let r = (r * 255) / 31;
        let g = (g * 255) / 31;
        let b = (b * 255) / 31;
        0xFF000000 | (r << 16) | (g << 8) | b
    }

    pub fn background_palette_color(&self, palette: u8, color_index: u8) -> u32 {
        let base = (palette as usize) * 8 + (color_index as usize) * 2;
        let low = self.bg_palette_ram[base] as u16;
        let high = self.bg_palette_ram[base + 1] as u16;
        let rgb555 = low | (high << 8);
        Self::cgb_rgb555_to_argb(rgb555)
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

    pub fn background_tile_attribute_info(attributes: u8) -> (u8, bool, bool, bool, bool) {
        let palette = attributes & 0x07;
        let vram_bank = attributes & 0x08 != 0;
        let flip_x = attributes & 0x20 != 0;
        let flip_y = attributes & 0x40 != 0;
        let priority = attributes & 0x80 != 0;
        (palette, vram_bank, flip_x, flip_y, priority)
    }

    // Existing window/sprite helpers continue below in the source tree.
}
