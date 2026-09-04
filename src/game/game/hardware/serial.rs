#[derive(Debug)]
pub struct Serial {
    sb: u8,
    sc: u8,
    transfer_cycles: u32,
    transferring: bool,
    interrupt: bool,
}

impl Serial {
    pub fn new() -> Self {
        Self {
            sb: 0,
            sc: 0x7E,
            transfer_cycles: 0,
            transferring: false,
            interrupt: false,
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        match address {
            0xFF01 => self.sb,
            0xFF02 => self.sc,
            _ => 0xFF,
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        match address {
            0xFF01 => {
                self.sb = value;
            }

            0xFF02 => {
                self.sc = value & 0x83;

                // Start transfer.
                if self.sc & 0x80 != 0 {
                    self.transferring = true;
                    self.transfer_cycles = 0;
                }
            }

            _ => {}
        }
    }

    pub fn step(&mut self, cycles: u32) {
        if !self.transferring {
            return;
        }

        self.transfer_cycles += cycles;

        // Simplified serial transfer:
        // complete after 8 bits × 512 T-cycles.
        if self.transfer_cycles >= 4096 {
            self.transfer_cycles = 0;
            self.transferring = false;

            // Transfer finished.
            self.sc &= !0x80;

            self.interrupt = true;
        }
    }

    pub fn take_interrupt(&mut self) -> bool {
        let pending = self.interrupt;
        self.interrupt = false;
        pending
    }
}

#[cfg(test)]
mod tests {
    use super::Serial;

    #[test]
    fn serial_registers_read_write() {
        let mut serial = Serial::new();

        serial.write(0xFF01, 0x55);
        assert_eq!(serial.read(0xFF01), 0x55);

        serial.write(0xFF02, 0x81);
        assert_eq!(serial.read(0xFF02), 0x81);
    }

    #[test]
    fn transfer_generates_interrupt() {
        let mut serial = Serial::new();

        serial.write(0xFF02, 0x81);

        assert!(!serial.take_interrupt());

        serial.step(4095);
        assert!(!serial.take_interrupt());

        serial.step(1);
        assert!(serial.take_interrupt());

        assert_eq!(serial.read(0xFF02) & 0x80, 0);
    }
}