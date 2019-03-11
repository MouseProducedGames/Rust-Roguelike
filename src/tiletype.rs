pub struct TileTypeData
{
    passable: bool,
    // transparent: bool,
}

pub const TILE_TYPE_INDEX_VOID: u32 = 0;
pub static TILE_TYPE_DATA: [TileTypeData; 3] =
    [
        // The void.
        TileTypeData{ passable: false, /* transparent: true */ },
        // Wall.
        TileTypeData{ passable: false, /* transparent: false */ },
        // Floor.
        TileTypeData{ passable: true, /* transparent: true */ }
    ];

impl TileTypeData
{
    /* pub fn new(passable: bool, transparent: bool) -> Self
    {
        Self {
            passable: passable,
            transparent: transparent,
        }
    } */

    pub fn passable(&self) -> bool
    {
        self.passable
    }

    /* pub fn transparent(&self) -> bool
    {
        self.transparent
    } */
}
