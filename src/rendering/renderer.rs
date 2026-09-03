use minifb::{Window, WindowOptions};

pub const WIDTH: usize = 160;
pub const HEIGHT: usize = 144;

pub struct Renderer {
    window: Window,
    buffer: Vec<u32>,
}

impl Renderer {
    pub fn new() -> Self {
        let window = Window::new(
            "Zelda - Native Rust",
            WIDTH,
            HEIGHT,
            WindowOptions {
                resize: false,
                scale: minifb::Scale::X4,
                ..WindowOptions::default()
            },
        )
        .expect("Nie można utworzyć okna gry");

        Self {
            window,
            buffer: vec![0; WIDTH * HEIGHT],
        }
    }

    pub fn draw(&mut self) {
        self.window
            .update_with_buffer(&self.buffer, WIDTH, HEIGHT)
            .expect("Nie można zaktualizować obrazu");
    }

    pub fn is_open(&self) -> bool {
        self.window.is_open()
    }
    pub fn window(&self) -> &Window {
        &self.window
    }
}