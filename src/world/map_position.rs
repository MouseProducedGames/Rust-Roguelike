/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes
use specs::{Component, VecStorage};
use std::cmp::{Eq, PartialEq};
extern crate derive_more;

// Internal includes

#[derive(Copy, Clone, Debug, Default)]
pub struct MapPosition {
    x: u16,
    y: u16,
}

impl Component for MapPosition {
    type Storage = VecStorage<Self>;
}

impl MapPosition {
    pub(crate) fn new(x: u16, y: u16) -> Self {
        Self { x: x, y: y }
    }

    pub fn x(self) -> u16 {
        self.x
    }
    pub fn y(self) -> u16 {
        self.y
    }
}

impl Eq for MapPosition {}

impl PartialEq<MapPosition> for MapPosition {
    fn eq(&self, other: &MapPosition) -> bool {
        (self.x == other.x) && (self.y == other.y)
    }
}
