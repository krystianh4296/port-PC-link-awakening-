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
    bgp: u8,
    cycle_counter: u32,
    mode: u8,
    mode3_cycles: u32,
    vblank_interrupt: bool,
    stat_interrupt: bool,
    stat_irq_line: bool,
    hblank_started: bool,
}

impl Ppu {
    pub fn new() -> Self {
        let mut bg_palette_ram = [0u8; 64];
        let mut obj_palette_ram = [0u8; 64];
        let default_palette = [0x7FFFu16, 0x56B5u16, 0x294Au16, 0x0000u16];
        for (i, color) in default_palette.iter().enumerate() {
            bg_palette_ram[i * 2] = *color as u8;
            bg_palette_ram[i * 2 + 1] = (*color >> 8) as u8;
            obj_palette_ram[i * 2] = *color as u8;
            obj_palette_ram[i * 2 + 1] = (*color >> 8) as u8;
        }
        Self {
            ly: 0, lyc: 0, lcdc: 0x91, stat: 0x82,
            scx: 0, scy: 0, wx: 0, wy: 0, window_line: 0,
            framebuffer: [0xFF000000; 160 * 144], frame_ready: false,
            bg_palette_ram, obj_palette_ram, bgpi: 0, obpi: 0,
            bgp: 0xFC, cycle_counter: 0, mode: 2, mode3_cycles: 172,
            vblank_interrupt: false, stat_interrupt: false, stat_irq_line: false,
            hblank_started: false,
        }
    }

    pub fn step(&mut self, cycles: u32, oam: &[u8; 0xA0], vram0: &[u8; 0x2000], vram1: &[u8; 0x2000]) {
        if self.lcdc & 0x80 == 0 { return; }
        for _ in 0..cycles { self.step_cycle(oam, vram0, vram1); }
    }

    fn step_cycle(&mut self, oam: &[u8; 0xA0], vram0: &[u8; 0x2000], vram1: &[u8; 0x2000]) {
        self.cycle_counter += 1;
        match self.mode {
            2 if self.cycle_counter >= 80 => {
                self.cycle_counter = 0;
                // Sprite selection and the low SCX bits are sampled at the
                // start of pixel transfer. Keep this duration fixed until
                // HBlank so one scanline always remains 456 dots long.
                self.mode3_cycles = self.mode3_length(oam);
                self.set_mode(3);
            }
            3 if self.cycle_counter >= self.mode3_cycles => {
                self.cycle_counter = 0;
                if self.ly < 144 {
                    let line = self.render_background_scanline_with_window(
                        vram0,
                        vram1,
                        self.ly,
                        self.window_line,
                    );
                    let start = self.ly as usize * 160;
                    self.framebuffer[start..start + 160].copy_from_slice(&line);
                    self.render_sprites_scanline(oam, vram0, vram1, self.ly);
                    if self.window_is_visible_on_line(self.ly) {
                        self.window_line = self.window_line.wrapping_add(1);
                    }
                }
                self.set_mode(0);
                self.hblank_started = true;
            }
            0 if self.cycle_counter >= 376 - self.mode3_cycles => {
                self.cycle_counter = 0;
                self.ly = self.ly.wrapping_add(1);
                if self.ly == 144 { self.set_mode(1); self.vblank_interrupt = true; self.frame_ready = true; }
                else if self.ly > 153 { self.ly = 0; self.window_line = 0; self.set_mode(2); }
                else { self.set_mode(2); }
                self.update_lyc_flag(); self.update_stat_interrupt();
            }
            1 if self.cycle_counter >= 456 => {
                self.cycle_counter = 0;
                self.ly = self.ly.wrapping_add(1);
                if self.ly > 153 { self.ly = 0; self.window_line = 0; self.set_mode(2); }
                self.update_lyc_flag(); self.update_stat_interrupt();
            }
            _ => {}
        }
    }

    fn set_mode(&mut self, mode: u8) { self.mode = mode; self.stat = (self.stat & !3) | mode; self.update_stat_interrupt(); }

    fn mode3_length(&self, oam: &[u8; 0xA0]) -> u32 {
        let mut length = 172 + u32::from(self.scx & 7);
        if self.window_is_visible_on_line(self.ly) {
            length += 6;
        }

        let sprite_height = if self.lcdc & 0x04 != 0 { 16 } else { 8 };
        let mut sprites: Vec<(u8, usize)> = (0..40)
            .filter(|&index| {
                let y = oam[index * 4] as i16 - 16;
                y <= self.ly as i16 && y + sprite_height > self.ly as i16
            })
            .take(10)
            .map(|index| (oam[index * 4 + 1], index))
            .collect();
        sprites.sort_unstable();

        let mut seen_tiles = [false; 32];
        for (oam_x, _) in sprites {
            if oam_x == 0 {
                length += 11;
                continue;
            }
            let screen_x = i16::from(oam_x) - 8;
            let fetch_x = screen_x.max(0);
            let tile_x = if self.window_is_visible_on_line(self.ly)
                && screen_x >= i16::from(self.wx) - 7
            {
                ((fetch_x - (i16::from(self.wx) - 7)) as usize / 8) & 31
            } else {
                ((fetch_x + i16::from(self.scx)) as usize / 8) & 31
            };
            if !seen_tiles[tile_x] {
                seen_tiles[tile_x] = true;
                let pixel_in_tile = (fetch_x as usize + usize::from(self.scx)) & 7;
                length += (5usize.saturating_sub(pixel_in_tile) + 6) as u32;
            } else {
                length += 6;
            }
        }
        length.min(289)
    }
    fn update_lyc_flag(&mut self) { if self.ly == self.lyc { self.stat |= 4; } else { self.stat &= !4; } }
    fn update_stat_interrupt(&mut self) {
        let coincidence = self.ly == self.lyc;
        let irq = (self.mode == 0 && self.stat & 8 != 0) || (self.mode == 1 && self.stat & 0x10 != 0)
            || (self.mode == 2 && self.stat & 0x20 != 0) || (coincidence && self.stat & 0x40 != 0);
        if irq && !self.stat_irq_line { self.stat_interrupt = true; }
        self.stat_irq_line = irq;
    }

    pub fn read(&self, address: u16) -> u8 {
        match address {
            0xFF40 => self.lcdc, 0xFF41 => self.stat | 0x80, 0xFF42 => self.scy,
            0xFF43 => self.scx, 0xFF44 => self.ly, 0xFF45 => self.lyc, 0xFF47 => self.bgp,
            0xFF4A => self.wy, 0xFF4B => self.wx, 0xFF68 => self.bgpi,
            0xFF69 => self.bg_palette_ram[(self.bgpi & 0x3F) as usize], 0xFF6A => self.obpi,
            0xFF6B => self.obj_palette_ram[(self.obpi & 0x3F) as usize], _ => 0xFF,
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        match address {
            0xFF40 => {
                let was = self.lcdc & 0x80 != 0; let now = value & 0x80 != 0; self.lcdc = value;
                if was && !now {
                    self.ly = 0;
                    self.cycle_counter = 0;
                    self.mode3_cycles = 172;
                    self.frame_ready = false;
                    self.set_mode(0);
                } else if !was && now {
                    self.ly = 0;
                    self.cycle_counter = 0;
                    self.mode3_cycles = 172;
                    self.frame_ready = false;
                    self.set_mode(2);
                }
                self.update_lyc_flag(); self.update_stat_interrupt();
            }
            0xFF41 => { self.stat = (self.stat & 7) | (value & 0x78) | 0x80; self.update_stat_interrupt(); }
            0xFF42 => self.scy = value,
            0xFF43 => self.scx = value,
            0xFF45 => { self.lyc = value; self.update_lyc_flag(); self.update_stat_interrupt(); }
            0xFF47 => self.bgp = value, 0xFF4A => self.wy = value, 0xFF4B => self.wx = value,
            0xFF68 => self.bgpi = value,
            0xFF69 => { let i = (self.bgpi & 0x3F) as usize; self.bg_palette_ram[i] = value; if self.bgpi & 0x80 != 0 { self.bgpi = 0x80 | ((i as u8 + 1) & 0x3F); } }
            0xFF6A => self.obpi = value,
            0xFF6B => { let i = (self.obpi & 0x3F) as usize; self.obj_palette_ram[i] = value; if self.obpi & 0x80 != 0 { self.obpi = 0x80 | ((i as u8 + 1) & 0x3F); } }
            _ => {}
        }
    }

    pub fn take_vblank_interrupt(&mut self) -> bool { let x = self.vblank_interrupt; self.vblank_interrupt = false; x }
    pub fn take_stat_interrupt(&mut self) -> bool { let x = self.stat_interrupt; self.stat_interrupt = false; x }
    pub fn take_hblank_started(&mut self) -> bool { let x = self.hblank_started; self.hblank_started = false; x }
    pub fn framebuffer(&self) -> &[u32; 160 * 144] { &self.framebuffer }
    pub fn frame_ready(&self) -> bool { self.frame_ready }
    pub fn take_frame_ready(&mut self) -> bool { let x = self.frame_ready; self.frame_ready = false; x }
    #[cfg(test)] pub fn ly(&self) -> u8 { self.ly }
    #[cfg(test)] pub fn mode(&self) -> u8 { self.mode }

    pub fn background_tile_index(vram: &[u8; 0x2000], bg_x: u8, bg_y: u8, map_base: u16) -> u8 {
        let map_offset = (map_base - 0x8000) as usize;
        let tile_x = (bg_x as usize) >> 3;
        let tile_y = (bg_y as usize) >> 3;
        vram[map_offset + tile_y * 32 + tile_x]
    }

    pub fn background_tile_data(vram: &[u8; 0x2000], tile_index: u8, base: u16) -> [u8; 16] {
        let address = if base == 0x8000 { 0x8000usize + tile_index as usize * 16 }
        else { let signed_index = tile_index as i8 as isize; (0x9000isize + signed_index * 16) as usize };
        let offset = address - 0x8000;
        let mut tile = [0; 16];
        tile.copy_from_slice(&vram[offset..offset + 16]);
        tile
    }

    pub fn decode_tile_row(tile: &[u8; 16], row: usize) -> [u8; 8] {
        let lo = tile[row * 2]; let hi = tile[row * 2 + 1];
        std::array::from_fn(|x| { let bit = 7 - x; ((hi >> bit) & 1) << 1 | ((lo >> bit) & 1) })
    }

    pub fn background_tile_attributes(vram: &[u8; 0x2000], bg_x: u8, bg_y: u8, map_base: u16) -> u8 { Self::background_tile_index(vram, bg_x, bg_y, map_base) }
    pub fn background_tile_attribute_info(a: u8) -> (u8, bool, bool, bool, bool) { (a & 7, a & 8 != 0, a & 0x20 != 0, a & 0x40 != 0, a & 0x80 != 0) }
    pub fn cgb_rgb555_to_argb(color: u16) -> u32 {
        let r = ((color & 0x1F) as u32 * 255) / 31; let g = (((color >> 5) & 0x1F) as u32 * 255) / 31; let b = (((color >> 10) & 0x1F) as u32 * 255) / 31;
        0xFF000000 | r << 16 | g << 8 | b
    }
    pub fn background_palette_color(&self, palette: u8, index: u8) -> u32 {
        let base = palette as usize * 8 + index as usize * 2; let color = self.bg_palette_ram[base] as u16 | ((self.bg_palette_ram[base+1] as u16) << 8); Self::cgb_rgb555_to_argb(color)
    }
    pub fn apply_bgp_palette(&self, color_index: u8) -> u8 { let shift = (color_index & 0x03) * 2; (self.bgp >> shift) & 0x03 }

    pub fn render_background_scanline_cgb(&self, vram0: &[u8; 0x2000], vram1: &[u8; 0x2000], y: u8) -> [u32; 160] {
        self.render_background_scanline_with_window(vram0, vram1, y, y.wrapping_sub(self.wy))
    }

    fn window_is_visible_on_line(&self, y: u8) -> bool {
        self.lcdc & 0x20 != 0 && y >= self.wy && self.wx <= 166
    }

    fn render_background_scanline_with_window(
        &self,
        vram0: &[u8; 0x2000],
        vram1: &[u8; 0x2000],
        y: u8,
        window_line: u8,
    ) -> [u32; 160] {
        let mut out = [0u32; 160];
        let base = if self.lcdc & 0x10 != 0 { 0x8000 } else { 0x9000 };
        for x in 0..160usize {
            let use_window = self.window_is_visible_on_line(y)
                && x as i16 >= self.wx as i16 - 7;
            let (map, bg_x, bg_y) = if use_window {
                (
                    if self.lcdc & 0x40 != 0 { 0x9C00 } else { 0x9800 },
                    (x as i16 - (self.wx as i16 - 7)) as usize,
                    window_line as usize,
                )
            } else {
                (
                    if self.lcdc & 8 != 0 { 0x9C00 } else { 0x9800 },
                    (x + self.scx as usize) & 0xFF,
                    (y as usize + self.scy as usize) & 0xFF,
                )
            };
            let tile_x = bg_x >> 3; let tile_y = bg_y >> 3;
            let map_index = (map - 0x8000) as usize + tile_y * 32 + tile_x;
            let tile_index = vram0[map_index];
            let attr = vram1[map_index];
            let (palette, bank, flip_x, flip_y, _) = Self::background_tile_attribute_info(attr);
            let tile_vram = if bank { vram1 } else { vram0 };
            let tile = Self::background_tile_data(tile_vram, tile_index, base);
            let row = if flip_y { 7 - (bg_y & 7) } else { bg_y & 7 };
            let px = if flip_x { 7 - (bg_x & 7) } else { bg_x & 7 };
            let ci = Self::decode_tile_row(&tile, row)[px];
            out[x] = self.background_palette_color(palette, ci);
        }
        out
    }

    fn background_pixel_info_at(
        &self,
        vram0: &[u8; 0x2000],
        vram1: &[u8; 0x2000],
        x: usize,
        y: u8,
    ) -> (u8, bool) {
        if self.lcdc & 0x01 == 0 {
            return (0, false);
        }
        let use_window = self.window_is_visible_on_line(y)
            && x as i16 >= self.wx as i16 - 7;
        let (map, bg_x, bg_y) = if use_window {
            (
                if self.lcdc & 0x40 != 0 { 0x9C00 } else { 0x9800 },
                (x as i16 - (self.wx as i16 - 7)) as usize,
                self.window_line as usize,
            )
        } else {
            (
                if self.lcdc & 8 != 0 { 0x9C00 } else { 0x9800 },
                (x + self.scx as usize) & 0xFF,
                (y as usize + self.scy as usize) & 0xFF,
            )
        };
        let base = if self.lcdc & 0x10 != 0 { 0x8000 } else { 0x9000 };
        let tile_x = bg_x >> 3; let tile_y = bg_y >> 3;
        let map_index = (map - 0x8000) as usize + tile_y * 32 + tile_x;
        let tile_index = vram0[map_index];
        let attr = vram1[map_index];
        let (_, bank, flip_x, flip_y, _) = Self::background_tile_attribute_info(attr);
        let tile_vram = if bank { vram1 } else { vram0 };
        let tile = Self::background_tile_data(tile_vram, tile_index, base);
        let row = if flip_y { 7 - (bg_y & 7) } else { bg_y & 7 };
        let px = if flip_x { 7 - (bg_x & 7) } else { bg_x & 7 };
        (Self::decode_tile_row(&tile, row)[px], attr & 0x80 != 0)
    }

    fn obj_palette_color(&self, palette: u8, index: u8) -> u32 {
        let base = palette as usize * 8 + index as usize * 2;
        let color = self.obj_palette_ram[base] as u16 | ((self.obj_palette_ram[base + 1] as u16) << 8);
        Self::cgb_rgb555_to_argb(color)
    }

    fn render_sprites_scanline(&mut self, oam: &[u8; 0xA0], vram0: &[u8; 0x2000], vram1: &[u8; 0x2000], line: u8) {
        if self.lcdc & 0x02 == 0 { return; }
        const WIDTH: usize = 160;
        let sprite_height = if self.lcdc & 0x04 != 0 { 16usize } else { 8usize };

        // Mode 2 selects at most ten sprites per scanline in OAM order.
        // Draw the selected list in reverse so a lower OAM index retains
        // priority when sprites overlap.
        let visible_sprites: Vec<usize> = (0..40)
            .filter(|&index| {
                let sprite_y = oam[index * 4] as i32 - 16;
                sprite_y <= line as i32 && sprite_y + sprite_height as i32 > line as i32
            })
            .take(10)
            .collect();

        for index in visible_sprites.into_iter().rev() {
            let base = index * 4;
            let y = oam[base] as i32;
            let x = oam[base + 1] as i32;
            let tile_index = oam[base + 2];
            let attrs = oam[base + 3];
            let sprite_y = y - 16;
            let sprite_x = x - 8;
            if sprite_y > line as i32 || sprite_y + sprite_height as i32 <= line as i32 { continue; }

            let flip_y = attrs & 0x40 != 0;
            let flip_x = attrs & 0x20 != 0;
            let behind_bg = attrs & 0x80 != 0;
            let palette = attrs & 0x07;
            let tile_vram = if attrs & 0x08 != 0 { vram1 } else { vram0 };

            let row = (line as i32 - sprite_y) as usize;
            let sprite_row = if flip_y { sprite_height - 1 - row } else { row };
            if sprite_row >= sprite_height { continue; }
            let tile_id = if sprite_height == 16 {
                (tile_index & 0xFE).wrapping_add(if sprite_row >= 8 { 1 } else { 0 })
            } else { tile_index };
            let tile_offset = tile_id as usize * 16;
            if tile_offset + 15 >= tile_vram.len() { continue; }
            let pixel_row = sprite_row & 7;
            let low = tile_vram[tile_offset + pixel_row * 2];
            let high = tile_vram[tile_offset + pixel_row * 2 + 1];

            for col in 0..8usize {
                let screen_x = sprite_x + col as i32;
                if screen_x < 0 || screen_x >= WIDTH as i32 { continue; }
                let sprite_col = if flip_x { 7 - col } else { col };
                let bit = 7 - sprite_col;
                let color_id = ((high >> bit) & 1) << 1 | ((low >> bit) & 1);
                if color_id == 0 { continue; }
                let sx = screen_x as usize;
                let (bg_color_id, bg_priority) = self.background_pixel_info_at(vram0, vram1, sx, line);
                if bg_color_id != 0 && (behind_bg || bg_priority) { continue; }
                self.framebuffer[line as usize * WIDTH + sx] = self.obj_palette_color(palette, color_id);
            }
        }
    }
}
