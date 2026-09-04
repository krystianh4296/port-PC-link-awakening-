#[derive(Debug)]
pub struct Timer {
    /// Internal 16-bit divider counter.
    ///
    /// DIV (FF04) exposes bits 15..8 of this counter.
    div_counter: u16,

    /// Timer counter (FF05).
    tima: u8,

    /// Timer modulo (FF06).
    tma: u8,

    /// Timer control (FF07).
    ///
    /// Bit 2   = timer enable
    /// Bits 1-0 = clock select
    tac: u8,

    /// Number of T-cycles remaining before TIMA is reloaded from TMA.
    ///
    /// 0 = no reload pending.
    reload_delay: u8,

    /// Set when the timer requests an interrupt.
    interrupt_pending: bool,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            div_counter: 0,
            tima: 0,
            tma: 0,
            tac: 0,
            reload_delay: 0,
            interrupt_pending: false,
        }
    }

    /// Advances the timer by the given number of Game Boy T-cycles.
    pub fn step(&mut self, cycles: u32) {
        for _ in 0..cycles {
            self.step_cycle();
        }
    }

    fn step_cycle(&mut self) {
        // Handle delayed TIMA reload.
        if self.reload_delay != 0 {
            self.reload_delay -= 1;

            if self.reload_delay == 0 {
                self.tima = self.tma;
                self.interrupt_pending = true;
            }
        }

        // The timer is driven from a bit of the free-running divider.
        //
        // A TIMA increment happens on a falling edge:
        //
        //     1 -> 0
        //
        // of the selected divider bit.
        let old_signal = self.timer_signal();

        self.div_counter = self.div_counter.wrapping_add(1);

        let new_signal = self.timer_signal();

        if old_signal && !new_signal {
            self.increment_tima();
        }
    }

    fn timer_signal(&self) -> bool {
        if self.tac & 0x04 == 0 {
            return false;
        }

        let bit = match self.tac & 0x03 {
            0x00 => 9, // 4096 Hz
            0x01 => 3, // 262144 Hz
            0x02 => 5, // 65536 Hz
            0x03 => 7, // 16384 Hz
            _ => unreachable!(),
        };

        self.div_counter & (1 << bit) != 0
    }

    fn increment_tima(&mut self) {
        let (new_value, overflow) = self.tima.overflowing_add(1);

        self.tima = new_value;

        if overflow {
            // TIMA does not immediately become TMA.
            //
            // Hardware leaves TIMA at 00 for a short period and then
            // reloads it from TMA and requests the timer interrupt.
            self.tima = 0x00;
            self.reload_delay = 4;
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        match address {
            0xFF04 => (self.div_counter >> 8) as u8,
            0xFF05 => self.tima,
            0xFF06 => self.tma,

            // Unused upper bits read as 1.
            0xFF07 => self.tac | 0xF8,

            _ => 0xFF,
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        match address {
            0xFF04 => self.write_div(),

            0xFF05 => {
                self.tima = value;

                // Writing TIMA during the reload window cancels
                // the pending reload.
                if self.reload_delay != 0 {
                    self.reload_delay = 0;
                }
            }

            0xFF06 => {
                self.tma = value;
            }

            0xFF07 => {
                self.write_tac(value);
            }

            _ => {}
        }
    }

    fn write_div(&mut self) {
        // Writing DIV resets the entire internal divider.
        //
        // This can itself create a falling edge on the timer input,
        // which may increment TIMA.
        let old_signal = self.timer_signal();

        self.div_counter = 0;

        let new_signal = self.timer_signal();

        if old_signal && !new_signal {
            self.increment_tima();
        }
    }

    fn write_tac(&mut self, value: u8) {
        // Only bits 2..0 are writable.
        let old_signal = self.timer_signal();

        self.tac = value & 0x07;

        let new_signal = self.timer_signal();

        // Changing TAC can itself cause a falling edge.
        if old_signal && !new_signal {
            self.increment_tima();
        }
    }

    /// Returns true if the timer has requested an interrupt.
    pub fn interrupt_pending(&self) -> bool {
        self.interrupt_pending
    }

    /// Clears and returns the pending interrupt request.
    pub fn take_interrupt(&mut self) -> bool {
        let pending = self.interrupt_pending;
        self.interrupt_pending = false;
        pending
    }

    #[cfg(test)]
    pub fn div_counter(&self) -> u16 {
        self.div_counter
    }

    #[cfg(test)]
    pub fn tima(&self) -> u8 {
        self.tima
    }

    #[cfg(test)]
    pub fn tma(&self) -> u8 {
        self.tma
    }

    #[cfg(test)]
    pub fn tac(&self) -> u8 {
        self.tac
    }
}

#[cfg(test)]
mod tests {
    use super::Timer;

    #[test]
    fn div_increments() {
        let mut timer = Timer::new();

        assert_eq!(timer.read(0xFF04), 0x00);

        timer.step(256);

        assert_eq!(timer.read(0xFF04), 0x01);
    }

    #[test]
    fn div_resets_on_write() {
        let mut timer = Timer::new();

        timer.step(1000);
        assert_ne!(timer.div_counter(), 0);

        timer.write(0xFF04, 0xFF);

        assert_eq!(timer.div_counter(), 0);
        assert_eq!(timer.read(0xFF04), 0x00);
    }

    #[test]
    fn tima_increments_at_4096hz() {
        let mut timer = Timer::new();

        timer.write(0xFF05, 0x00);
        timer.write(0xFF07, 0x04); // enable, 4096 Hz

        timer.step(1024);

        assert_eq!(timer.tima(), 0x01);
    }

    #[test]
    fn tima_increments_at_262144hz() {
        let mut timer = Timer::new();

        timer.write(0xFF05, 0x00);
        timer.write(0xFF07, 0x05); // enable, 262144 Hz

        timer.step(16);

        assert_eq!(timer.tima(), 0x01);
    }

    #[test]
    fn tima_overflow_reloads_from_tma() {
        let mut timer = Timer::new();

        timer.write(0xFF06, 0x42);
        timer.write(0xFF05, 0xFF);
        timer.write(0xFF07, 0x05); // enable, 262144 Hz

        timer.step(16);

        // Overflow happened, reload is delayed.
        assert_eq!(timer.tima(), 0x00);

        timer.step(4);

        assert_eq!(timer.tima(), 0x42);
        assert!(timer.interrupt_pending());
    }

    #[test]
    fn timer_interrupt_is_taken() {
        let mut timer = Timer::new();

        timer.write(0xFF06, 0x99);
        timer.write(0xFF05, 0xFF);
        timer.write(0xFF07, 0x05);

        timer.step(20);

        assert!(timer.interrupt_pending());
        assert!(timer.take_interrupt());
        assert!(!timer.interrupt_pending());
    }

    #[test]
    fn tac_can_disable_timer() {
        let mut timer = Timer::new();

        timer.write(0xFF05, 0x00);
        timer.write(0xFF07, 0x05); // enabled

        timer.step(16);

        assert_eq!(timer.tima(), 0x01);

        timer.write(0xFF07, 0x00); // disabled
        timer.step(32);

        assert_eq!(timer.tima(), 0x01);
    }
}