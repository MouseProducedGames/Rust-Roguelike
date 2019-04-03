/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes

// Internal include
use crate::abilities::Ability;

pub enum Item {
    // Name, Ability.
    Generic(String, Ability),
}
