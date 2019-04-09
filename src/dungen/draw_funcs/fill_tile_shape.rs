use crate::dungen::dungeon_generator::DungeonGenerator;
/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

 **/
// External includes.

// Standard includes.
use std::cell::RefCell;
use std::rc::Rc;

// Internal includes.
use crate::world::{Mapping, TiledArea};

pub struct FillTileShape {
    tile_type: u32,
}

impl FillTileShape {
    pub fn new(tile_type: u32) -> FillTileShape {
        Self { tile_type }
    }
}

impl DungeonGenerator for FillTileShape {
    fn apply<'a>(&'a mut self, area: &'a mut (dyn TiledArea + 'a)) where dyn TiledArea + 'a: Mapping<'a>
    {
        let mut iter_index: u32 = 0;
        let mut keep_going: bool = true;
        while keep_going {
            match area.iter_surface_area(&mut iter_index) {
                Some(pos) => {
                    *area.tile_type_mut(pos) = self.tile_type;
                }
                _ => {
                    keep_going = false;
                    continue;
                }
            }
        }
    }
}
