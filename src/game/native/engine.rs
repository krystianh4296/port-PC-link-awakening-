use minifb::{Key, Window};

use crate::rendering::renderer::{HEIGHT, WIDTH};
use crate::rom::Rom;
use crate::native::{capture_boot_scene, BootSceneError};

/// Stan natywnej gry. Nie zawiera CPU, pamięci ani rejestrów Game Boya.
pub struct NativeGame {
    running: bool,
    reference_title: String,
    /// Nieruchoma warstwa sceny zidentyfikowana przez porównanie klatek ROM-u.
    background: Vec<u32>,
    /// Kolejne natywne warstwy różnicowe; `None` oznacza piksel tła.
    animation_layers: Vec<Vec<(usize, u32)>>,
    animation_time: f32,
}

impl NativeGame {
    /// Tworzy natywną sesję po walidacji ROM-u referencyjnego.
    pub fn from_rom(rom: &Rom) -> Result<Self, BootSceneError> {
        let scene = capture_boot_scene(rom.clone())?;
        let (background, animation_layers) = split_scene_layers(scene.frames());
        Ok(Self {
            running: true,
            reference_title: rom.header().title.clone(),
            background,
            animation_layers,
            animation_time: 0.0,
        })
    }

    pub fn is_running(&self) -> bool { self.running }
    pub fn reference_title(&self) -> &str { &self.reference_title }

    pub fn update(&mut self, window: &Window, delta_seconds: f32) {
        if window.is_key_down(Key::Escape) {
            self.running = false;
            return;
        }

        // Natywne odtwarzanie zarejestrowanych klatek: emulator nie działa
        // już po zakończeniu ekstrakcji sceny.
        self.animation_time += delta_seconds.min(0.05);
    }

    pub fn render(&self, framebuffer: &mut [u32; WIDTH * HEIGHT]) {
        let frame = ((self.animation_time * 60.0) as usize) % self.animation_layers.len();
        framebuffer.copy_from_slice(&self.background);
        for &(index, color) in &self.animation_layers[frame] {
            framebuffer[index] = color;
        }
    }
}

/// Dzieli scenę na stałą mapę i zmienne piksele. Ta reprezentacja jest już
/// niezależna od framebufferu emulatora: Rust składa warstwy w czasie gry.
fn split_scene_layers(
    frames: &[Box<[u32; WIDTH * HEIGHT]>],
) -> (Vec<u32>, Vec<Vec<(usize, u32)>>) {
    assert!(!frames.is_empty(), "Scena ROM nie zawiera klatek");
    let background = (0..WIDTH * HEIGHT).map(|index| {
        let color = frames[0][index];
        if frames.iter().all(|frame| frame[index] == color) { color } else { 0xFF000000 }
    }).collect();
    let layers = frames.iter().map(|frame| (0..WIDTH * HEIGHT)
        .filter_map(|index| {
            let color = frame[index];
            (!frames.iter().all(|other| other[index] == color)).then_some((index, color))
        })
        .collect())
        .collect();
    (background, layers)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn layer_split_preserves_each_original_frame() {
        let mut first = [0xFF000000; WIDTH * HEIGHT];
        let mut second = first;
        second[7] = 0xFFFFFFFF;
        first[9] = 0xFF123456;
        second[9] = 0xFF123456;
        let (background, layers) = split_scene_layers(&[Box::new(first), Box::new(second)]);
        assert_eq!(background[9], 0xFF123456);
        assert!(layers[0].contains(&(7, 0xFF000000)));
        assert!(layers[1].contains(&(7, 0xFFFFFFFF)));
    }
}
