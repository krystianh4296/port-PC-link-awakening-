#[derive(Clone, Copy, Debug)]
pub struct GraphicsBlock {
    pub name: &'static str,
    pub bank: usize,
    pub offset: usize,
    pub size: usize,
}

impl GraphicsBlock {
    pub const fn new(
        name: &'static str,
        bank: usize,
        offset: usize,
        size: usize,
    ) -> Self {
        Self {
            name,
            bank,
            offset,
            size,
        }
    }

    pub fn rom_offset(&self) -> usize {
        self.bank * 0x4000 + self.offset
    }

    pub fn tile_count(&self) -> usize {
        self.size / 16
    }
    
}
pub const GRAPHICS_BANK_0C: &[GraphicsBlock] = &[
    GraphicsBlock::new(
        "LinkCharacterTiles",
        0x0C,
        0x0000,
        0,
    ),
    GraphicsBlock::new(
        "CharacterVfxTiles",
        0x0C,
        0x0000,
        0,
    ),
    GraphicsBlock::new(
        "Items1Tiles",
        0x0C,
        0x0000,
        0,
    ),
    GraphicsBlock::new(
        "InventoryEquipmentItemsTiles",
        0x0C,
        0x0000,
        0,
    ),
    GraphicsBlock::new(
        "InventoryOverworldItemsTiles",
        0x0C,
        0x0000,
        0,
    ),
    GraphicsBlock::new(
        "SirenInstrumentsTiles",
        0x0C,
        0x0000,
        0,
    ),
    GraphicsBlock::new(
        "Overworld1Tiles",
        0x0C,
        0x0000,
        0,
    ),
    GraphicsBlock::new(
        "OverworldLandscapeTiles",
        0x0C,
        0x0000,
        0,
    ),
    GraphicsBlock::new(
        "LinkCharacter2Tiles",
        0x0C,
        0x0000,
        0,
    ),
];