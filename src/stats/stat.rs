/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/

// External includes.

// Standard includes.

// Internal includes.

pub trait Stat {
    fn value(&self) -> i32;

    fn value_mut(&mut self) -> &mut i32;
}
