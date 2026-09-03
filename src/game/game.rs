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

    pub fn update(&mut self, input: &Input) {
        if input.escape_pressed() {
            self.running = false;
        }
    }

    pub fn is_running(&self) -> bool {
        self.running
    }
}