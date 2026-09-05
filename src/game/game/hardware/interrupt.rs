#[derive(Debug)]
pub struct InterruptController {
    ie: u8,
    if_reg: u8,
}

impl InterruptController {
    pub fn new() -> Self {
        Self {
            ie: 0,
            if_reg: 0,
        }
    }

    pub fn read_ie(&self) -> u8 {
        self.ie
    }

    pub fn write_ie(&mut self, value: u8) {
        self.ie = value;
    }

    pub fn read_if(&self) -> u8 {
        self.if_reg | 0xE0
    }

    pub fn write_if(&mut self, value: u8) {
        self.if_reg = value & 0x1F;
    }

    pub fn request(&mut self, bit: u8) {
        self.if_reg |= 1 << bit;
    }

    pub fn clear(&mut self, bit: u8) {
        self.if_reg &= !(1 << bit);
    }

    pub fn pending(&self) -> u8 {
        self.if_reg & self.ie & 0x1F
    }
}
#[cfg(test)]
mod tests {
    use super::InterruptController;

    #[test]
    fn request_sets_if_bit() {
        let mut ic = InterruptController::new();

        ic.request(2);

        assert_eq!(ic.read_if() & 0x04, 0x04);
    }

    #[test]
    fn clear_clears_if_bit() {
        let mut ic = InterruptController::new();

        ic.request(2);
        ic.clear(2);

        assert_eq!(ic.read_if() & 0x04, 0);
    }

    #[test]
    fn pending_masks_ie_and_if() {
        let mut ic = InterruptController::new();

        ic.write_ie(0x05); // VBlank + Timer

        ic.request(0);
        ic.request(2);
        ic.request(4);

        assert_eq!(ic.pending(), 0x05);
    }

    #[test]
    fn write_if_only_keeps_lower_five_bits() {
        let mut ic = InterruptController::new();

        ic.write_if(0xFF);

        assert_eq!(ic.read_if() & 0x1F, 0x1F);
    }

    #[test]
    fn read_if_sets_unused_bits() {
        let mut ic = InterruptController::new();
        ic.write_if(0x05);

        assert_eq!(ic.read_if(), 0xE5);
    }
}
