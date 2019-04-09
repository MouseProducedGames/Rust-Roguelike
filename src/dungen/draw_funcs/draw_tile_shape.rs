/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

 **/
// External includes.

// Standard includes.

// Internal includes.
use crate::dungen::DungeonGenerator;
use crate::rrl_math::{Bounds, Position};
use crate::world::TiledArea;

pub struct DrawTileShape {
    tile_type: u32,
}

impl DrawTileShape {
    pub fn new(tile_type: u32) -> Self {
        Self { tile_type }
    }
}

impl DungeonGenerator for DrawTileShape {
    fn apply(&mut self, area: &mut dyn TiledArea, _generation_areas: &mut Vec<(Position, Position)>) {
        // let temp: &mut TiledShape2D = self;
        // for ( x, y ) in TiledShape2DCircumferenceIterator::new( self )
        let mut iter_index: u32 = 0;
        let mut keep_going: bool = true;
        while keep_going {
            match area.iter_circumference(&mut iter_index) {
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
