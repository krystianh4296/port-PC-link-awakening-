use minifb::{Key, Window};

/// Kolejność bitów zgodna z rejestrem joypada Game Boya.
#[repr(u8)]
#[derive(Clone, Copy)]
pub enum GameButton {
    Right = 0, Left = 1, Up = 2, Down = 3,
    A = 4, B = 5, Select = 6, Start = 7,
}

pub struct Input {
    escape_pressed: bool,
    pressed: u8,
}

impl Input {
    pub fn new() -> Self {
        Self {
            escape_pressed: false,
            pressed: 0,
        }
    }

    pub fn update(&mut self, window: &Window) {
        self.escape_pressed = window.is_key_down(Key::Escape);
        self.pressed = 0;
        self.set(GameButton::Right, window.is_key_down(Key::Right) || window.is_key_down(Key::D));
        self.set(GameButton::Left, window.is_key_down(Key::Left) || window.is_key_down(Key::A));
        self.set(GameButton::Up, window.is_key_down(Key::Up) || window.is_key_down(Key::W));
        self.set(GameButton::Down, window.is_key_down(Key::Down) || window.is_key_down(Key::S));
        self.set(GameButton::A, window.is_key_down(Key::Z) || window.is_key_down(Key::K));
        self.set(GameButton::B, window.is_key_down(Key::X) || window.is_key_down(Key::L));
        self.set(GameButton::Select, window.is_key_down(Key::RightShift) || window.is_key_down(Key::Tab));
        self.set(GameButton::Start, window.is_key_down(Key::Enter) || window.is_key_down(Key::Space));
    }

    pub fn escape_pressed(&self) -> bool {
        self.escape_pressed
    }

    pub fn is_pressed(&self, button: GameButton) -> bool {
        self.pressed & (1 << button as u8) != 0
    }

    fn set(&mut self, button: GameButton, pressed: bool) {
        if pressed { self.pressed |= 1 << button as u8; }
    }
}
