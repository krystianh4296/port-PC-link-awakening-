//! Jednorazowe pobranie sceny startowej z ROM-u referencyjnego.
//!
//! Kod Game Boya jest wykonywany tylko podczas ekstrakcji.  Wynikiem jest
//! zwykły bufor pikseli, który natywny renderer wyświetla bez emulacji w pętli.

use crate::game::Game;
use crate::rendering::renderer::{HEIGHT, WIDTH};
use crate::rom::Rom;

const MAX_BOOT_STEPS: usize = 4_000_000;
// Pomijamy pierwszą klatkę inicjalizacji PPU. Dalsze odtwarzanie scen
// (tytułowej lub mapy) zostanie włączone po naprawie pełnej pętli emulacji.
const WARMUP_FRAMES: u16 = 90;
const CAPTURED_FRAMES: usize = 12;

#[derive(Debug)]
pub enum BootSceneError {
    DidNotReachFrame { steps: usize, cycles: u64, pc: u16, lcdc: u8, ly: u8, stat: u8 },
}

impl std::fmt::Display for BootSceneError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DidNotReachFrame { steps, cycles, pc, lcdc, ly, stat } => write!(
                f,
                "ROM nie utworzył klatki po {steps} krokach CPU / {cycles} cyklach (PC={pc:04X}, LCDC={lcdc:02X}, LY={ly:02X}, STAT={stat:02X})"
            ),
        }
    }
}

impl std::error::Error for BootSceneError {}

/// Odczytana scena referencyjna: kafelki, mapa, paleta oraz sprite'y już
/// złożone przez istniejący dekoder GBC.
pub struct BootScene {
    frames: Vec<Box<[u32; WIDTH * HEIGHT]>>,
    pub frames_to_ready: u16,
}

impl BootScene {
    pub fn frames(&self) -> &[Box<[u32; WIDTH * HEIGHT]>] { &self.frames }
}

pub fn capture_boot_scene(rom: Rom) -> Result<BootScene, BootSceneError> {
    std::thread::Builder::new()
        .name("rom-scene-extractor".into())
        // GameMemory zawiera bufory VRAM/WRAM/PPU; na Windows nie mieści się
        // w domyślnym stosie wątku głównego podczas konstrukcji.
        .stack_size(16 * 1024 * 1024)
        .spawn(move || capture_boot_scene_on_large_stack(rom))
        .expect("Nie można uruchomić ekstraktora sceny ROM")
        .join()
        .expect("Ekstraktor sceny ROM zakończył się awarią")
}

fn capture_boot_scene_on_large_stack(rom: Rom) -> Result<BootScene, BootSceneError> {
    let mut game = Game::new(rom);
    let mut frames_to_ready = 0u16;
    let mut frames = Vec::with_capacity(CAPTURED_FRAMES);
    let mut cycles = 0u64;
    for _ in 1..=MAX_BOOT_STEPS {
        cycles += game.step() as u64;
        if game.take_frame_ready() {
            frames_to_ready += 1;
            if frames_to_ready >= WARMUP_FRAMES {
                frames.push(Box::new(*game.framebuffer()));
            }
            if frames.len() == CAPTURED_FRAMES {
                return Ok(BootScene { frames, frames_to_ready });
            }
        }
    }
    Err(BootSceneError::DidNotReachFrame {
        steps: MAX_BOOT_STEPS,
        cycles,
        pc: game.cpu().pc,
        lcdc: game.read(0xFF40),
        ly: game.read(0xFF44),
        stat: game.read(0xFF41),
    })
}
