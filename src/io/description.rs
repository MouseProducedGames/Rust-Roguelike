/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes

// Standard includes.
use std::ops::Deref;

// internal includes

pub struct LongDescription(pub &'static str);

impl Deref for LongDescription {
    type Target = &'static str;

    fn deref(&self) -> &&'static str {
        &&self.0
    }
}

pub struct ShortDescription(pub &'static str);

impl Deref for ShortDescription {
    type Target = &'static str;

    fn deref(&self) -> &&'static str {
        &&self.0
    }
}
