use super::ppu::Ppu;

fn blank_vram() -> ([u8; 0x2000], [u8; 0x2000]) {
    ([0; 0x2000], [0; 0x2000])
}

#[test]
fn background_scroll_scy_wraps_from_line_255_to_line_0() {
    let mut ppu = Ppu::new();
    let (mut vram0, vram1) = blank_vram();

    // Tile 0 = color index 1.
    for row in 0..8 {
        vram0[row * 2] = 0xFF;
        vram0[row * 2 + 1] = 0x00;
    }

    // Tile 1 = color index 2.
    for row in 0..8 {
        vram0[16 + row * 2] = 0x00;
        vram0[16 + row * 2 + 1] = 0xFF;
    }

    // Row 31 of the background map uses tile 0.
    vram0[0x1800 + 31 * 32] = 0;
    // Row 0 uses tile 1.
    vram0[0x1800] = 1;

    // SCY=255: screen y=0 samples background line 255 (map row 31).
    ppu.write(0xFF42, 255);
    let line_255 = ppu.render_background_scanline_cgb(&vram0, &vram1, 0);

    // SCY=255: screen y=1 wraps to background line 0 (map row 0).
    let line_0 = ppu.render_background_scanline_cgb(&vram0, &vram1, 1);

    let color1 = Ppu::cgb_rgb555_to_argb(0x56B5);
    let color2 = Ppu::cgb_rgb555_to_argb(0x294A);

    assert_eq!(line_255[0], color1);
    assert_eq!(line_255[159], color1);
    assert_eq!(line_0[0], color2);
    assert_eq!(line_0[159], color2);
}
