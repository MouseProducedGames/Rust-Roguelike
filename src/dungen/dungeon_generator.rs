/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

 **/
// External includes.

// Standard includes.

// Internal includes.
use crate::world::{Mapping, TiledArea};

pub trait DungeonGenerator {
    fn apply<TArea>(&mut self, area: &mut TArea)
    where
        TArea: TiledArea + Mapping;
}
