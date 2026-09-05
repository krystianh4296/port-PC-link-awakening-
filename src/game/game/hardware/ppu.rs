#[derive(Debug)]
pub struct Ppu {
    ly: u8,
    lyc: u8,
    lcdc: u8,

    stat: u8,

    scx: u8,
    scy: u8,
    bgp: u8,
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
            bgp: 0xFC,
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

    pub fn step(&mut self, cycles: u32) {
        if self.lcdc & 0x80 == 0 {
            return;
        }

        for _ in 0..cycles {
            self.step_cycle();
        }
    }

    fn step_cycle(&mut self) {
        self.cycle_counter += 1;

        match self.mode {
            // OAM search
            2 => {
                if self.cycle_counter >= 80 {
                    self.cycle_counter = 0;
                    self.set_mode(3);
                }
            }

            // Pixel transfer
            3 => {
                if self.cycle_counter >= 172 {
                    self.cycle_counter = 0;
                    self.set_mode(0);
                }
            }

            // HBlank
            0 => {
                if self.cycle_counter >= 204 {
                    self.cycle_counter = 0;
                    self.advance_line();
                }
            }

            // VBlank
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
        self.ly = self.ly.wrapping_add(1);

        if self.ly == 144 {
            self.set_mode(1);
            self.vblank_interrupt = true;
        } else if self.ly > 153 {
            self.ly = 0;
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

        if coincidence {
            self.stat |= 0x04;
        } else {
            self.stat &= !0x04;
        }

        let hblank_enabled = self.stat & 0x08 != 0;
        let vblank_enabled = self.stat & 0x10 != 0;
        let oam_enabled = self.stat & 0x20 != 0;
        let lyc_enabled = self.stat & 0x40 != 0;

        let hblank_condition = self.mode == 0 && hblank_enabled;
        let vblank_condition = self.mode == 1 && vblank_enabled;
        let oam_condition = self.mode == 2 && oam_enabled;
        let lyc_condition = coincidence && lyc_enabled;

        let irq_line =
            hblank_condition ||
            vblank_condition ||
            oam_condition ||
            lyc_condition;

        // STAT interrupt is triggered on a rising edge.
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
            0xFF47 => self.bgp,
            0xFF68 => self.bgpi,
            0xFF69 => {
                let index = (self.bgpi & 0x3F) as usize;
                self.bg_palette_ram[index]
            }
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
                    // LCD OFF:
                    // PPU zatrzymuje pracę i wraca do stanu początkowego.
                    self.ly = 0;
                    self.cycle_counter = 0;
                    self.mode = 0;

                    self.stat = (self.stat & !0x07) | 0;
                    self.vblank_interrupt = false;
                    self.stat_interrupt = false;
                    self.stat_irq_line = false;

                    self.update_lyc_flag();
                    self.update_stat_interrupt();
                } else if !was_enabled && now_enabled {
                    // LCD ON:
                    // start od początku pierwszej linii w OAM search.
                    self.ly = 0;
                    self.cycle_counter = 0;
                    self.mode = 2;

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

            0xFF47 => self.bgp = value,

            0xFF45 => {
                self.lyc = value;
                self.update_lyc_flag();
                self.update_stat_interrupt();
            }

            0xFF44 => {}
            0xFF68 => {
    self.bgpi = value;
}

0xFF69 => {
    let index = (self.bgpi & 0x3F) as usize;
    self.bg_palette_ram[index] = value;

    if self.bgpi & 0x80 != 0 {
        self.bgpi = (self.bgpi & 0x80) | ((index as u8 + 1) & 0x3F);
    }
}

0xFF6A => {
    self.obpi = value;
}

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
    vram: &[u8; 0x2000],
) -> [u8; 160] {
    self.render_background_scanline_at(vram, self.ly)
}
pub fn render_background_scanline_at(
    &self,
    vram: &[u8; 0x2000],
    screen_y: u8,
) -> [u8; 160] {
    let mut pixels = [0u8; 160];

    let map_base = if self.lcdc & 0x08 != 0 {
        0x9C00
    } else {
        0x9800
    };

    let tile_data_base = if self.lcdc & 0x10 != 0 {
        0x8000
    } else {
        0x9000
    };

    let bg_y = screen_y.wrapping_add(self.scy);

    for screen_x in 0..160u16 {
        let screen_x = screen_x as u8;
        let bg_x = screen_x.wrapping_add(self.scx);

        let tile_index =
            Self::background_tile_index(
                vram,
                bg_x,
                bg_y,
                map_base,
            );

        let tile =
            Self::background_tile_data(
                vram,
                tile_index,
                tile_data_base,
            );

        let row = (bg_y & 0x07) as usize;
        let pixel_x = (bg_x & 0x07) as usize;

        let row_pixels =
            Self::decode_tile_row(&tile, row);

        pixels[screen_x as usize] =
        self.apply_bgp_palette(row_pixels[pixel_x]);
    }

    pixels
}
pub fn render_background_frame(
    &self,
    vram: &[u8; 0x2000],
) -> [u8; 160 * 144] {
    let mut frame = [0u8; 160 * 144];

    for y in 0..144 {
        let line = self.render_background_scanline_at(
            vram,
            y as u8,
        );

        let start = y * 160;
        let end = start + 160;

        frame[start..end].copy_from_slice(&line);
    }

    frame
}
pub fn apply_bgp_palette(&self, color: u8) -> u8 {
    let shift = color * 2;
    (self.bgp >> shift) & 0b11
}
}


#[cfg(test)]
mod tests {
    use super::Ppu;

    #[test]
    fn vblank_starts_at_ly_144() {
        let mut ppu = Ppu::new();

        ppu.step(456 * 143);

        assert_eq!(ppu.ly(), 143);
        assert!(!ppu.take_vblank_interrupt());

        ppu.step(456);

        assert_eq!(ppu.ly(), 144);
        assert!(ppu.take_vblank_interrupt());
        assert!(!ppu.take_vblank_interrupt());
    }

    #[test]
    fn ppu_modes_follow_scanline_timing() {
        let mut ppu = Ppu::new();

        assert_eq!(ppu.mode(), 2);

        ppu.step(80);
        assert_eq!(ppu.mode(), 3);

        ppu.step(172);
        assert_eq!(ppu.mode(), 0);

        ppu.step(204);
        assert_eq!(ppu.ly(), 1);
        assert_eq!(ppu.mode(), 2);
    }

    #[test]
    fn stat_read_write() {
        let mut ppu = Ppu::new();

        ppu.write(0xFF41, 0x78);

        let stat = ppu.read(0xFF41);

        assert_eq!(stat & 0x78, 0x78);
        assert_eq!(stat & 0x80, 0x80);
    }

    #[test]
    fn lyc_coincidence_flag() {
        let mut ppu = Ppu::new();

        ppu.write(0xFF45, 0);

        assert_eq!(ppu.read(0xFF41) & 0x04, 0x04);

        ppu.write(0xFF45, 5);

        assert_eq!(ppu.read(0xFF41) & 0x04, 0);
    }

    #[test]
    fn stat_oam_interrupt() {
        let mut ppu = Ppu::new();

        // Enable OAM STAT interrupt.
        ppu.write(0xFF41, 0x20);

        // PPU starts in mode 2.
        assert!(ppu.take_stat_interrupt());

        // Interrupt was consumed.
        assert!(!ppu.take_stat_interrupt());

        // Move through mode 3 -> mode 0 -> next line mode 2.
        ppu.step(80);
        ppu.step(172);
        ppu.step(204);

        assert_eq!(ppu.mode(), 2);
        assert!(ppu.take_stat_interrupt());
    }

    #[test]
    fn stat_hblank_interrupt() {
        let mut ppu = Ppu::new();

        // Enable HBlank STAT interrupt.
        ppu.write(0xFF41, 0x08);

        ppu.step(80);
        ppu.step(172);

        assert_eq!(ppu.mode(), 0);
        assert!(ppu.take_stat_interrupt());

        assert!(!ppu.take_stat_interrupt());
    }

    #[test]
    fn stat_lyc_interrupt() {
        let mut ppu = Ppu::new();

        // LYC = 1
        ppu.write(0xFF45, 1);

        // Enable LYC STAT interrupt.
        ppu.write(0xFF41, 0x40);

        ppu.step(456);

        assert_eq!(ppu.ly(), 1);
        assert!(ppu.take_stat_interrupt());

        assert!(!ppu.take_stat_interrupt());
    }
    #[test]
fn lcd_disable_stops_ppu() {
    let mut ppu = Ppu::new();

    assert_eq!(ppu.read(0xFF40) & 0x80, 0x80);
    assert_eq!(ppu.mode(), 2);

    ppu.step(80);
    assert_eq!(ppu.mode(), 3);

    ppu.write(0xFF40, 0x11);

    assert_eq!(ppu.read(0xFF40) & 0x80, 0);
    assert_eq!(ppu.ly(), 0);
    assert_eq!(ppu.mode(), 0);

    ppu.step(1000);

    assert_eq!(ppu.ly(), 0);
    assert_eq!(ppu.mode(), 0);
}
#[test]
fn lcd_enable_restarts_ppu() {
    let mut ppu = Ppu::new();

    ppu.write(0xFF40, 0x00);

    assert_eq!(ppu.ly(), 0);
    assert_eq!(ppu.mode(), 0);

    ppu.write(0xFF40, 0x80);

    assert_eq!(ppu.ly(), 0);
    assert_eq!(ppu.mode(), 2);

    ppu.step(80);

    assert_eq!(ppu.mode(), 3);
}
#[test]
fn scx_scy_scroll_wraps_background_coordinates() {
    let mut ppu = Ppu::new();

    ppu.write(0xFF42, 250);
    ppu.write(0xFF43, 252);

    ppu.ly = 10;

    assert_eq!(ppu.background_pixel_position(0), (252, 4));
    assert_eq!(ppu.background_pixel_position(8), (4, 4));
}
#[test]
fn decode_blank_tile_row() {
    let tile = [0u8; 16];

    assert_eq!(
        Ppu::decode_tile_row(&tile, 0),
        [0,0,0,0,0,0,0,0]
    );
}
#[test]
fn decode_full_tile_row() {
    let mut tile = [0u8; 16];

    tile[0] = 0xFF;
    tile[1] = 0xFF;

    assert_eq!(
        Ppu::decode_tile_row(&tile, 0),
        [3,3,3,3,3,3,3,3]
    );
}
#[test]
fn decode_mixed_tile_row() {
    let mut tile = [0u8; 16];

    tile[0] = 0b01010101;
    tile[1] = 0b00110011;

    assert_eq!(
        Ppu::decode_tile_row(&tile, 0),
        [0,1,2,3,0,1,2,3]
    );
}
#[test]
fn decode_last_tile_row() {
    let mut tile = [0u8; 16];

    tile[14] = 0xFF;
    tile[15] = 0x00;

    assert_eq!(
        Ppu::decode_tile_row(&tile, 7),
        [1,1,1,1,1,1,1,1]
    );
}
#[test]
fn background_tile_index_reads_correct_tile() {
    let mut vram = [0u8; 0x2000];

    // Mapa 0x9800.
    // tile (2, 3) => 0x1800 + 3 * 32 + 2
    let index = 0x1800 + 3 * 32 + 2;
    vram[index] = 0x57;

    assert_eq!(
        Ppu::background_tile_index(&vram, 16, 24, 0x9800),
        0x57
    );
}
#[test]
fn background_tile_index_supports_second_tile_map() {
    let mut vram = [0u8; 0x2000];

    let index = 0x1C00 + 5 * 32 + 7;
    vram[index] = 0xA3;

    assert_eq!(
        Ppu::background_tile_index(&vram, 56, 40, 0x9C00),
        0xA3
    );
}
#[test]
fn background_tile_data_uses_8000_mode() {
    let mut vram = [0u8; 0x2000];

    let offset = 3 * 16;

    for i in 0..16 {
        vram[offset + i] = i as u8;
    }

    let tile = Ppu::background_tile_data(
        &vram,
        3,
        0x8000,
    );

    assert_eq!(tile, [
        0, 1, 2, 3,
        4, 5, 6, 7,
        8, 9, 10, 11,
        12, 13, 14, 15,
    ]);
}
#[test]
fn background_tile_data_uses_signed_8800_mode() {
    let mut vram = [0u8; 0x2000];

    // tile_index = 0xFF = -1
    // $9000 + (-1 * 16) = $8FF0
    let offset = 0x0FF0;

    for i in 0..16 {
        vram[offset + i] = 0xA0 + i as u8;
    }

    let tile = Ppu::background_tile_data(
        &vram,
        0xFF,
        0x9000,
    );

    assert_eq!(tile, [
        0xA0, 0xA1, 0xA2, 0xA3,
        0xA4, 0xA5, 0xA6, 0xA7,
        0xA8, 0xA9, 0xAA, 0xAB,
        0xAC, 0xAD, 0xAE, 0xAF,
    ]);
}
#[test]
fn background_tile_data_decodes_row() {
    let mut vram = [0u8; 0x2000];

    let offset = 2 * 16;

    // row 0 = kolory 0,1,2,3,0,1,2,3
    vram[offset] = 0b01010101;
    vram[offset + 1] = 0b00110011;

    let tile = Ppu::background_tile_data(
        &vram,
        2,
        0x8000,
    );

    assert_eq!(
        Ppu::decode_tile_row(&tile, 0),
        [0, 1, 2, 3, 0, 1, 2, 3]
    );
}
#[test]
fn render_background_scanline_reads_tiles_and_scroll() {
    let mut ppu = Ppu::new();

    // LCDC:
    // bit 7 = LCD ON
    // bit 4 = tile data 8000
    // bit 3 = tile map 9800
    ppu.write(0xFF40, 0x99);

    let mut vram = [0u8; 0x2000];

    // Tile #0 w 0x8000.
    //
    // Pierwszy wiersz:
    // 00 01 02 03 00 01 02 03
    vram[0] = 0b01010101;
    vram[1] = 0b00110011;

    // Tile #0 w mapie 0x9800.
    vram[0x1800] = 0;

    let pixels = ppu.render_background_scanline(&vram);

    assert_eq!(
        &pixels[0..8],
        &[0, 1, 2, 3, 0, 1, 2, 3]
    );
}
#[test]
fn render_background_scanline_respects_scx() {
    let mut ppu = Ppu::new();

    // LCDC: LCD ON, tile data 8000, tile map 9800
    ppu.write(0xFF40, 0x91);
    ppu.write(0xFF43, 4);

    let mut vram = [0u8; 0x2000];

    // Tile 0 = all 0
    vram[0] = 0x00;
    vram[1] = 0x00;

    // Tile 1 = all 3
    vram[16] = 0xFF;
    vram[17] = 0xFF;

    // BG map 9800
    vram[0x1800] = 0;
    vram[0x1801] = 1;

    let pixels = ppu.render_background_scanline(&vram);

    assert_eq!(&pixels[0..4], &[0, 0, 0, 0]);
    assert_eq!(&pixels[4..12], &[3, 3, 3, 3, 3, 3, 3, 3]);
}
#[test]
fn render_background_frame_has_144_lines() {
    let mut ppu = Ppu::new();

    // LCD ON, tile data 8000, tile map 9800
    ppu.write(0xFF40, 0x91);

    let mut vram = [0u8; 0x2000];

    // Tile 0 = kolor 0
    vram[0] = 0x00;
    vram[1] = 0x00;

    // Tile 1 = kolor 3
    vram[16] = 0xFF;
    vram[17] = 0xFF;

    // Pierwszy tile map: tile 0
    // Drugi tile map: tile 1
    vram[0x1800] = 0;
    vram[0x1801] = 1;

    let frame = ppu.render_background_frame(&vram);

    assert_eq!(frame.len(), 160 * 144);

    // Pierwsze 4 piksele z tile 0
    assert_eq!(&frame[0..4], &[0, 0, 0, 0]);

    // Piksele 8..15 pochodzą z tile 1
    assert_eq!(
        &frame[8..16],
        &[3, 3, 3, 3, 3, 3, 3, 3]
    );

    // Początek drugiej linii powinien być taki sam.
    assert_eq!(&frame[160..164], &[0, 0, 0, 0]);
}
#[test]
fn bgp_default_palette() {
    let ppu = Ppu::new();

    assert_eq!(ppu.apply_bgp_palette(0), 0);
    assert_eq!(ppu.apply_bgp_palette(1), 1);
    assert_eq!(ppu.apply_bgp_palette(2), 2);
    assert_eq!(ppu.apply_bgp_palette(3), 3);
}
#[test]
fn bgp_reverse_palette() {
    let mut ppu = Ppu::new();

    ppu.write(0xFF47, 0x1B);

    assert_eq!(ppu.apply_bgp_palette(0), 3);
    assert_eq!(ppu.apply_bgp_palette(1), 2);
    assert_eq!(ppu.apply_bgp_palette(2), 1);
    assert_eq!(ppu.apply_bgp_palette(3), 0);
}
#[test]
fn render_scanline_uses_bgp_palette() {
    let mut ppu = Ppu::new();

    ppu.write(0xFF40, 0x91);
    ppu.write(0xFF47, 0x1B);

    let mut vram = [0u8; 0x2000];

    // Tile 0 = 0,1,2,3,0,1,2,3
    vram[0] = 0b01010101;
    vram[1] = 0b00110011;

    vram[0x1800] = 0;

    let pixels = ppu.render_background_scanline_at(&vram, 0);

    assert_eq!(
        &pixels[0..8],
        &[3,2,1,0,3,2,1,0]
    );
}
}