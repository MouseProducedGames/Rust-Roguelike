/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes
use specs::{Component, VecStorage};

// Standard includes.

// Internal includes

#[derive(Copy, Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct MapPosition {
    x: u16,
    y: u16,
}

impl Component for MapPosition {
    type Storage = VecStorage<Self>;
}

impl MapPosition {
    pub(crate) fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }

    pub fn x(self) -> u16 {
        self.x
    }
    pub fn y(self) -> u16 {
        self.y
    }
}
