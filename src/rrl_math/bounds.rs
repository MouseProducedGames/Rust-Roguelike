/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes

// Internal includes
use crate::world::Mapping;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bounds {
    pub width: u16,
    pub height: u16,
}

impl Mapping for Bounds {
    fn height(&self) -> u16 {
        self.height
    }
    fn width(&self) -> u16 {
        self.width
    }
}
