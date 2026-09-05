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

    // Tile 0 row 0: color index 1 on all eight pixels.
    vram0[0] = 0xFF;
    vram0[1] = 0x00;
    // BG map 0x9800 points to tile 0.
    vram0[0x1800] = 0;

    let line = ppu.render_background_scanline_cgb(&vram0, &vram1, 0);
    assert!(line.iter().all(|&pixel| pixel != 0xFF000000));
}
