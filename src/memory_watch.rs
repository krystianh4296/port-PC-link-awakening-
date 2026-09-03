use crate::bus::Bus;
use minifb::{Key, Window, WindowOptions};
use std::time::{Duration, Instant};

const WIDTH: usize = 1180;
const HEIGHT: usize = 820;

const CHAR_WIDTH: usize = 8;
const CHAR_HEIGHT: usize = 12;

const ROW_HEIGHT: usize = 18;
const HEADER_HEIGHT: usize = 82;

const BYTES_PER_ROW: usize = 16;
const VISIBLE_ROWS: usize = (HEIGHT - HEADER_HEIGHT) / ROW_HEIGHT;

const REFRESH_INTERVAL: Duration = Duration::from_millis(100);

const BG: u32 = 0x101010;
const TEXT: u32 = 0xD0D0D0;
const HEADER: u32 = 0xFFFFFF;
const ADDRESS: u32 = 0x70B7FF;
const CHANGED: u32 = 0xFFFF55;
const ASCII: u32 = 0xAAAAAA;
const SELECTED: u32 = 0xFFFFFF;

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

            buffer: vec![BG; WIDTH * HEIGHT],
        }
    }

    pub fn update(&mut self, bus: &Bus) {
        if !self.window.is_open() {
            return;
        }

        self.handle_input();

        if self.last_refresh.elapsed() >= REFRESH_INTERVAL {
            self.refresh(bus);
            self.last_refresh = Instant::now();
        }

        self.render();

        self.window
            .update_with_buffer(&self.buffer, WIDTH, HEIGHT)
            .expect("Błąd aktualizacji okna Memory Watch");
    }

    fn refresh(&mut self, bus: &Bus) {
        let size = self
            .end
            .wrapping_sub(self.start)
            .wrapping_add(1) as usize;

        let mut new_memory = Vec::with_capacity(size);

        for offset in 0..size {
            let address = self.start.wrapping_add(offset as u16);
            new_memory.push(bus.read_debug(address));
        }

        self.previous_memory = std::mem::replace(
            &mut self.current_memory,
            new_memory,
        );
    }

    fn handle_input(&mut self) {
        if self.window.is_key_pressed(Key::Escape, minifb::KeyRepeat::No) {
            self.window.set_position(-10000, -10000);
            return;
        }

        if self
            .window
            .is_key_pressed(Key::Tab, minifb::KeyRepeat::No)
        {
            self.selected_field = match self.selected_field {
                InputField::Start => InputField::End,
                InputField::End => InputField::Start,
            };

            self.cursor = 0;
        }

        if self
            .window
            .is_key_pressed(Key::Left, minifb::KeyRepeat::Yes)
        {
            if self.cursor > 0 {
                self.cursor -= 1;
            }
        }

        if self
            .window
            .is_key_pressed(Key::Right, minifb::KeyRepeat::Yes)
        {
            if self.cursor < 3 {
                self.cursor += 1;
            }
        }

        if self
            .window
            .is_key_pressed(Key::Up, minifb::KeyRepeat::Yes)
        {
            if self.scroll_row > 0 {
                self.scroll_row -= 1;
            }
        }

        if self
            .window
            .is_key_pressed(Key::Down, minifb::KeyRepeat::Yes)
        {
            self.scroll_row += 1;
            self.clamp_scroll();
        }

        if self
            .window
            .is_key_pressed(Key::PageUp, minifb::KeyRepeat::No)
        {
            self.scroll_row = self.scroll_row.saturating_sub(VISIBLE_ROWS);
        }

        if self
            .window
            .is_key_pressed(Key::PageDown, minifb::KeyRepeat::No)
        {
            self.scroll_row += VISIBLE_ROWS;
            self.clamp_scroll();
        }

        self.handle_hex_keys();
    }

    fn handle_hex_keys(&mut self) {
        let digit = if self.window.is_key_pressed(Key::Key0, minifb::KeyRepeat::No) {
            Some(0)
        } else if self.window.is_key_pressed(Key::Key1, minifb::KeyRepeat::No) {
            Some(1)
        } else if self.window.is_key_pressed(Key::Key2, minifb::KeyRepeat::No) {
            Some(2)
        } else if self.window.is_key_pressed(Key::Key3, minifb::KeyRepeat::No) {
            Some(3)
        } else if self.window.is_key_pressed(Key::Key4, minifb::KeyRepeat::No) {
            Some(4)
        } else if self.window.is_key_pressed(Key::Key5, minifb::KeyRepeat::No) {
            Some(5)
        } else if self.window.is_key_pressed(Key::Key6, minifb::KeyRepeat::No) {
            Some(6)
        } else if self.window.is_key_pressed(Key::Key7, minifb::KeyRepeat::No) {
            Some(7)
        } else if self.window.is_key_pressed(Key::Key8, minifb::KeyRepeat::No) {
            Some(8)
        } else if self.window.is_key_pressed(Key::Key9, minifb::KeyRepeat::No) {
            Some(9)
        } else if self.window.is_key_pressed(Key::A, minifb::KeyRepeat::No) {
            Some(10)
        } else if self.window.is_key_pressed(Key::B, minifb::KeyRepeat::No) {
            Some(11)
        } else if self.window.is_key_pressed(Key::C, minifb::KeyRepeat::No) {
            Some(12)
        } else if self.window.is_key_pressed(Key::D, minifb::KeyRepeat::No) {
            Some(13)
        } else if self.window.is_key_pressed(Key::E, minifb::KeyRepeat::No) {
            Some(14)
        } else if self.window.is_key_pressed(Key::F, minifb::KeyRepeat::No) {
            Some(15)
        } else {
            None
        };

        let Some(value) = digit else {
            return;
        };

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

        if self.start > self.end {
            std::mem::swap(&mut self.start, &mut self.end);

            self.start_digits = hex_digits(self.start);
            self.end_digits = hex_digits(self.end);
        }

        self.scroll_row = 0;

        if self.cursor < 3 {
            self.cursor += 1;
        }
    }

    fn clamp_scroll(&mut self) {
        let total_rows = self.current_memory.len().div_ceil(BYTES_PER_ROW);

        if total_rows > VISIBLE_ROWS {
            let max_scroll = total_rows - VISIBLE_ROWS;

            if self.scroll_row > max_scroll {
                self.scroll_row = max_scroll;
            }
        } else {
            self.scroll_row = 0;
        }
    }

    fn render(&mut self) {
        self.buffer.fill(BG);

        self.draw_text(
            20,
            15,
            "MEMORY WATCH",
            HEADER,
        );

        self.draw_text(
            20,
            42,
            "TAB = pole | LEFT/RIGHT = pozycja | HEX = wpisywanie | UP/DOWN = scroll | PGUP/PGDN",
            TEXT,
        );

        let start_text = format!(
            "START: {:04X}",
            self.start
        );

        let end_text = format!(
            "END:   {:04X}",
            self.end
        );

        self.draw_text(
            20,
            65,
            &start_text,
            if self.selected_field == InputField::Start {
                SELECTED
            } else {
                TEXT
            },
        );

        self.draw_text(
            180,
            65,
            &end_text,
            if self.selected_field == InputField::End {
                SELECTED
            } else {
                TEXT
            },
        );

        self.draw_text(
            360,
            65,
            &format!(
                "RANGE: {} bytes | UPDATE: 100 ms",
                self.current_memory.len()
            ),
            TEXT,
        );

        self.draw_text(
            20,
            HEADER_HEIGHT - 8,
            "ADDRESS",
            ADDRESS,
        );

        for i in 0..BYTES_PER_ROW {
            let x = 100 + i * 38;

            self.draw_text(
                x,
                HEADER_HEIGHT - 8,
                &format!("{:02X}", i),
                TEXT,
            );
        }

        self.draw_text(
            750,
            HEADER_HEIGHT - 8,
            "ASCII",
            ASCII,
        );

        for row in 0..VISIBLE_ROWS {
            let memory_index =
                (self.scroll_row + row) * BYTES_PER_ROW;

            if memory_index >= self.current_memory.len() {
                break;
            }

            let y = HEADER_HEIGHT + row * ROW_HEIGHT;

            let address = self.start.wrapping_add(
                memory_index as u16
            );

            self.draw_text(
                20,
                y,
                &format!("{:04X}:", address),
                ADDRESS,
            );

            let mut ascii = String::new();

            for column in 0..BYTES_PER_ROW {
                let index = memory_index + column;

                if index >= self.current_memory.len() {
                    break;
                }

                let value = self.current_memory[index];

                let changed =
                    index >= self.previous_memory.len()
                        || self.previous_memory[index] != value;

                let x = 100 + column * 38;

                self.draw_text(
                    x,
                    y,
                    &format!("{:02X}", value),
                    if changed {
                        CHANGED
                    } else {
                        TEXT
                    },
                );

                let c = if value.is_ascii_graphic() {
                    value as char
                } else {
                    '.'
                };

                ascii.push(c);
            }

            self.draw_text(
                750,
                y,
                &ascii,
                ASCII,
            );
        }
    }

    fn draw_text(
        &mut self,
        x: usize,
        y: usize,
        text: &str,
        color: u32,
    ) {
        let mut cursor_x = x;

        for c in text.chars() {
            self.draw_char(cursor_x, y, c, color);
            cursor_x += CHAR_WIDTH;
        }
    }

    fn draw_char(
        &mut self,
        x: usize,
        y: usize,
        c: char,
        color: u32,
    ) {
        let glyph = glyph(c);

        for (row, bits) in glyph.iter().enumerate() {
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

        ':' => [0x00,0x04,0x04,0x00,0x04,0x04,0x00],
        '-' => [0x00,0x00,0x00,0x1F,0x00,0x00,0x00],
        '>' => [0x10,0x08,0x04,0x02,0x04,0x08,0x10],
        '/' => [0x01,0x02,0x04,0x08,0x10,0x00,0x00],
        '.' => [0x00,0x00,0x00,0x00,0x00,0x0C,0x0C],
        ' ' => [0; 7],

        _ => [0; 7],
    }
}