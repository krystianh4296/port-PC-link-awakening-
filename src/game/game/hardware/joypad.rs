#[derive(Debug)]
pub struct Joypad {
    joyp: u8,
    pressed: u8,
    interrupt: bool,
}

impl Joypad {
    pub fn new() -> Self {
        Self {
            joyp: 0xCF,
            pressed: 0,
            interrupt: false,
        }
    }

    pub fn read(&self) -> u8 {
        let mut value = self.joyp | 0x0F;
        // P14 low wybiera kierunki, P15 low wybiera przyciski akcji.
        if self.joyp & 0x10 == 0 { value &= 0xF0 | !(self.pressed & 0x0F); }
        if self.joyp & 0x20 == 0 { value &= 0xF0 | !((self.pressed >> 4) & 0x0F); }
        value
    }

    pub fn write(&mut self, value: u8) {
        self.joyp = (value & 0x30) | 0xC0;
    }

    pub fn button_pressed(&mut self, button: u8) {
        self.set_button(button, true);
    }

    pub fn set_button(&mut self, button: u8, pressed: bool) {
        let mask = 1 << button;
        let was_pressed = self.pressed & mask != 0;
        if pressed { self.pressed |= mask; } else { self.pressed &= !mask; }
        if pressed && !was_pressed { self.interrupt = true; }
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

    #[test]
    fn action_buttons_are_reported_when_action_group_is_selected() {
        let mut joypad = Joypad::new();
        joypad.write(0x10); // P15 low: select A/B/Select/Start
        joypad.set_button(4, true); // A
        joypad.set_button(7, true); // Start

        assert_eq!(joypad.read() & 0x0F, 0b0110);
        assert!(joypad.take_interrupt());
    }

    #[test]
    fn direction_and_action_groups_do_not_leak_into_each_other() {
        let mut joypad = Joypad::new();
        joypad.set_button(0, true); // Right
        joypad.set_button(4, true); // A

        joypad.write(0x20); // P14 low: directions
        assert_eq!(joypad.read() & 0x0F, 0b1110);
        joypad.write(0x10); // P15 low: actions
        assert_eq!(joypad.read() & 0x0F, 0b1110);
    }
}
