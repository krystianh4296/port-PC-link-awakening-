use crate::bus::Bus;
use minifb::{Key, KeyRepeat, Window, WindowOptions};
use std::time::{Duration, Instant};

const WIDTH: usize = 1180;
const HEIGHT: usize = 820;
const CHAR_WIDTH: usize = 8;
const ROW_HEIGHT: usize = 18;
const HEADER_HEIGHT: usize = 116;
const BYTES_PER_ROW: usize = 16;
const VISIBLE_ROWS: usize = (HEIGHT - HEADER_HEIGHT) / ROW_HEIGHT;
const REFRESH_INTERVAL: Duration = Duration::from_millis(100);

const BG: u32 = 0x101010;
const PANEL: u32 = 0x181818;
const BORDER: u32 = 0x505050;
const TEXT: u32 = 0xD0D0D0;
const HEADER: u32 = 0xFFFFFF;
const ADDRESS: u32 = 0x70B7FF;
const CHANGED: u32 = 0xFFFF55;
const ASCII: u32 = 0xAAAAAA;
const CURSOR: u32 = 0x70B7FF;
const PAUSED: u32 = 0xFFAA55;

#[derive(Clone, Copy, PartialEq, Eq)]
enum InputField {
    Start,
    End,
}

pub struct MemoryWatch {
    pub window: Window,
    start: u16,
    end: u16,
    selected_field: InputField,
    start_digits: [u8; 4],
    end_digits: [u8; 4],
    cursor: usize,
    scroll_row: usize,
    previous_memory: Vec<u8>,
    current_memory: Vec<u8>,
    last_refresh: Instant,
    paused: bool,
    buffer: Vec<u32>,
}

impl MemoryWatch {
    pub fn new() -> Self {
        let mut window = Window::new(
            "Memory Watch",
            WIDTH,
            HEIGHT,
            WindowOptions::default(),
        )
        .expect("Nie udało się utworzyć okna Memory Watch");

        window.set_target_fps(60);

        let start = 0x0000;
        let end = 0xD000;

        Self {
            window,
            start,
            end,
            selected_field: InputField::Start,
            start_digits: hex_digits(start),
            end_digits: hex_digits(end),
            cursor: 0,
            scroll_row: 0,
            previous_memory: Vec::new(),
            current_memory: Vec::new(),
            last_refresh: Instant::now() - REFRESH_INTERVAL,
            paused: false,
            buffer: vec![BG; WIDTH * HEIGHT],
        }
    }

    pub fn update(&mut self, bus: &Bus) {
        if !self.window.is_open() {
            return;
        }

        self.handle_input();

        if !self.paused && self.last_refresh.elapsed() >= REFRESH_INTERVAL {
            self.refresh(bus);
            self.last_refresh = Instant::now();
        }

        self.render();

        self.window
            .update_with_buffer(&self.buffer, WIDTH, HEIGHT)
            .expect("Błąd aktualizacji okna Memory Watch");
    }

    fn range_rows(&self) -> usize {
        if self.start > self.end {
            0
        } else {
            let bytes = self.end as usize - self.start as usize + 1;
            bytes.div_ceil(BYTES_PER_ROW)
        }
    }

    fn max_scroll(&self) -> usize {
        self.range_rows().saturating_sub(VISIBLE_ROWS)
    }

    fn clamp_scroll(&mut self) {
        self.scroll_row = self.scroll_row.min(self.max_scroll());
    }

    fn refresh(&mut self, bus: &Bus) {
        if self.start > self.end {
            self.current_memory.clear();
            self.previous_memory.clear();
            self.scroll_row = 0;
            return;
        }

        let size = self.end as usize - self.start as usize + 1;
        let mut new_memory = Vec::with_capacity(size);

        for offset in 0..size {
            new_memory.push(bus.read_debug((self.start as usize + offset) as u16));
        }

        self.previous_memory = std::mem::replace(&mut self.current_memory, new_memory);
        self.clamp_scroll();
    }

    fn handle_input(&mut self) {
        if self.window.is_key_pressed(Key::Escape, KeyRepeat::No) {
            self.window.set_position(-10000, -10000);
            return;
        }

        if self.window.is_key_pressed(Key::Tab, KeyRepeat::No) {
            self.selected_field = match self.selected_field {
                InputField::Start => InputField::End,
                InputField::End => InputField::Start,
            };
            self.cursor = 0;
        }

        if self.window.is_key_pressed(Key::Enter, KeyRepeat::No) {
            self.apply_range();
        }

        if self.window.is_key_pressed(Key::Space, KeyRepeat::No) {
            self.paused = !self.paused;
        }

        if self.window.is_key_pressed(Key::Left, KeyRepeat::Yes) {
            self.cursor = self.cursor.saturating_sub(1);
        }

        if self.window.is_key_pressed(Key::Right, KeyRepeat::Yes) {
            self.cursor = (self.cursor + 1).min(3);
        }

        if self.window.is_key_pressed(Key::Up, KeyRepeat::Yes) {
            self.scroll_row = self.scroll_row.saturating_sub(1);
        }

        if self.window.is_key_pressed(Key::Down, KeyRepeat::Yes) {
            self.scroll_row = (self.scroll_row + 1).min(self.max_scroll());
        }

        if self.window.is_key_pressed(Key::PageUp, KeyRepeat::No) {
            self.scroll_row = self.scroll_row.saturating_sub(VISIBLE_ROWS);
        }

        if self.window.is_key_pressed(Key::PageDown, KeyRepeat::No) {
            self.scroll_row = (self.scroll_row + VISIBLE_ROWS).min(self.max_scroll());
        }

        if self.window.is_key_pressed(Key::Home, KeyRepeat::No) {
            self.scroll_row = 0;
        }

        if self.window.is_key_pressed(Key::End, KeyRepeat::No) {
            self.scroll_row = self.max_scroll();
        }

        self.handle_hex_keys();
    }

    fn handle_hex_keys(&mut self) {
        let keys = [
            (Key::Key0, 0), (Key::Key1, 1), (Key::Key2, 2), (Key::Key3, 3),
            (Key::Key4, 4), (Key::Key5, 5), (Key::Key6, 6), (Key::Key7, 7),
            (Key::Key8, 8), (Key::Key9, 9), (Key::A, 10), (Key::B, 11),
            (Key::C, 12), (Key::D, 13), (Key::E, 14), (Key::F, 15),
        ];

        let digit = keys.iter().find_map(|(key, value)| {
            self.window.is_key_pressed(*key, KeyRepeat::No).then_some(*value)
        });

        let Some(value) = digit else { return; };

        match self.selected_field {
            InputField::Start => {
                self.start_digits[self.cursor] = value;
                self.start = digits_to_u16(self.start_digits);
            }
            InputField::End => {
                self.end_digits[self.cursor] = value;
                self.end = digits_to_u16(self.end_digits);
            }
        }

        // Start i End są zatwierdzane dopiero Enterem.
        // Nie zamieniamy ich podczas wpisywania kolejnych cyfr.
        if self.cursor < 3 {
            self.cursor += 1;
        }
    }

    fn apply_range(&mut self) {
        if self.start > self.end {
            std::mem::swap(&mut self.start, &mut self.end);
            self.start_digits = hex_digits(self.start);
            self.end_digits = hex_digits(self.end);
        }

        self.scroll_row = 0;
        self.previous_memory.clear();
        self.current_memory.clear();
        self.last_refresh = Instant::now() - REFRESH_INTERVAL;
    }

    fn render(&mut self) {
        self.buffer.fill(BG);

        self.draw_text(20, 15, "Memory Watch", HEADER);

        self.fill_rect(0, 36, WIDTH, 43, PANEL);
        self.draw_line(0, 36, WIDTH, 36, BORDER);
        self.draw_line(0, 79, WIDTH, 79, BORDER);

        self.draw_text(20, 51, "Start:", TEXT);
        self.draw_text(76, 51, &format!("[{:04X}]", self.start), TEXT);

        self.draw_text(205, 51, "End:", TEXT);
        self.draw_text(252, 51, &format!("[{:04X}]", self.end), TEXT);

        self.draw_text(400, 51, "Refresh: 100 ms", TEXT);

        let pause_text = if self.paused { "[Resume]" } else { "[Pause]" };
        self.draw_text(580, 51, pause_text, if self.paused { PAUSED } else { TEXT });

        self.draw_text(760, 51, "TAB=field ENTER=apply SPACE=pause", TEXT);

        let cursor_x = match self.selected_field {
            InputField::Start => 84 + self.cursor * CHAR_WIDTH,
            InputField::End => 260 + self.cursor * CHAR_WIDTH,
        };
        self.draw_rect(cursor_x, 64, 6, 2, CURSOR);

        self.draw_text(20, 91, "ADDRESS", ADDRESS);
        for i in 0..BYTES_PER_ROW {
            self.draw_text(100 + i * 38, 91, &format!("{:02X}", i), TEXT);
        }
        self.draw_text(750, 91, "ASCII", ASCII);
        self.draw_line(0, 109, WIDTH, 109, BORDER);

        let total_rows = self.range_rows();

        for row in 0..VISIBLE_ROWS {
            let memory_index = (self.scroll_row + row) * BYTES_PER_ROW;
            if memory_index >= self.current_memory.len() {
                break;
            }

            let y = HEADER_HEIGHT + row * ROW_HEIGHT;
            let address = self.start as usize + memory_index;

            self.draw_text(20, y, &format!("{:04X}", address), ADDRESS);

            let mut ascii = String::with_capacity(BYTES_PER_ROW);

            for column in 0..BYTES_PER_ROW {
                let index = memory_index + column;
                if index >= self.current_memory.len() {
                    break;
                }

                let value = self.current_memory[index];
                let changed = index >= self.previous_memory.len()
                    || self.previous_memory[index] != value;

                self.draw_text(
                    100 + column * 38,
                    y,
                    &format!("{:02X}", value),
                    if changed { CHANGED } else { TEXT },
                );

                ascii.push(if value.is_ascii_graphic() { value as char } else { '.' });
            }

            self.draw_text(750, y, &ascii, ASCII);
        }

        // Informacja o pozycji przewijania. Dla C000-CFFF będzie 256 wierszy.
        self.draw_text(
            930,
            91,
            &format!("{}/{}", self.scroll_row + 1, total_rows.max(1)),
            TEXT,
        );
    }

    fn draw_line(&mut self, x1: usize, y1: usize, x2: usize, _y2: usize, color: u32) {
        for x in x1.min(WIDTH)..x2.min(WIDTH) {
            if y1 < HEIGHT {
                self.buffer[y1 * WIDTH + x] = color;
            }
        }
    }

    fn draw_rect(&mut self, x: usize, y: usize, w: usize, h: usize, color: u32) {
        for yy in y.min(HEIGHT)..(y + h).min(HEIGHT) {
            for xx in x.min(WIDTH)..(x + w).min(WIDTH) {
                self.buffer[yy * WIDTH + xx] = color;
            }
        }
    }

    fn fill_rect(&mut self, x: usize, y: usize, w: usize, h: usize, color: u32) {
        self.draw_rect(x, y, w, h, color);
    }

    fn draw_text(&mut self, x: usize, y: usize, text: &str, color: u32) {
        let mut cursor_x = x;
        for c in text.chars() {
            self.draw_char(cursor_x, y, c, color);
            cursor_x += CHAR_WIDTH;
        }
    }

    fn draw_char(&mut self, x: usize, y: usize, c: char, color: u32) {
        for (row, bits) in glyph(c).iter().enumerate() {
            for col in 0..5 {
                if bits & (1 << (4 - col)) != 0 {
                    let px = x + col;
                    let py = y + row;
                    if px < WIDTH && py < HEIGHT {
                        self.buffer[py * WIDTH + px] = color;
                    }
                }
            }
        }
    }
}

fn hex_digits(value: u16) -> [u8; 4] {
    [
        ((value >> 12) & 0xF) as u8,
        ((value >> 8) & 0xF) as u8,
        ((value >> 4) & 0xF) as u8,
        (value & 0xF) as u8,
    ]
}

fn digits_to_u16(digits: [u8; 4]) -> u16 {
    ((digits[0] as u16) << 12)
        | ((digits[1] as u16) << 8)
        | ((digits[2] as u16) << 4)
        | digits[3] as u16
}

fn glyph(c: char) -> [u8; 7] {
    match c.to_ascii_uppercase() {
        '0' => [0x0E,0x11,0x13,0x15,0x19,0x11,0x0E],
        '1' => [0x04,0x0C,0x04,0x04,0x04,0x04,0x0E],
        '2' => [0x0E,0x11,0x01,0x02,0x04,0x08,0x1F],
        '3' => [0x1E,0x01,0x01,0x0E,0x01,0x01,0x1E],
        '4' => [0x02,0x06,0x0A,0x12,0x1F,0x02,0x02],
        '5' => [0x1F,0x10,0x10,0x1E,0x01,0x01,0x1E],
        '6' => [0x06,0x08,0x10,0x1E,0x11,0x11,0x0E],
        '7' => [0x1F,0x01,0x02,0x04,0x08,0x08,0x08],
        '8' => [0x0E,0x11,0x11,0x0E,0x11,0x11,0x0E],
        '9' => [0x0E,0x11,0x11,0x0F,0x01,0x02,0x0C],
        'A' => [0x0E,0x11,0x11,0x1F,0x11,0x11,0x11],
        'B' => [0x1E,0x11,0x11,0x1E,0x11,0x11,0x1E],
        'C' => [0x0E,0x11,0x10,0x10,0x10,0x11,0x0E],
        'D' => [0x1E,0x11,0x11,0x11,0x11,0x11,0x1E],
        'E' => [0x1F,0x10,0x10,0x1E,0x10,0x10,0x1F],
        'F' => [0x1F,0x10,0x10,0x1E,0x10,0x10,0x10],
        'G' => [0x0E,0x11,0x10,0x17,0x11,0x11,0x0E],
        'H' => [0x11,0x11,0x11,0x1F,0x11,0x11,0x11],
        'I' => [0x0E,0x04,0x04,0x04,0x04,0x04,0x0E],
        'J' => [0x01,0x01,0x01,0x01,0x11,0x11,0x0E],
        'K' => [0x11,0x12,0x14,0x18,0x14,0x12,0x11],
        'L' => [0x10,0x10,0x10,0x10,0x10,0x10,0x1F],
        'M' => [0x11,0x1B,0x15,0x15,0x11,0x11,0x11],
        'N' => [0x11,0x19,0x15,0x13,0x11,0x11,0x11],
        'O' => [0x0E,0x11,0x11,0x11,0x11,0x11,0x0E],
        'P' => [0x1E,0x11,0x11,0x1E,0x10,0x10,0x10],
        'Q' => [0x0E,0x11,0x11,0x11,0x15,0x12,0x0D],
        'R' => [0x1E,0x11,0x11,0x1E,0x14,0x12,0x11],
        'S' => [0x0F,0x10,0x10,0x0E,0x01,0x01,0x1E],
        'T' => [0x1F,0x04,0x04,0x04,0x04,0x04,0x04],
        'U' => [0x11,0x11,0x11,0x11,0x11,0x11,0x0E],
        'V' => [0x11,0x11,0x11,0x11,0x0A,0x0A,0x04],
        'W' => [0x11,0x11,0x11,0x15,0x15,0x1B,0x11],
        'X' => [0x11,0x11,0x0A,0x04,0x0A,0x11,0x11],
        'Y' => [0x11,0x11,0x0A,0x04,0x04,0x04,0x04],
        'Z' => [0x1F,0x01,0x02,0x04,0x08,0x10,0x1F],
        ':' => [0x00,0x04,0x04,0x00,0x04,0x04,0x00],
        '[' => [0x0E,0x08,0x08,0x08,0x08,0x08,0x0E],
        ']' => [0x0E,0x02,0x02,0x02,0x02,0x02,0x0E],
        '-' => [0x00,0x00,0x00,0x1F,0x00,0x00,0x00],
        '=' => [0x00,0x1F,0x00,0x1F,0x00,0x00,0x00],
        '/' => [0x01,0x02,0x02,0x04,0x08,0x08,0x10],
        '.' => [0x00,0x00,0x00,0x00,0x00,0x0C,0x0C],
        '|' => [0x04,0x04,0x04,0x04,0x04,0x04,0x04],
        ' ' => [0x00,0x00,0x00,0x00,0x00,0x00,0x00],
        _ => [0x1F,0x11,0x15,0x11,0x15,0x11,0x1F],
    }
}
