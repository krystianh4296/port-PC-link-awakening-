pub type TilePixel = u8;

#[derive(Clone, Debug)]
pub struct Tile {
    pixels: [[TilePixel; 8]; 8],
}

impl Tile {
    pub fn render(
        &self,
        renderer: &mut Renderer,
        x: usize,
        y: usize,
    ) {
        const COLORS: [u32; 4] = [
            0xFFFFFFFF,
            0xFFAAAAAA,
            0xFF555555,
            0xFF000000,
        ];

        for py in 0..8 {
            for px in 0..8 {
                let color = COLORS[self.pixel(px, py) as usize];
                renderer.set_pixel(x + px, y + py, color);
            }
        }
    }
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