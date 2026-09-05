use minifb::{Key, KeyRepeat, MouseMode, Window, WindowOptions};
use crate::game::Game;

const TILE: usize = 8;
const COLS: usize = 16;
const ROWS: usize = 24;
const LABEL: usize = 8;

pub struct TileViewer {
    window: Window,
    scale: usize,
    bank: u8,
    buffer: Vec<u32>,
    selected: Option<usize>,
}

impl TileViewer {
    pub fn new() -> Self {
        Self::with_scale(2, 0)
    }

    fn dimensions(scale: usize) -> (usize, usize) {
        (COLS * TILE * scale, ROWS * (TILE + LABEL) * scale)
    }

    fn with_scale(scale: usize, bank: u8) -> Self {
        let (w, h) = Self::dimensions(scale);
        let window = Window::new(
            &format!("VRAM Tile Viewer - Bank {} - {}x", bank, scale),
            w, h,
            WindowOptions { resize: false, ..WindowOptions::default() },
        ).expect("Nie można utworzyć okna Tile Viewer");
        Self { window, scale, bank, buffer: vec![0; w * h], selected: None }
    }

    pub fn is_open(&self) -> bool { self.window.is_open() }

    pub fn update(&mut self, game: &mut Game) {
        self.handle_input();
        if !self.window.is_open() { return; }

        let previous_bank = game.read(0xFF4F) & 1;
        game.write(0xFF4F, self.bank);
        let mut vram = [0u8; 0x1800];
        for (i, byte) in vram.iter_mut().enumerate() {
            *byte = game.read(0x8000 + i as u16);
        }
        game.write(0xFF4F, previous_bank);

        self.render(&vram);
        let (w, h) = Self::dimensions(self.scale);
        self.window.update_with_buffer(&self.buffer, w, h)
            .expect("Nie można zaktualizować okna Tile Viewer");
    }

    fn handle_input(&mut self) {
        let requested_scale = if self.window.is_key_pressed(Key::Key2, KeyRepeat::No) { Some(2) }
            else if self.window.is_key_pressed(Key::Key4, KeyRepeat::No) { Some(4) }
            else if self.window.is_key_pressed(Key::Key8, KeyRepeat::No) { Some(8) }
            else { None };
        if let Some(scale) = requested_scale { self.recreate(scale); }

        if self.window.is_key_pressed(Key::B, KeyRepeat::No) {
            self.bank ^= 1;
            self.window.set_title(&format!("VRAM Tile Viewer - Bank {} - {}x", self.bank, self.scale));
        }

        if let Some((mx, my)) = self.window.get_mouse_pos(MouseMode::Discard) {
            let cell_w = TILE * self.scale;
            let cell_h = (TILE + LABEL) * self.scale;
            let col = mx as usize / cell_w;
            let row = my as usize / cell_h;
            if col < COLS && row < ROWS { self.selected = Some(row * COLS + col); }
        }
    }

    fn recreate(&mut self, scale: usize) {
        if scale == self.scale { return; }
        let selected = self.selected;
        let bank = self.bank;
        *self = Self::with_scale(scale, bank);
        self.selected = selected;
    }

    fn render(&mut self, vram: &[u8; 0x1800]) {
        self.buffer.fill(0xFF202020);
        let cell_w = TILE * self.scale;
        let cell_h = (TILE + LABEL) * self.scale;

        for tile_index in 0..384usize {
            let col = tile_index % COLS;
            let row = tile_index / COLS;
            let ox = col * cell_w;
            let oy = row * cell_h;
            let base = tile_index * 16;

            for y in 0..8usize {
                let lo = vram[base + y * 2];
                let hi = vram[base + y * 2 + 1];
                for x in 0..8usize {
                    let bit = 7 - x;
                    let index = ((hi >> bit) & 1) << 1 | ((lo >> bit) & 1);
                    let shade = match index { 0 => 0xFFFFFFFF, 1 => 0xFFAAAAAA, 2 => 0xFF555555, _ => 0xFF000000 };
                    self.fill_rect(ox + x * self.scale, oy + (y + LABEL) * self.scale, self.scale, self.scale, shade);
                }
            }

            if self.selected == Some(tile_index) {
                self.stroke_rect(ox, oy + LABEL * self.scale, cell_w, TILE * self.scale, 0xFFFFFF00);
            }
            self.draw_index(tile_index, ox + 1, oy + 1);
        }
    }

    fn fill_rect(&mut self, x: usize, y: usize, w: usize, h: usize, color: u32) {
        let width = Self::dimensions(self.scale).0;
        let height = Self::dimensions(self.scale).1;
        for py in y..(y + h).min(height) {
            let start = py * width + x.min(width);
            let end = (x + w).min(width);
            if start < py * width + end { self.buffer[start..py * width + end].fill(color); }
        }
    }

    fn stroke_rect(&mut self, x: usize, y: usize, w: usize, h: usize, color: u32) {
        for px in x..x + w { self.set_pixel(px, y, color); self.set_pixel(px, y + h - 1, color); }
        for py in y..y + h { self.set_pixel(x, py, color); self.set_pixel(x + w - 1, py, color); }
    }

    fn set_pixel(&mut self, x: usize, y: usize, color: u32) {
        let width = Self::dimensions(self.scale).0;
        if x < width && y < Self::dimensions(self.scale).1 { self.buffer[y * width + x] = color; }
    }

    fn draw_index(&mut self, value: usize, ox: usize, oy: usize) {
        for (i, ch) in format!("{:03X}", value).bytes().enumerate() {
            if let Some(g) = glyph(ch) {
                for y in 0..5 { for x in 0..3 {
                    if g[y][x] { self.set_pixel(ox + i * 4 + x, oy + y, 0xFFFF0000); }
                }}
            }
        }
    }
}

fn glyph(ch: u8) -> Option<[[bool; 3]; 5]> {
    let rows = ["111101101101111", "010110010010111", "110001111100111", "110001111001111", "101101111001001", "111100111001111", "111100111101111", "111001001001001", "111101111101111", "111101111001111", "111101111101101", "110101110101110", "111100100100111", "110101101101110", "111100111100111", "111100111100100"];
    let i = match ch { b'0'..=b'9' => (ch - b'0') as usize, b'A'..=b'F' => 10 + (ch - b'A') as usize, _ => return None };
    let s = rows[i];
    let mut out = [[false; 3]; 5];
    for y in 0..5 { for x in 0..3 { out[y][x] = s.as_bytes()[y * 3 + x] == b'1'; } }
    Some(out)
}
