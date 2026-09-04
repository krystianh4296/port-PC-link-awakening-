#[derive(Debug)]
pub struct Joypad {
    joyp: u8,
    interrupt: bool,
}

impl Joypad {
    pub fn new() -> Self {
        Self {
            joyp: 0xCF,
            interrupt: false,
        }
    }

    pub fn read(&self) -> u8 {
        self.joyp
    }

    pub fn write(&mut self, value: u8) {
        self.joyp = (value & 0x30) | 0xC0;
    }

    pub fn button_pressed(&mut self, button: u8) {
        self.joyp &= !(1 << button);
        self.interrupt = true;
    }

    pub fn take_interrupt(&mut self) -> bool {
        let pending = self.interrupt;
        self.interrupt = false;
        pending
    }
}

#[cfg(test)]
mod tests {
    use super::Joypad;

    #[test]
    fn joyp_read_write() {
        let mut joypad = Joypad::new();

        joypad.write(0x10);

        assert_eq!(joypad.read() & 0x30, 0x10);
    }

    #[test]
    fn button_press_generates_interrupt() {
        let mut joypad = Joypad::new();

        assert!(!joypad.take_interrupt());

        joypad.button_pressed(0);

        assert!(joypad.take_interrupt());
        assert_eq!(joypad.read() & 0x01, 0);
    }
}