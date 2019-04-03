/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

 **/
// External includes.

// Internal includes.
use crate::world::{Mapping, Tilemap};

pub trait DungenCommon {
    fn create_new(width: u16, height: u16) -> Tilemap {
        Tilemap::new(width, height)
    }

    fn finish(&mut self) -> Tilemap;
}

impl DungenCommon for Tilemap {
    fn finish(&mut self) -> Tilemap {
        let mut output = Tilemap::new(self.width(), self.height());
        for pos in self.get_position(0, 0) {
            *output.tile_type_mut(pos) = self.tile_type(pos);
        }

        output
    }
}
