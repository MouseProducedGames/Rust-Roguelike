/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

 **/

// External includes.

// Standard includes.

// Internal includes.

#[derive(Clone, Copy)]
pub struct TileTypeData {
    passable: bool,
    transparent: bool,
}

pub const TILE_TYPE_INDEX_VOID: u32 = 0;
pub const TILE_TYPE_INDEX_WALL: u32 = 1;
pub const TILE_TYPE_INDEX_FLOOR: u32 = 2;
pub const TILE_TYPE_INDEX_DOOR: u32 = 3;
pub static TILE_TYPE_DATA: [TileTypeData; 8] = [
    // The void.
    TileTypeData {
        passable: false,
        transparent: true,
    },
    // Wall.
    TileTypeData {
        passable: false,
        transparent: false,
    },
    // Floor.
    TileTypeData {
        passable: true,
        transparent: true,
    },
    // Door.
    TileTypeData {
        passable: true,
        transparent: false,
    },
    // Open door.
    TileTypeData {
        passable: true,
        transparent: true,
    },
    // Secret door.
    TileTypeData {
        passable: true,
        transparent: false,
    },
    // Discovered secret door.
    TileTypeData {
        passable: true,
        transparent: false,
    },
    // Open secret door.
    TileTypeData {
        passable: true,
        transparent: true,
    },
];

impl TileTypeData {
    pub fn _new(passable: bool, transparent: bool) -> Self {
        Self {
            passable,
            transparent,
        }
    }

    pub fn passable(self) -> bool {
        self.passable
    }

    pub fn transparent(self) -> bool {
        self.transparent
    }
}
