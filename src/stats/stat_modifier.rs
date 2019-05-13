/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

 **/
// External includes.

// Standard includes.

// Internal includes.
use super::Stat;
use crate::game::GameValueFixed;

pub trait StatModifier: Stat {
    fn modifier(&self) -> GameValueFixed {
        (self.value() / 2) - GameValueFixed::from_int(5)
    }
}
