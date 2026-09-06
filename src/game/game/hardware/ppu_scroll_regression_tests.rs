use super::ppu::Ppu;

fn blank_vram() -> ([u8; 0x2000], [u8; 0x2000]) {
    ([0; 0x2000], [0; 0x2000])
}

#[test]
fn background_scroll_scy_wraps_from_line_255_to_line_0() {
    let mut ppu = Ppu::new();
    let (mut vram0, vram1) = blank_vram();

    // ------------------------------------------------------------
    // TILE DATA
    // ------------------------------------------------------------

    // Tile 0 = color index 1 for every pixel.
    for row in 0..8 {
        vram0[row * 2] = 0xFF;
        vram0[row * 2 + 1] = 0x00;
    }

    // Tile 1 = color index 2 for every pixel.
    for row in 0..8 {
        vram0[16 + row * 2] = 0x00;
        vram0[16 + row * 2 + 1] = 0xFF;
    }

    // ------------------------------------------------------------
    // BG MAP 0x9800
    // ------------------------------------------------------------

    let map_base = 0x9800u16;
    let map_offset = (map_base - 0x8000) as usize;

    // Background map row 31 -> tile 0.
    //
    // bg_y = 255
    // tile_y = 255 / 8 = 31
    //
    // Therefore screen line 0 with SCY=255 must use tile 0.
    let row_31_index = map_offset + 31 * 32;
    vram0[row_31_index] = 0;

    // Background map row 0 -> tile 1.
    //
    // bg_y = 0
    // tile_y = 0
    //
    // Therefore screen line 1 with SCY=255 must use tile 1.
    let row_0_index = map_offset;
    vram0[row_0_index] = 1;

    // ------------------------------------------------------------
    // SCY = 255
    // ------------------------------------------------------------

    ppu.write(0xFF42, 255);

    // Screen line 0:
    //     0 + 255 = 255
    //
    // Must read BG line 255 -> map row 31 -> tile 0.
    let line_255 = ppu.render_background_scanline_cgb(
        &vram0,
        &vram1,
        0,
    );

    // Screen line 1:
    //     1 + 255 = 256 -> 0
    //
    // Must wrap to BG line 0 -> map row 0 -> tile 1.
    let line_0 = ppu.render_background_scanline_cgb(
        &vram0,
        &vram1,
        1,
    );

    // ------------------------------------------------------------
    // EXPECTED PALETTE COLORS
    // ------------------------------------------------------------

    let color1 = Ppu::cgb_rgb555_to_argb(0x56B5);
    let color2 = Ppu::cgb_rgb555_to_argb(0x294A);

    // ------------------------------------------------------------
    // DIAGNOSTICS
    // ------------------------------------------------------------

    let bg_y_255 = (255usize + 255usize) & 0xFF;
    let bg_y_0 = (1usize + 255usize) & 0xFF;

    let tile_y_255 = bg_y_255 >> 3;
    let tile_y_0 = bg_y_0 >> 3;

    let tile_x_255 = 0usize >> 3;
    let tile_x_0 = 0usize >> 3;

    let map_index_255 =
        map_offset + tile_y_255 * 32 + tile_x_255;

    let map_index_0 =
        map_offset + tile_y_0 * 32 + tile_x_0;

    let tile_index_255 = vram0[map_index_255];
    let tile_index_0 = vram0[map_index_0];

    let attr_255 =
        Ppu::background_tile_attributes(
            &vram1,
            0,
            bg_y_255 as u8,
            map_base,
        );

    let attr_0 =
        Ppu::background_tile_attributes(
            &vram1,
            0,
            bg_y_0 as u8,
            map_base,
        );

    let (palette_255, bank_255, flip_x_255, flip_y_255, priority_255) =
        Ppu::background_tile_attribute_info(attr_255);

    let (palette_0, bank_0, flip_x_0, flip_y_0, priority_0) =
        Ppu::background_tile_attribute_info(attr_0);

    eprintln!();
    eprintln!("========== SCY REGRESSION DIAGNOSTIC ==========");
    eprintln!();

    eprintln!("--- SCREEN LINE 0 ---");
    eprintln!("screen_y       = 0");
    eprintln!("SCY            = {}", 255);
    eprintln!("bg_y           = {}", bg_y_255);
    eprintln!("tile_x         = {}", tile_x_255);
    eprintln!("tile_y         = {}", tile_y_255);
    eprintln!("map_base       = {:#06X}", map_base);
    eprintln!("map_index      = {:#06X}", map_index_255);
    eprintln!("tile_index     = {}", tile_index_255);
    eprintln!("attr           = {:#04X}", attr_255);
    eprintln!("palette        = {}", palette_255);
    eprintln!("bank           = {}", bank_255);
    eprintln!("flip_x         = {}", flip_x_255);
    eprintln!("flip_y         = {}", flip_y_255);
    eprintln!("priority       = {}", priority_255);
    eprintln!("actual[0]      = {:#010X}", line_255[0]);
    eprintln!("actual[159]    = {:#010X}", line_255[159]);
    eprintln!("expected       = {:#010X}", color1);
    eprintln!();

    eprintln!("--- SCREEN LINE 1 ---");
    eprintln!("screen_y       = 1");
    eprintln!("SCY            = {}", 255);
    eprintln!("bg_y           = {}", bg_y_0);
    eprintln!("tile_x         = {}", tile_x_0);
    eprintln!("tile_y         = {}", tile_y_0);
    eprintln!("map_base       = {:#06X}", map_base);
    eprintln!("map_index      = {:#06X}", map_index_0);
    eprintln!("tile_index     = {}", tile_index_0);
    eprintln!("attr           = {:#04X}", attr_0);
    eprintln!("palette        = {}", palette_0);
    eprintln!("bank           = {}", bank_0);
    eprintln!("flip_x         = {}", flip_x_0);
    eprintln!("flip_y         = {}", flip_y_0);
    eprintln!("priority       = {}", priority_0);
    eprintln!("actual[0]      = {:#010X}", line_0[0]);
    eprintln!("actual[159]    = {:#010X}", line_0[159]);
    eprintln!("expected       = {:#010X}", color2);
    eprintln!();

    eprintln!("--- TILE DATA ---");

    let tile0 = Ppu::background_tile_data(
        &vram0,
        tile_index_255,
        0x8000,
    );

    let tile1 = Ppu::background_tile_data(
        &vram0,
        tile_index_0,
        0x8000,
    );

    eprintln!(
        "tile 0 row 0 = {:?}",
        Ppu::decode_tile_row(&tile0, 0)
    );

    eprintln!(
        "tile 1 row 0 = {:?}",
        Ppu::decode_tile_row(&tile1, 0)
    );

    eprintln!();
    eprintln!("===============================================");
    eprintln!();

    // ------------------------------------------------------------
    // ASSERTIONS
    // ------------------------------------------------------------

    assert_eq!(
        line_255[0],
        color1,
        "SCY=255, screen y=0 should sample BG line 255 / map row 31 / tile 0"
    );

    assert_eq!(
        line_255[159],
        color1,
        "SCY=255, screen y=0 should use tile 0 across the scanline"
    );

    assert_eq!(
        line_0[0],
        color2,
        "SCY=255, screen y=1 should wrap to BG line 0 / map row 0 / tile 1"
    );

    assert_eq!(
        line_0[159],
        color2,
        "SCY=255, screen y=1 should use tile 1 across the scanline"
    );
}