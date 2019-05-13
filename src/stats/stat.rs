/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/

// External includes.

// Standard includes.

// Internal includes.
use crate::game::GameValueFixed;

pub trait Stat {
    fn value(&self) -> GameValueFixed;

    fn value_mut(&mut self) -> &mut GameValueFixed;
}
