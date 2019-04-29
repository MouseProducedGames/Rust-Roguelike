/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.

// Standard includes.

// Internal includes.
use crate::maps::Mapping;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bounds {
    pub width: u16,
    pub height: u16,
}

impl Bounds {
    pub fn new(width: u16, height: u16) -> Self {
        Self { width, height }
    }
}

impl<'a> Mapping<'a> for Bounds {
    fn height(&self) -> u16 {
        self.height
    }
    fn width(&self) -> u16 {
        self.width
    }
}
