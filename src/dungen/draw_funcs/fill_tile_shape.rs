use crate::dungen::dungeon_generator::DungeonGenerator;
/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

 **/
// External includes.

// Internal includes.
use crate::world::TiledArea;

pub struct FillTileShape {
    tile_type: u32,
}

impl FillTileShape {
    pub fn new(tile_type: u32) -> FillTileShape {
        Self { tile_type }
    }
}

impl DungeonGenerator for FillTileShape {
    fn apply(&mut self, area: &mut dyn TiledArea) {
        let mut iter_index: u32 = 0;
        let mut keep_going: bool = true;
        while keep_going {
            let (x, y);
            match area.iter_surface_area(&mut iter_index) {
                Some((it_x, it_y)) => {
                    x = it_x;
                    y = it_y
                }
                _ => {
                    keep_going = false;
                    continue;
                }
            }
            *area.tile_type_mut(x, y) = self.tile_type;
        }
    }
}
