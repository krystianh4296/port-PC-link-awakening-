#[derive(Debug)]
pub struct Ppu {
    ly: u8,
    lyc: u8,
    lcdc: u8,

    stat: u8,

    cycle_counter: u32,
    mode: u8,

    vblank_interrupt: bool,
    stat_interrupt: bool,

    stat_irq_line: bool,
}

impl Ppu {
    pub fn new() -> Self {
        Self {
            ly: 0,
            lyc: 0,
            lcdc: 0x91,
            stat: 0x80,

            cycle_counter: 0,
            mode: 2,

            vblank_interrupt: false,
            stat_interrupt: false,

            stat_irq_line: false,
        }
    }

    pub fn step(&mut self, cycles: u32) {
        for _ in 0..cycles {
            self.step_cycle();
        }
    }

    fn step_cycle(&mut self) {
        self.cycle_counter += 1;

        match self.mode {
            // OAM search
            2 => {
                if self.cycle_counter >= 80 {
                    self.cycle_counter = 0;
                    self.set_mode(3);
                }
            }

            // Pixel transfer
            3 => {
                if self.cycle_counter >= 172 {
                    self.cycle_counter = 0;
                    self.set_mode(0);
                }
            }

            // HBlank
            0 => {
                if self.cycle_counter >= 204 {
                    self.cycle_counter = 0;
                    self.advance_line();
                }
            }

            // VBlank
            1 => {
                if self.cycle_counter >= 456 {
                    self.cycle_counter = 0;
                    self.advance_line();
                }
            }

            _ => unreachable!(),
        }
    }

    fn advance_line(&mut self) {
        self.ly = self.ly.wrapping_add(1);

        if self.ly == 144 {
            self.set_mode(1);
            self.vblank_interrupt = true;
        } else if self.ly > 153 {
            self.ly = 0;
            self.set_mode(2);
        } else if self.ly >= 144 {
            self.set_mode(1);
        } else {
            self.set_mode(2);
        }

        self.update_lyc_flag();
        self.update_stat_interrupt();
    }

    fn set_mode(&mut self, mode: u8) {
        self.mode = mode;

        self.stat = (self.stat & !0x03) | mode;

        self.update_stat_interrupt();
    }

    fn update_lyc_flag(&mut self) {
        if self.ly == self.lyc {
            self.stat |= 0x04;
        } else {
            self.stat &= !0x04;
        }
    }

    fn update_stat_interrupt(&mut self) {
        let coincidence = self.ly == self.lyc;

        if coincidence {
            self.stat |= 0x04;
        } else {
            self.stat &= !0x04;
        }

        let hblank_enabled = self.stat & 0x08 != 0;
        let vblank_enabled = self.stat & 0x10 != 0;
        let oam_enabled = self.stat & 0x20 != 0;
        let lyc_enabled = self.stat & 0x40 != 0;

        let hblank_condition = self.mode == 0 && hblank_enabled;
        let vblank_condition = self.mode == 1 && vblank_enabled;
        let oam_condition = self.mode == 2 && oam_enabled;
        let lyc_condition = coincidence && lyc_enabled;

        let irq_line =
            hblank_condition ||
            vblank_condition ||
            oam_condition ||
            lyc_condition;

        // STAT interrupt is triggered on a rising edge.
        if irq_line && !self.stat_irq_line {
            self.stat_interrupt = true;
        }

        self.stat_irq_line = irq_line;
    }

    pub fn read(&self, address: u16) -> u8 {
        match address {
            0xFF40 => self.lcdc,
            0xFF41 => self.stat | 0x80,
            0xFF44 => self.ly,
            0xFF45 => self.lyc,
            _ => 0xFF,
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        match address {
            0xFF40 => {
                self.lcdc = value;
            }

            0xFF41 => {
                self.stat = (self.stat & 0x07) | (value & 0x78) | 0x80;
                self.update_stat_interrupt();
            }

            0xFF45 => {
                self.lyc = value;
                self.update_lyc_flag();
                self.update_stat_interrupt();
            }

            0xFF44 => {}

            _ => {}
        }
    }

    pub fn take_vblank_interrupt(&mut self) -> bool {
        let pending = self.vblank_interrupt;
        self.vblank_interrupt = false;
        pending
    }

    pub fn take_stat_interrupt(&mut self) -> bool {
        let pending = self.stat_interrupt;
        self.stat_interrupt = false;
        pending
    }

    #[cfg(test)]
    pub fn ly(&self) -> u8 {
        self.ly
    }

    #[cfg(test)]
    pub fn mode(&self) -> u8 {
        self.mode
    }
}

#[cfg(test)]
mod tests {
    use super::Ppu;

    #[test]
    fn vblank_starts_at_ly_144() {
        let mut ppu = Ppu::new();

        ppu.step(456 * 143);

        assert_eq!(ppu.ly(), 143);
        assert!(!ppu.take_vblank_interrupt());

        ppu.step(456);

        assert_eq!(ppu.ly(), 144);
        assert!(ppu.take_vblank_interrupt());
        assert!(!ppu.take_vblank_interrupt());
    }

    #[test]
    fn ppu_modes_follow_scanline_timing() {
        let mut ppu = Ppu::new();

        assert_eq!(ppu.mode(), 2);

        ppu.step(80);
        assert_eq!(ppu.mode(), 3);

        ppu.step(172);
        assert_eq!(ppu.mode(), 0);

        ppu.step(204);
        assert_eq!(ppu.ly(), 1);
        assert_eq!(ppu.mode(), 2);
    }

    #[test]
    fn stat_read_write() {
        let mut ppu = Ppu::new();

        ppu.write(0xFF41, 0x78);

        let stat = ppu.read(0xFF41);

        assert_eq!(stat & 0x78, 0x78);
        assert_eq!(stat & 0x80, 0x80);
    }

    #[test]
    fn lyc_coincidence_flag() {
        let mut ppu = Ppu::new();

        ppu.write(0xFF45, 0);

        assert_eq!(ppu.read(0xFF41) & 0x04, 0x04);

        ppu.write(0xFF45, 5);

        assert_eq!(ppu.read(0xFF41) & 0x04, 0);
    }

    #[test]
    fn stat_oam_interrupt() {
        let mut ppu = Ppu::new();

        // Enable OAM STAT interrupt.
        ppu.write(0xFF41, 0x20);

        // PPU starts in mode 2.
        assert!(ppu.take_stat_interrupt());

        // Interrupt was consumed.
        assert!(!ppu.take_stat_interrupt());

        // Move through mode 3 -> mode 0 -> next line mode 2.
        ppu.step(80);
        ppu.step(172);
        ppu.step(204);

        assert_eq!(ppu.mode(), 2);
        assert!(ppu.take_stat_interrupt());
    }

    #[test]
    fn stat_hblank_interrupt() {
        let mut ppu = Ppu::new();

        // Enable HBlank STAT interrupt.
        ppu.write(0xFF41, 0x08);

        ppu.step(80);
        ppu.step(172);

        assert_eq!(ppu.mode(), 0);
        assert!(ppu.take_stat_interrupt());

        assert!(!ppu.take_stat_interrupt());
    }

    #[test]
    fn stat_lyc_interrupt() {
        let mut ppu = Ppu::new();

        // LYC = 1
        ppu.write(0xFF45, 1);

        // Enable LYC STAT interrupt.
        ppu.write(0xFF41, 0x40);

        ppu.step(456);

        assert_eq!(ppu.ly(), 1);
        assert!(ppu.take_stat_interrupt());

        assert!(!ppu.take_stat_interrupt());
    }
}