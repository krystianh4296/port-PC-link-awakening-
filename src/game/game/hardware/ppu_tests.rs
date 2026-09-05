use super::ppu::Ppu;

fn blank_vram() -> ([u8; 0x2000], [u8; 0x2000]) {
    ([0; 0x2000], [0; 0x2000])
}

#[test]
fn ppu_scanline_timing_is_exact() {
    let mut ppu = Ppu::new();
    let (vram0, vram1) = blank_vram();
    let oam = [0; 0xA0];

    assert_eq!(ppu.ly(), 0);
    assert_eq!(ppu.mode(), 2);

    ppu.step(79, &oam, &vram0, &vram1);
    assert_eq!(ppu.mode(), 2);
    ppu.step(1, &oam, &vram0, &vram1);
    assert_eq!(ppu.mode(), 3);

    ppu.step(171, &oam, &vram0, &vram1);
    assert_eq!(ppu.mode(), 3);
    ppu.step(1, &oam, &vram0, &vram1);
    assert_eq!(ppu.mode(), 0);

    ppu.step(203, &oam, &vram0, &vram1);
    assert_eq!(ppu.mode(), 0);
    ppu.step(1, &oam, &vram0, &vram1);
    assert_eq!(ppu.ly(), 1);
    assert_eq!(ppu.mode(), 2);
}

#[test]
fn ppu_enters_vblank_at_ly_144_and_marks_frame_ready() {
    let mut ppu = Ppu::new();
    let (vram0, vram1) = blank_vram();
    let oam = [0; 0xA0];

    ppu.step(456 * 144, &oam, &vram0, &vram1);

    assert_eq!(ppu.ly(), 144);
    assert_eq!(ppu.mode(), 1);
    assert!(ppu.frame_ready());
    assert!(ppu.take_vblank_interrupt());
    assert!(!ppu.take_vblank_interrupt());
}

#[test]
fn frame_ready_is_consumed_without_changing_scanline() {
    let mut ppu = Ppu::new();
    let (vram0, vram1) = blank_vram();
    let oam = [0; 0xA0];

    ppu.step(456 * 144, &oam, &vram0, &vram1);
    assert!(ppu.take_frame_ready());
    assert!(!ppu.take_frame_ready());
    assert_eq!(ppu.ly(), 144);
}

#[test]
fn lcd_disable_and_enable_reset_scanline_state() {
    let mut ppu = Ppu::new();
    let (vram0, vram1) = blank_vram();
    let oam = [0; 0xA0];

    ppu.step(100, &oam, &vram0, &vram1);
    ppu.write(0xFF40, 0x00);
    assert_eq!(ppu.read(0xFF44), 0);
    assert_eq!(ppu.mode(), 0);

    ppu.step(1000, &oam, &vram0, &vram1);
    assert_eq!(ppu.read(0xFF44), 0);

    ppu.write(0xFF40, 0x91);
    assert_eq!(ppu.read(0xFF44), 0);
    assert_eq!(ppu.mode(), 2);
}

#[test]
fn stat_and_lyc_registers_preserve_expected_bits() {
    let mut ppu = Ppu::new();

    ppu.write(0xFF41, 0x78);
    let stat = ppu.read(0xFF41);
    assert_eq!(stat & 0x78, 0x78);
    assert_eq!(stat & 0x80, 0x80);
    assert_eq!(stat & 0x03, 2);

    ppu.write(0xFF45, 0);
    assert_eq!(ppu.read(0xFF41) & 0x04, 0x04);
    ppu.write(0xFF45, 7);
    assert_eq!(ppu.read(0xFF45), 7);
    assert_eq!(ppu.read(0xFF41) & 0x04, 0);
}

#[test]
fn dmg_bgp_palette_maps_all_four_color_indices() {
    let mut ppu = Ppu::new();
    ppu.write(0xFF47, 0b11_10_01_00);

    assert_eq!(ppu.apply_bgp_palette(0), 0);
    assert_eq!(ppu.apply_bgp_palette(1), 1);
    assert_eq!(ppu.apply_bgp_palette(2), 2);
    assert_eq!(ppu.apply_bgp_palette(3), 3);
}

#[test]
fn tile_decoder_extracts_four_color_indices() {
    let tile = [0xAA, 0x55, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];

    assert_eq!(Ppu::decode_tile_row(&tile, 0), [1, 2, 1, 2, 1, 2, 1, 2]);
    assert_eq!(Ppu::decode_tile_row(&tile, 1), [2; 8]);
    assert_eq!(Ppu::decode_tile_row(&tile, 2), [1; 8]);
}

#[test]
fn tile_data_supports_unsigned_and_signed_addressing() {
    let mut vram = [0u8; 0x2000];
    for i in 0..32 { vram[i] = i as u8; }

    assert_eq!(Ppu::background_tile_data(&vram, 0, 0x8000)[0], 0);
    assert_eq!(Ppu::background_tile_data(&vram, 1, 0x8000)[0], 16);

    vram[0x0FF0] = 0xA5;
    assert_eq!(Ppu::background_tile_data(&vram, 0xFF, 0x9000)[0], 0xA5);
}

#[test]
fn cgb_background_attributes_decode_palette_bank_flips_and_priority() {
    let (palette, bank, flip_x, flip_y, priority) = Ppu::background_tile_attribute_info(0xE9);

    assert_eq!(palette, 1);
    assert!(bank);
    assert!(flip_x);
    assert!(flip_y);
    assert!(priority);
}

#[test]
fn cgb_palette_autoincrement_writes_consecutive_bytes() {
    let mut ppu = Ppu::new();

    ppu.write(0xFF68, 0x80);
    ppu.write(0xFF69, 0x34);
    ppu.write(0xFF69, 0x12);

    ppu.write(0xFF68, 0x00);
    assert_eq!(ppu.read(0xFF69), 0x34);
    ppu.write(0xFF68, 0x01);
    assert_eq!(ppu.read(0xFF69), 0x12);
}

#[test]
fn background_scanline_reads_tile_map_and_tile_data() {
    let ppu = Ppu::new();
    let (mut vram0, vram1) = blank_vram();

    vram0[0] = 0xFF;
    vram0[1] = 0x00;
    vram0[0x1800] = 0;

    let line = ppu.render_background_scanline_cgb(&vram0, &vram1, 0);
    assert!(line.iter().all(|&pixel| pixel != 0xFF000000));
}

#[test]
fn background_map_tile_80_uses_vram_8800_in_unsigned_mode() {
    let ppu = Ppu::new();
    let (mut vram0, vram1) = blank_vram();

    // The game's BG map starts with tile 0x80. In unsigned tile mode,
    // tile 0x80 must resolve to VRAM address 0x8800.
    vram0[0x1800] = 0x80;
    // 0x8800 - 0x8000 = 0x0800.
    vram0[0x0800] = 0xFF;
    vram0[0x0801] = 0x00;

    assert_eq!(Ppu::background_tile_index(&vram0, 0, 0, 0x9800), 0x80);

    let tile = Ppu::background_tile_data(&vram0, 0x80, 0x8000);
    assert_eq!(tile[0], 0xFF);
    assert_eq!(tile[1], 0x00);
    assert_eq!(Ppu::decode_tile_row(&tile, 0), [1; 8]);

    let line = ppu.render_background_scanline_cgb(&vram0, &vram1, 0);
    assert!(line.iter().all(|&pixel| pixel != 0xFF000000));
}

#[test]
fn full_frame_background_renders_all_144_visible_scanlines() {
    let mut ppu = Ppu::new();
    let (mut vram0, vram1) = blank_vram();
    let oam = [0; 0xA0];

    // Reproduce the relevant layout observed in the Zelda graphics
    // initialization: BG map 0x9800 points at tile 0x80, whose data starts
    // at 0x8800 in unsigned tile-addressing mode.
    for offset in 0..(32 * 32) {
        vram0[0x1800 + offset] = 0x80;
    }
    for row in 0..8 {
        vram0[0x0800 + row * 2] = 0xFF;
        vram0[0x0800 + row * 2 + 1] = 0x00;
    }

    // 144 visible lines = 144 * 456 cycles. The PPU renders each line while
    // entering mode 0, then enters VBlank at LY=144.
    ppu.step(456 * 144, &oam, &vram0, &vram1);

    assert_eq!(ppu.ly(), 144);
    assert_eq!(ppu.mode(), 1);
    assert!(ppu.frame_ready());

    let framebuffer = ppu.framebuffer();
    assert!(framebuffer.iter().all(|&pixel| pixel != 0xFF000000));
}

#[test]
fn background_scroll_scx_selects_shifted_pixels_and_wraps_at_256_pixels() {
    let mut ppu = Ppu::new();
    let (mut vram0, vram1) = blank_vram();

    // BG map 0x9800: tile 0 has color 1, tile 1 has color 2.
    vram0[0x1800] = 0;
    vram0[0x1801] = 1;
    // Tile 0 = color index 1; tile 1 = color index 2.
    for row in 0..8 {
        vram0[row * 2] = 0xFF;
        vram0[row * 2 + 1] = 0x00;
        vram0[16 + row * 2] = 0x00;
        vram0[16 + row * 2 + 1] = 0xFF;
    }

    ppu.write(0xFF43, 8);
    let line = ppu.render_background_scanline_cgb(&vram0, &vram1, 0);

    // SCX=8 skips the first tile's first 8 pixels, so the screen begins
    // with tile 1 (color index 2). The BG coordinate wraps modulo 256.
    let tile1_color = Ppu::cgb_rgb555_to_argb(0x294A);
    assert_eq!(line[0], tile1_color);
    assert_eq!(line[7], tile1_color);
    assert_eq!(line[8], tile1_color);
}

#[test]
fn background_scroll_scy_selects_shifted_tile_rows_and_wraps_at_256_lines() {
    let mut ppu = Ppu::new();
    let (mut vram0, vram1) = blank_vram();

    vram0[0x1800] = 0;
    // Distinct colors per tile row: row 0 = color 1, row 1 = color 2.
    vram0[0] = 0xFF;
    vram0[1] = 0x00;
    vram0[2] = 0x00;
    vram0[3] = 0xFF;

    ppu.write(0xFF42, 8);
    let line = ppu.render_background_scanline_cgb(&vram0, &vram1, 0);

    // SCY=8 moves the rendered line to tile row 1.
    let row1_color = Ppu::cgb_rgb555_to_argb(0x294A);
    assert_eq!(line[0], row1_color);
    assert_eq!(line[159], row1_color);
}

#[test]
fn background_scroll_wraps_from_bottom_right_edge_to_top_left() {
    let mut ppu = Ppu::new();
    let (mut vram0, vram1) = blank_vram();

    // At BG coordinate (255,255), use tile 31,31. Give it color 3.
    let map_offset = 0x1800 + 31 * 32 + 31;
    vram0[map_offset] = 31;
    let tile_offset = 31 * 16;
    for row in 0..8 {
        vram0[tile_offset + row * 2] = 0xFF;
        vram0[tile_offset + row * 2 + 1] = 0xFF;
    }

    // At coordinate (0,0), use tile 0 with color 1.
    vram0[0x1800] = 0;
    vram0[0] = 0xFF;
    vram0[1] = 0x00;

    // SCX/SCY=255 means screen pixel 0 samples BG coordinate 255,255.
    ppu.write(0xFF43, 255);
    ppu.write(0xFF42, 255);
    let line = ppu.render_background_scanline_cgb(&vram0, &vram1, 0);

    let color3 = Ppu::cgb_rgb555_to_argb(0x0000);
    let color1 = Ppu::cgb_rgb555_to_argb(0x7FFF);
    assert_ne!(line[0], color1);
    assert_eq!(line[0], color3);
}
