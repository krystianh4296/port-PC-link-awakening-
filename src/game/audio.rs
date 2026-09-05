use std::collections::VecDeque;
use std::num::{NonZeroU16, NonZeroU32};
use std::sync::{Arc, Mutex};

use rodio::{DeviceSinkBuilder, MixerDeviceSink, Player, Source};

/// Natywne wyjście stereo APU. Kolejka ogranicza opóźnienie do pół sekundy.
pub struct Audio {
    buffer: Arc<Mutex<VecDeque<f32>>>,
    _stream: MixerDeviceSink,
    _player: Player,
}

impl Audio {
    pub fn new() -> Self {
        let stream = DeviceSinkBuilder::open_default_sink()
            .expect("Nie udało się otworzyć urządzenia audio");
        let player = Player::connect_new(stream.mixer());
        let buffer = Arc::new(Mutex::new(VecDeque::with_capacity(48_000)));
        player.append(AudioSource { buffer: buffer.clone() });
        Self { buffer, _stream: stream, _player: player }
    }

    pub fn push_stereo(&self, left: f32, right: f32) {
        let mut buffer = self.buffer.lock().expect("Kolejka audio została zablokowana");
        if buffer.len() + 2 <= 48_000 {
            buffer.push_back(left);
            buffer.push_back(right);
        }
    }
}

struct AudioSource { buffer: Arc<Mutex<VecDeque<f32>>> }

impl Iterator for AudioSource {
    type Item = f32;
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.buffer.lock().ok()?.pop_front().unwrap_or(0.0))
    }
}

impl Source for AudioSource {
    fn current_span_len(&self) -> Option<usize> { None }
    fn channels(&self) -> NonZeroU16 { NonZeroU16::new(2).unwrap() }
    fn sample_rate(&self) -> NonZeroU32 { NonZeroU32::new(48_000).unwrap() }
    fn total_duration(&self) -> Option<std::time::Duration> { None }
}
