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
    assert_eq!(ppu.read(0xFF41) & 0x03, 0);

    ppu.step(1000, &oam, &vram0, &vram1);
    assert_eq!(ppu.read(0xFF44), 0);

    ppu.write(0xFF40, 0x91);
    assert_eq!(ppu.read(0xFF44), 0);
    assert_eq!(ppu.mode(), 2);
    assert_eq!(ppu.read(0xFF41) & 0x03, 2);
}

#[test]
fn mode_3_length_includes_scroll_and_sprite_fetch_penalties() {
    let mut ppu = Ppu::new();
    let (vram0, vram1) = blank_vram();
    let oam = [0; 0xA0];
    ppu.write(0xFF43, 3);
    ppu.step(80, &oam, &vram0, &vram1);
    ppu.step(174, &oam, &vram0, &vram1);
    assert_eq!(ppu.mode(), 3);
    ppu.step(1, &oam, &vram0, &vram1);
    assert_eq!(ppu.mode(), 0);

    let mut with_sprite = Ppu::new();
    let mut sprite_oam = [0; 0xA0];
    sprite_oam[0] = 16;
    sprite_oam[1] = 8;
    with_sprite.step(80, &sprite_oam, &vram0, &vram1);
    with_sprite.step(182, &sprite_oam, &vram0, &vram1);
    assert_eq!(with_sprite.mode(), 3);
    with_sprite.step(1, &sprite_oam, &vram0, &vram1);
    assert_eq!(with_sprite.mode(), 0);

    // Mode 0 contracts by the same amount, preserving the fixed 456-dot
    // scanline timing required by software polling LY/STAT.
    with_sprite.step(456 - 80 - 183, &sprite_oam, &vram0, &vram1);
    assert_eq!(with_sprite.ly(), 1);
    assert_eq!(with_sprite.mode(), 2);
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
fn window_uses_its_own_tile_map_at_wx_minus_seven() {
    let mut ppu = Ppu::new();
    let (mut vram0, vram1) = blank_vram();
    let oam = [0; 0xA0];

    // The background map uses tile 0/color 1, while the window map uses
    // tile 1/color 2. WX=7 places the window's left edge at screen x=0.
    for row in 0..8 {
        vram0[row * 2] = 0xFF;
        vram0[row * 2 + 1] = 0x00;
        vram0[16 + row * 2] = 0x00;
        vram0[16 + row * 2 + 1] = 0xFF;
    }
    vram0[0x1800] = 0;
    vram0[0x1C00] = 1;

    ppu.write(0xFF40, 0xF1); // LCD, BG, window enabled; window map 9C00.
    ppu.write(0xFF4A, 0);
    ppu.write(0xFF4B, 7);
    ppu.step(258, &oam, &vram0, &vram1);

    assert_eq!(ppu.framebuffer()[0], Ppu::cgb_rgb555_to_argb(0x294A));
}

#[test]
fn background_map_tile_80_uses_vram_8800_in_unsigned_mode() {
    let ppu = Ppu::new();
    let (mut vram0, vram1) = blank_vram();

    vram0[0x1800] = 0x80;
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

    for offset in 0..(32 * 32) { vram0[0x1800 + offset] = 0x80; }
    for row in 0..8 {
        vram0[0x0800 + row * 2] = 0xFF;
        vram0[0x0800 + row * 2 + 1] = 0x00;
    }

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

    vram0[0x1800] = 0;
    vram0[0x1801] = 1;
    vram0[0x1802] = 2;

    for row in 0..8 {
        vram0[row * 2] = 0xFF;
        vram0[row * 2 + 1] = 0x00;
        vram0[16 + row * 2] = 0x00;
        vram0[16 + row * 2 + 1] = 0xFF;
        vram0[32 + row * 2] = 0xFF;
        vram0[32 + row * 2 + 1] = 0xFF;
    }

    ppu.write(0xFF43, 8);
    let line = ppu.render_background_scanline_cgb(&vram0, &vram1, 0);

    let tile1_color = Ppu::cgb_rgb555_to_argb(0x294A);
    let tile2_color = Ppu::cgb_rgb555_to_argb(0x0000);
    assert_eq!(line[0], tile1_color);
    assert_eq!(line[7], tile1_color);
    assert_eq!(line[8], tile2_color);
}

#[test]
fn background_scroll_scy_selects_shifted_tile_rows_and_wraps_at_256_lines() {
    let mut ppu = Ppu::new();
    let (mut vram0, vram1) = blank_vram();

    vram0[0x1800 + 32] = 0;
    vram0[0] = 0xFF;
    vram0[1] = 0x00;
    vram0[2] = 0x00;
    vram0[3] = 0xFF;

    ppu.write(0xFF42, 8);
    let line = ppu.render_background_scanline_cgb(&vram0, &vram1, 0);

    let row0_color = Ppu::cgb_rgb555_to_argb(0x56B5);
    assert_eq!(line[0], row0_color);
    assert_eq!(line[159], row0_color);
}

#[test]
fn background_scroll_wraps_from_bottom_right_edge_to_top_left() {
    let mut ppu = Ppu::new();
    let (mut vram0, vram1) = blank_vram();

    let map_offset = 0x1800 + 31 * 32 + 31;
    vram0[map_offset] = 31;
    let tile_offset = 31 * 16;
    for row in 0..8 {
        vram0[tile_offset + row * 2] = 0xFF;
        vram0[tile_offset + row * 2 + 1] = 0xFF;
    }

    vram0[0x1800] = 0;
    vram0[0] = 0xFF;
    vram0[1] = 0x00;

    ppu.write(0xFF43, 255);
    ppu.write(0xFF42, 255);
    let line = ppu.render_background_scanline_cgb(&vram0, &vram1, 0);

    let color3 = Ppu::cgb_rgb555_to_argb(0x0000);
    let color1 = Ppu::cgb_rgb555_to_argb(0x7FFF);
    assert_ne!(line[0], color1);
    assert_eq!(line[0], color3);
}

#[test]
fn sprite_rendering_reads_oam_without_changing_bg_scroll() {
    let mut ppu = Ppu::new();
    let (mut vram0, vram1) = blank_vram();
    let mut oam = [0u8; 0xA0];

    // Tile 1 is a solid color-1 sprite. The BG remains tile 0/color 0.
    for row in 0..8 {
        vram0[16 + row * 2] = 0xFF;
        vram0[16 + row * 2 + 1] = 0x00;
    }

    // OAM coordinates are offset by (8, 16), so (8, 16) puts the sprite
    // at screen position (0, 0).
    oam[0] = 16;
    oam[1] = 8;
    oam[2] = 1;
    oam[3] = 0;

    // Enable LCD, BG and OBJ. This advances through mode 2 and mode 3,
    // causing scanline 0 to be rendered exactly once.
    ppu.write(0xFF40, 0x93);
    ppu.step(263, &oam, &vram0, &vram1);

    let expected_sprite_color = Ppu::cgb_rgb555_to_argb(0x56B5);
    let framebuffer = ppu.framebuffer();
    assert_eq!(framebuffer[0], expected_sprite_color);
    assert_eq!(framebuffer[7], expected_sprite_color);
    assert_eq!(framebuffer[8], Ppu::cgb_rgb555_to_argb(0x7FFF));

    // SCX/SCY were not modified by sprite rendering.
    assert_eq!(ppu.read(0xFF43), 0);
    assert_eq!(ppu.read(0xFF42), 0);
}

#[test]
fn sprite_scanline_uses_only_the_first_ten_oam_entries() {
    let mut ppu = Ppu::new();
    let (mut vram0, vram1) = blank_vram();
    let mut oam = [0u8; 0xA0];

    // The first ten OAM entries are on the scanline but fully off-screen.
    // They still consume the hardware's ten-sprite selection budget.
    for index in 0..10 {
        let base = index * 4;
        oam[base] = 16;
        oam[base + 1] = 0;
    }

    // Entry 10 would be visible if the renderer considered more than ten.
    for row in 0..8 {
        vram0[16 + row * 2] = 0xFF;
        vram0[16 + row * 2 + 1] = 0x00;
    }
    oam[40] = 16;
    oam[41] = 8;
    oam[42] = 1;

    ppu.write(0xFF40, 0x93);
    ppu.step(362, &oam, &vram0, &vram1);

    assert_eq!(ppu.framebuffer()[0], Ppu::cgb_rgb555_to_argb(0x7FFF));
}

#[test]
fn cgb_background_priority_tile_is_drawn_over_a_sprite() {
    let mut ppu = Ppu::new();
    let (mut vram0, mut vram1) = blank_vram();
    let mut oam = [0u8; 0xA0];

    // BG tile 0 is color 1 and has the CGB priority attribute. Sprite tile 1
    // is color 2 at the same screen position.
    for row in 0..8 {
        vram0[row * 2] = 0xFF;
        vram0[row * 2 + 1] = 0x00;
        vram0[16 + row * 2] = 0x00;
        vram0[16 + row * 2 + 1] = 0xFF;
    }
    vram1[0x1800] = 0x80;
    oam[0] = 16;
    oam[1] = 8;
    oam[2] = 1;

    ppu.write(0xFF40, 0x93);
    ppu.step(263, &oam, &vram0, &vram1);

    assert_eq!(ppu.framebuffer()[0], Ppu::cgb_rgb555_to_argb(0x56B5));
}
