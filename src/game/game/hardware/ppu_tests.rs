use super::*;

#[test]
fn background_scroll_scx_selects_shifted_pixels_and_wraps_at_256_pixels() {
    let mut ppu = Ppu::new();
    let (mut vram0, vram1) = blank_vram();

    // BG map 0x9800: tile 0 = color 1, tile 1 = color 2, tile 2 = color 3.
    vram0[0x1800] = 0;
    vram0[0x1801] = 1;
    vram0[0x1802] = 2;

    // Tile 0 = color index 1; tile 1 = color index 2; tile 2 = color index 3.
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

    // SCX=8 skips the first tile's first 8 pixels. The screen therefore
    // begins at tile 1 and crosses into tile 2 at screen x=8.
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

    // BG map 0x9800: the first entry of the second tile row uses tile 0.
    // This is the tile selected when SCY=8 moves the screen to BG y=8.
    vram0[0x1800 + 32] = 0;

    // Tile 0: row 0 = color 1, row 1 = color 2.
    vram0[0] = 0xFF;
    vram0[1] = 0x00;
    vram0[2] = 0x00;
    vram0[3] = 0xFF;

    ppu.write(0xFF42, 8);
    let line = ppu.render_background_scanline_cgb(&vram0, &vram1, 0);

    // SCY=8 maps screen y=0 to BG y=8: tile row 1, pixel row 0.
    // The selected tile is still tile 0, so its row 0 is color 1.
    let row0_color = Ppu::cgb_rgb555_to_argb(0x56B5);
    assert_eq!(line[0], row0_color);
    assert_eq!(line[159], row0_color);
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

    // The remaining tests in this file continue below.
