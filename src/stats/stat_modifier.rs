/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

 **/
// External includes

// internal include
use crate::stats::Stat;

pub trait StatModifier: Stat {
    fn modifier(&self) -> i32 {
        (self.value() / 2) - 5
    }
}
