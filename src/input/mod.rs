use minifb::{Key, Window};

pub struct Input {
    escape_pressed: bool,
}

impl Input {
    pub fn new() -> Self {
        Self {
            escape_pressed: false,
        }
    }

    pub fn update(&mut self, window: &Window) {
        self.escape_pressed = window.is_key_down(Key::Escape);
    }

    pub fn escape_pressed(&self) -> bool {
        self.escape_pressed
    }
}