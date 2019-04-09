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

pub trait DungeonGenerator {
    fn apply(&mut self, area: &mut dyn TiledArea);
}
