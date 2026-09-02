use std::collections::VecDeque;
use crate::audio::Audio;

const SAMPLE_RATE: u32 = 48_000;
const CPU_CLOCK: u32 = 4_194_304;

pub struct Apu {
    pub enabled: bool,

    pub(crate) frame_sequencer_cycles: u32,
    pub(crate) frame_sequencer_step: u8,
    pub(crate) sample_cycles: u32,

    pub(crate) ch1: SquareChannel,
    pub(crate) ch2: SquareChannel,
    pub(crate) ch3: WaveChannel,
    pub(crate) ch4: NoiseChannel,

    pub(crate) nr50: u8,
    pub(crate) nr51: u8,
    pub(crate) nr52: u8,

    pub(crate) audio: Option<crate::audio::Audio>,
    pub(crate) sample_counter: u64,
}

impl Apu {
    pub fn new() -> Self {
        Self {
            enabled: false,

            frame_sequencer_cycles: 0,
            frame_sequencer_step: 0,
            sample_cycles: 0,

            ch1: SquareChannel::new(true),
            ch2: SquareChannel::new(false),
            ch3: WaveChannel::new(),
            ch4: NoiseChannel::new(),

            nr50: 0,
            nr51: 0,
            nr52: 0,

            audio: None,
            sample_counter: 0,
        }
    }
    
    pub fn set_audio(&mut self, audio: Audio) {
    self.audio = Some(audio);
}

    pub fn step(&mut self, cycles: u32) {
        if !self.enabled {
            return;
        }

        // Game Boy CPU: 4,194,304 Hz
        //
        // Frame sequencer: 512 Hz
        // 4,194,304 / 512 = 8192
        self.frame_sequencer_cycles += cycles;

        while self.frame_sequencer_cycles >= 8192 {
            self.frame_sequencer_cycles -= 8192;

            self.tick_frame_sequencer();
        }

        // Docelowo chcemy około 48 kHz.
        self.sample_cycles += cycles;



        self.sample_cycles += cycles * SAMPLE_RATE;

        while self.sample_cycles >= CPU_CLOCK {
            self.sample_cycles -= CPU_CLOCK;
            self.generate_sample();
        }

        // CH1 frequency timer
        self.ch1.step(cycles);
        self.ch2.step(cycles);
        self.ch3.step(cycles);
        self.ch4.step(cycles);
    }

    fn tick_frame_sequencer(&mut self) {
        match self.frame_sequencer_step {
            // 0, 2, 4, 6 = length counter
            0 | 2 | 4 | 6 => {
                self.ch1.clock_length();
                self.ch2.clock_length();
                self.ch3.clock_length();
                self.ch4.clock_length();
            }

            // 7 = envelope
            7 => {
                self.ch1.clock_envelope();
                self.ch2.clock_envelope();
                self.ch4.clock_envelope();
            }

            _ => {}
        }

        // Sweep CH1:
        //
        // 2, 6 = sweep
        if self.frame_sequencer_step == 2
            || self.frame_sequencer_step == 6
        {
            self.ch1.clock_sweep();
        }

        self.frame_sequencer_step =
            (self.frame_sequencer_step + 1) & 7;
    }

    fn generate_sample(&mut self) {
        self.sample_counter += 1;
        let channels = [
            self.ch1.output(),
            self.ch2.output(),
            self.ch3.output(),
            self.ch4.output(),
        ];

        let mut left = 0.0f32;
        let mut right = 0.0f32;

        for i in 0..4 {
            if self.nr51 & (1 << i) != 0 {
                right += channels[i];
            }

            if self.nr51 & (1 << (i + 4)) != 0 {
                left += channels[i];
            }
        }

        let right_volume =
            (self.nr50 & 0x07) as f32 / 7.0;

        let left_volume =
            ((self.nr50 >> 4) & 0x07) as f32 / 7.0;

        left *= left_volume;
        right *= right_volume;

        // Ograniczenie sumowania czterech kanałów.
        left *= 0.25;
        right *= 0.25;

        left = left.clamp(-1.0, 1.0);
        right = right.clamp(-1.0, 1.0);

        if let Some(audio) = &self.audio {
            audio.push_stereo(left, right);
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        match address {
            // =========================
            // CH1
            // =========================

            0xFF10 => self.ch1.nr0 | 0x80,
            0xFF11 => self.ch1.nr1,
            0xFF12 => self.ch1.nr2,
            0xFF13 => self.ch1.nr3,

            0xFF14 => {
                (self.ch1.nr4 & 0xC7) | 0xBF
            }

            // =========================
            // CH2
            // =========================

            0xFF16 => self.ch2.nr1,
            0xFF17 => self.ch2.nr2,
            0xFF18 => self.ch2.nr3,
            0xFF19 => (self.ch2.nr4 & 0xC7) | 0xBF,

            // =========================
            // CH3
            // =========================

            0xFF1A => {
                if self.ch3.enabled {
                    self.ch3.nr30 | 0x80
                } else {
                    self.ch3.nr30 & 0x7F
                }
            }

            0xFF1B => self.ch3.nr31,
            0xFF1C => self.ch3.nr32,
            0xFF1D => self.ch3.nr33,
            0xFF1E => (self.ch3.nr34 & 0xC7) | 0xBF,

            // =========================
            // CH4
            // =========================

            0xFF20 => self.ch4.nr41,
            0xFF21 => self.ch4.nr42,
            0xFF22 => self.ch4.nr43,
            0xFF23 => (self.ch4.nr44 & 0xC7) | 0xBF,

            // =========================
            // Mixer
            // =========================

            0xFF24 => self.nr50,
            0xFF25 => self.nr51,

            // =========================
            // NR52
            // =========================

            0xFF26 => {
                let mut value = if self.enabled {
                    0x80
                } else {
                    0x00
                };

                if self.ch1.enabled {
                    value |= 0x01;
                }

                if self.ch2.enabled {
                    value |= 0x02;
                }

                if self.ch3.enabled {
                    value |= 0x04;
                }

                if self.ch4.enabled {
                    value |= 0x08;
                }

                value | 0x70
            }

            // Wave RAM
            0xFF30..=0xFF3F => {
                self.ch3.wave_ram[(address - 0xFF30) as usize]
            }

            _ => 0xFF,
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        match address {
            // =========================
            // CH1
            // =========================

            0xFF10 => {
                self.ch1.nr0 = value;
            }

            0xFF11 => {
                self.ch1.nr1 = value;

                self.ch1.length_counter =
                    64 - (value & 0x3F);
            }

            0xFF12 => {
                self.ch1.nr2 = value;

                if value & 0xF8 == 0 {
                    self.ch1.enabled = false;
                }
            }

            0xFF13 => {
                self.ch1.nr3 = value;
            }

            0xFF14 => {
                self.ch1.nr4 = value;

                self.ch1.length_enabled =
                    value & 0x40 != 0;

                if value & 0x80 != 0 {
                    self.ch1.trigger();
                }
            }

            // =========================
            // CH2
            // =========================

            0xFF16 => {
                self.ch2.nr1 = value;

                self.ch2.length_counter =
                    64 - (value & 0x3F);
            }

            0xFF17 => {
                self.ch2.nr2 = value;

                if value & 0xF8 == 0 {
                    self.ch2.enabled = false;
                }
            }

            0xFF18 => {
                self.ch2.nr3 = value;
            }

            0xFF19 => {
                self.ch2.nr4 = value;

                self.ch2.length_enabled =
                    value & 0x40 != 0;

                if value & 0x80 != 0 {
                    self.ch2.trigger();
                }
            }

            // =========================
            // CH3
            // =========================

            0xFF1A => {
                self.ch3.nr30 = value;

                if value & 0x80 == 0 {
                    self.ch3.enabled = false;
                }
            }

            0xFF1B => {
                self.ch3.nr31 = value;

                self.ch3.length_counter =
                    256 - value as u16;
            }

            0xFF1C => {
                self.ch3.nr32 = value;
            }

            0xFF1D => {
                self.ch3.nr33 = value;
            }

            0xFF1E => {
                self.ch3.nr34 = value;

                self.ch3.length_enabled =
                    value & 0x40 != 0;

                if value & 0x80 != 0 {
                    self.ch3.trigger();
                }
            }

            // =========================
            // CH4
            // =========================

            0xFF20 => {
                self.ch4.nr41 = value;

                self.ch4.length_counter =
                    64 - (value & 0x3F);
            }

            0xFF21 => {
                self.ch4.nr42 = value;

                if value & 0xF8 == 0 {
                    self.ch4.enabled = false;
                }
            }

            0xFF22 => {
                self.ch4.nr43 = value;
            }

            0xFF23 => {
                self.ch4.nr44 = value;

                self.ch4.length_enabled =
                    value & 0x40 != 0;

                if value & 0x80 != 0 {
                    self.ch4.trigger();
                }
            }

            // =========================
            // Mixer
            // =========================

            0xFF24 => {
                self.nr50 = value;
            }

            0xFF25 => {
                self.nr51 = value;
            }

            // =========================
            // NR52
            // =========================

            0xFF26 => {
                self.nr52 = value;

                self.enabled =
                    value & 0x80 != 0;

                if !self.enabled {
                    self.ch1.enabled = false;
                    self.ch2.enabled = false;
                    self.ch3.enabled = false;
                    self.ch4.enabled = false;
                }
            }

            // Wave RAM
            0xFF30..=0xFF3F => {
                self.ch3.wave_ram[(address - 0xFF30) as usize] = value;
            }

            _ => {}
        }
    }
    
}


// ============================================================
// CHANNEL 1
// ============================================================

pub(crate) struct SquareChannel {
    pub(crate) enabled: bool,

    pub(crate) nr0: u8,
    pub(crate) nr1: u8,
    pub(crate) nr2: u8,
    pub(crate) nr3: u8,
    pub(crate) nr4: u8,

    pub(crate) length_counter: u8,
    pub(crate) length_enabled: bool,

    pub(crate) envelope_volume: u8,
    pub(crate) envelope_timer: u8,

    pub(crate) frequency_timer: u16,
    pub(crate) duty_position: u8,

    pub(crate) sweep_timer: u8,
    pub(crate) sweep_shadow_frequency: u16,
    pub(crate) sweep_enabled: bool,

    pub(crate) has_sweep: bool,
}

impl SquareChannel {
    fn new(has_sweep: bool) -> Self {
        Self {
            enabled: false,

            nr0: 0,
            nr1: 0,
            nr2: 0,
            nr3: 0,
            nr4: 0,

            length_counter: 0,
            length_enabled: false,

            envelope_volume: 0,
            envelope_timer: 0,

            frequency_timer: 0,
            duty_position: 0,

            sweep_timer: 0,
            sweep_shadow_frequency: 0,
            sweep_enabled: false,

            has_sweep,
        }
    }

    fn frequency(&self) -> u16 {
        ((self.nr4 as u16 & 0x07) << 8)
            | self.nr3 as u16
    }

    fn trigger(&mut self) {
    self.enabled = true;

    if self.length_counter == 0 {
        self.length_counter = 64;
    }

    self.envelope_volume =
        (self.nr2 >> 4) & 0x0F;

    self.envelope_timer =
        self.nr2 & 0x07;

    if self.envelope_timer == 0 {
        self.envelope_timer = 8;
    }

    self.frequency_timer =
        Self::frequency_period(self.frequency());

    self.duty_position = 0;

    if self.has_sweep {
        self.sweep_shadow_frequency =
            self.frequency();

        self.sweep_timer =
            (self.nr0 >> 4) & 0x07;

        if self.sweep_timer == 0 {
            self.sweep_timer = 8;
        }

        self.sweep_enabled =
            (self.nr0 & 0x77) != 0;
    }

    if self.nr2 & 0xF8 == 0 {
        self.enabled = false;
    }
}

    fn frequency_period(frequency: u16) -> u16 {
        let period = 2048u16.saturating_sub(frequency);

        period.saturating_mul(4)
    }

    fn step(&mut self, cycles: u32) {
        if !self.enabled {
            return;
        }

        let mut cycles = cycles;

        while cycles > 0 {
            if self.frequency_timer > cycles as u16 {
                self.frequency_timer -= cycles as u16;
                break;
            }

            let consumed = self.frequency_timer;

            cycles = cycles.saturating_sub(
                consumed as u32
            );

            self.frequency_timer =
                Self::frequency_period(
                    self.frequency()
                );

            self.duty_position =
                (self.duty_position + 1) & 7;
        }
    }

    fn clock_length(&mut self) {
        if !self.length_enabled {
            return;
        }

        if self.length_counter > 0 {
            self.length_counter -= 1;

            if self.length_counter == 0 {
                self.enabled = false;
            }
        }
    }

    fn clock_envelope(&mut self) {
        let period = self.nr2 & 0x07;

        if period == 0 {
            return;
        }

        if self.envelope_timer > 0 {
            self.envelope_timer -= 1;
        }

        if self.envelope_timer == 0 {
            self.envelope_timer = period;

            if self.nr2 & 0x08 != 0 {
                if self.envelope_volume < 15 {
                    self.envelope_volume += 1;
                }
            } else if self.envelope_volume > 0 {
                self.envelope_volume -= 1;
            }
        }
    }

    fn clock_sweep(&mut self) {
        if !self.has_sweep {
        return;
    }
        if !self.sweep_enabled {
            return;
        }

        if self.sweep_timer > 0 {
            self.sweep_timer -= 1;
        }

        if self.sweep_timer == 0 {
            let period =
                (self.nr0 >> 4) & 0x07;

            self.sweep_timer =
                if period == 0 { 8 } else { period };

            if period == 0 {
                return;
            }

            let shift = self.nr0 & 0x07;

            let delta =
                self.sweep_shadow_frequency >> shift;

            let new_frequency =
                if self.nr0 & 0x08 != 0 {
                    self.sweep_shadow_frequency
                        .saturating_sub(delta)
                } else {
                    self.sweep_shadow_frequency
                        .saturating_add(delta)
                };

            if new_frequency > 2047 {
                self.enabled = false;
                return;
            }

            self.sweep_shadow_frequency =
                new_frequency;

            self.nr3 =
                new_frequency as u8;

            self.nr4 =
                (self.nr4 & 0xF8)
                    | ((new_frequency >> 8) as u8 & 0x07);
        }
    }

    fn output(&self) -> f32 {
        if !self.enabled {
            return 0.0;
        }

        let duty = match (self.nr1 >> 6) & 0x03 {
            0 => 0b00000001,
            1 => 0b10000001,
            2 => 0b10000111,
            _ => 0b01111110,
        };

        let bit =
            (duty >> self.duty_position) & 1;

        if bit == 0 {
            -(self.envelope_volume as f32) / 15.0
        } else {
            self.envelope_volume as f32 / 15.0
        }
    }
}


// ============================================================
// CHANNEL 3
// ============================================================

pub(crate) struct WaveChannel {
    pub(crate) enabled: bool,

    pub(crate) nr30: u8,
    pub(crate) nr31: u8,
    pub(crate) nr32: u8,
    pub(crate) nr33: u8,
    pub(crate) nr34: u8,

    pub(crate) length_counter: u16,
    pub(crate) length_enabled: bool,

    pub(crate) frequency_timer: u16,
    pub(crate) position: u8,

    pub(crate) wave_ram: [u8; 16],
}

impl WaveChannel {
    
    fn new() -> Self {
        Self {
            enabled: false,

            nr30: 0,
            nr31: 0,
            nr32: 0,
            nr33: 0,
            nr34: 0,

            length_counter: 0,
            length_enabled: false,

            frequency_timer: 0,
            position: 0,

            wave_ram: [0; 16],
        }
    }
    
    fn frequency(&self) -> u16 {
        ((self.nr34 as u16 & 0x07) << 8)
            | self.nr33 as u16
    }

    fn frequency_period(frequency: u16) -> u16 {
        let period = 2048u16.saturating_sub(frequency);

        period.saturating_mul(2)
    }
    
    fn trigger(&mut self) {
        if self.nr30 & 0x80 == 0 {
            self.enabled = false;
            return;
        }

        self.enabled = true;

        if self.length_counter == 0 {
            self.length_counter = 256;
        }

        self.frequency_timer =
            Self::frequency_period(self.frequency());

        self.position = 0;
    }
    
    fn step(&mut self, cycles: u32) {
        if !self.enabled {
            return;
        }

        let mut cycles = cycles;

        while cycles > 0 {
            if self.frequency_timer > cycles as u16 {
                self.frequency_timer -= cycles as u16;
                break;
            }

            let consumed = self.frequency_timer;

            cycles = cycles.saturating_sub(
                consumed as u32
            );

            self.frequency_timer =
                Self::frequency_period(
                    self.frequency()
                );

            self.position =
                (self.position + 1) & 31;
        }
    }
    
    fn clock_length(&mut self) {
        if !self.length_enabled {
            return;
        }

        if self.length_counter > 0 {
            self.length_counter -= 1;

            if self.length_counter == 0 {
                self.enabled = false;
            }
        }
    }
    fn output(&self) -> f32 {
        if !self.enabled {
            return 0.0;
        }

        let byte = self.wave_ram[(self.position / 2) as usize];

        let sample = if self.position & 1 == 0 {
            byte >> 4
        } else {
            byte & 0x0F
        };

        let volume_code =
            (self.nr32 >> 5) & 0x03;

        let sample = match volume_code {
            0 => 0,
            1 => sample,
            2 => sample >> 1,
            3 => sample >> 2,
            _ => 0,
        };

        (sample as f32 / 15.0) * 2.0 - 1.0
    }

    
}



// ============================================================
// CHANNEL 4
// ============================================================

pub(crate) struct NoiseChannel {
    pub(crate) enabled: bool,

    pub(crate) nr41: u8,
    pub(crate) nr42: u8,
    pub(crate) nr43: u8,
    pub(crate) nr44: u8,

    pub(crate) length_counter: u8,
    pub(crate) length_enabled: bool,

    pub(crate) envelope_volume: u8,
    pub(crate) envelope_timer: u8,

    pub(crate) frequency_timer: u16,

    pub(crate) lfsr: u16,
}

impl NoiseChannel {
    fn new() -> Self {
        Self {
            enabled: false,

            nr41: 0,
            nr42: 0,
            nr43: 0,
            nr44: 0,

            length_counter: 0,
            length_enabled: false,

            envelope_volume: 0,
            envelope_timer: 0,

            frequency_timer: 0,

            lfsr: 0x7FFF,
        }
    }

    fn trigger(&mut self) {
        self.enabled = true;

        if self.length_counter == 0 {
            self.length_counter = 64;
        }

        self.envelope_volume =
            (self.nr42 >> 4) & 0x0F;

        self.envelope_timer =
            self.nr42 & 0x07;

        if self.envelope_timer == 0 {
            self.envelope_timer = 8;
        }

        self.lfsr = 0x7FFF;

        self.frequency_timer =
            self.frequency_period();
        
        if self.nr42 & 0xF8 == 0 {
            self.enabled = false;
        }
    }

    fn frequency_period(&self) -> u16 {
        let divisor = match self.nr43 & 0x07 {
            0 => 8,
            1 => 16,
            2 => 32,
            3 => 48,
            4 => 64,
            5 => 80,
            6 => 96,
            _ => 112,
        };

        let shift = (self.nr43 >> 4) & 0x0F;

        let period =
            (divisor as u32) << shift;

        period.min(0xFFFF) as u16
    }

    fn step(&mut self, cycles: u32) {
        if !self.enabled {
            return;
        }

        let mut cycles = cycles;

        while cycles > 0 {
            if self.frequency_timer > cycles as u16 {
                self.frequency_timer -= cycles as u16;
                break;
            }

            let consumed = self.frequency_timer;

            cycles =
                cycles.saturating_sub(consumed as u32);

            self.frequency_timer =
                self.frequency_period();

            self.clock_lfsr();
        }
    }

    fn clock_lfsr(&mut self) {
        let xor =
            (self.lfsr & 1)
            ^ ((self.lfsr >> 1) & 1);

        self.lfsr >>= 1;

        self.lfsr |= xor << 14;

        // NR43 bit 3:
        // 0 = 15-bit LFSR
        // 1 = 7-bit LFSR
        if self.nr43 & 0x08 != 0 {
            let bit = xor;

            self.lfsr &= !(1 << 6);
            self.lfsr |= bit << 6;
        }
    }

    fn clock_length(&mut self) {
        if !self.length_enabled {
            return;
        }

        if self.length_counter > 0 {
            self.length_counter -= 1;

            if self.length_counter == 0 {
                self.enabled = false;
            }
        }
    }

    fn clock_envelope(&mut self) {
        let period = self.nr42 & 0x07;

        if period == 0 {
            return;
        }

        if self.envelope_timer > 0 {
            self.envelope_timer -= 1;
        }

        if self.envelope_timer == 0 {
            self.envelope_timer = period;

            if self.nr42 & 0x08 != 0 {
                if self.envelope_volume < 15 {
                    self.envelope_volume += 1;
                }
            } else if self.envelope_volume > 0 {
                self.envelope_volume -= 1;
            }
        }
    }

    fn output(&self) -> f32 {
        if !self.enabled {
            return 0.0;
        }

        // Bit 0 LFSR jest wyjściem kanału.
        let bit = self.lfsr & 1;

        let volume =
            self.envelope_volume as f32 / 15.0;

        if bit == 0 {
            volume
        } else {
            -volume
        }
    }
}
