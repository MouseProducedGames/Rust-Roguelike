/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

 **/
// External includes.

// Standard includes.

// Internal includes.
use crate::world::TiledArea;

pub trait DungeonGenerator {
    fn apply(&mut self, area: &mut dyn TiledArea);
}
