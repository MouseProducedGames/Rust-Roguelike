use specs::{Component, VecStorage};
/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.

// Standard includes.
use std::cmp::{Eq, PartialEq};
use std::convert::From;
use std::ops::{Add, Sub};

// Internal includes.
use super::MapScanPosition;

#[derive(Copy, Clone, Debug)]
pub struct MapDisplacement {
    pub x: u16,
    pub y: u16,
}

impl Component for MapDisplacement {
    type Storage = VecStorage<Self>;
}

impl MapDisplacement {
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }

    pub fn _length_sqr(self) -> u16 {
        self.x * self.x + self.y * self.y
    }
}

impl Add<MapDisplacement> for MapDisplacement {
    type Output = MapDisplacement;

    fn add(self, other: MapDisplacement) -> MapDisplacement {
        MapDisplacement {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Eq for MapDisplacement {}

impl From<MapScanPosition> for MapDisplacement {
    fn from(item: MapScanPosition) -> MapDisplacement {
        MapDisplacement::new(item.x, item.y)
    }
}

impl PartialEq for MapDisplacement {
    fn eq(&self, other: &MapDisplacement) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Sub for MapDisplacement {
    type Output = MapDisplacement;

    fn sub(self, other: MapDisplacement) -> MapDisplacement {
        MapDisplacement {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
