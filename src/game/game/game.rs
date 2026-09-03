use crate::input::Input;

pub struct Game {
    running: bool,
}

impl Game {
    pub fn new() -> Self {
        Self {
            running: true,
        }
    }

    pub fn update(&mut self, _input: &Input, _delta_time: f32) {
    }

    pub fn is_running(&self) -> bool {
        self.running
    }
}