//! Natywny, niezależny od CPU Game Boya rdzeń gry.
//!
//! Ten moduł jest miejscem stopniowego odtwarzania zasad gry.  Nie wykonuje
//! kodu z ROM-u: ROM służy wyłącznie jako zweryfikowane źródło referencyjne.

pub mod engine;
pub mod rom_scene;

pub use engine::NativeGame;
pub use rom_scene::{capture_boot_scene, BootScene, BootSceneError};
