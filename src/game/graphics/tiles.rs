pub type TilePixel = u8;

#[derive(Clone, Debug)]
pub struct Tile {
    pixels: [[TilePixel; 8]; 8],
}

impl Tile {
    pub fn decode(bytes: &[u8; 16]) -> Self {
        let mut pixels = [[0u8; 8]; 8];

        for y in 0..8 {
            let low = bytes[y * 2];
            let high = bytes[y * 2 + 1];

            for x in 0..8 {
                let bit = 7 - x;

                let lo = (low >> bit) & 1;
                let hi = (high >> bit) & 1;

                pixels[y][x] = lo | (hi << 1);
            }
        }

        Self { pixels }
    }

    pub fn pixel(&self, x: usize, y: usize) -> TilePixel {
        self.pixels[y][x]
    }

    pub fn pixels(&self) -> &[[TilePixel; 8]; 8] {
        &self.pixels
    }
}