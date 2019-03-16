/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/

#[derive(Clone, Copy)]
pub struct TileTypeData
{
    passable: bool,
    transparent: bool,
}

pub const TILE_TYPE_INDEX_VOID: u32 = 0;
pub static TILE_TYPE_DATA: [TileTypeData; 4] =
    [
        // The void.
        TileTypeData{ passable: false, transparent: true  },
        // Wall.
        TileTypeData{ passable: false, transparent: false },
        // Floor.
        TileTypeData{ passable: true,  transparent: true  },
        // Door.
        TileTypeData{ passable: true,  transparent: false  }
    ];

impl TileTypeData
{
    pub fn _new(passable: bool, transparent: bool) -> Self
    {
        Self {
            passable,
            transparent,
        }
    }

    pub fn passable( self ) -> bool
    {
        self.passable
    }

    pub fn transparent( self ) -> bool
    {
        self.transparent
    }
}
