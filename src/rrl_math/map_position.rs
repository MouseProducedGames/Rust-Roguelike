/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes
use specs::{Component, VecStorage};
use std::cmp::{Eq, PartialEq};
use std::convert::From;
extern crate derive_more;
use std::iter::Iterator;
use std::ops::{Add, Sub};

// Internal includes
use super::MapDisplacement;

#[derive(Copy, Clone, Debug, Default)]
pub struct MapPosition {
    pub x: u16,
    pub y: u16,
    x_bound: u16,
    y_bound: u16,
}

impl Component for MapPosition {
    type Storage = VecStorage<Self>;
}

impl MapPosition {
    pub fn new(x: u16, y: u16, x_bound: u16, y_bound: u16) -> Self {
        Self {
            x: x,
            y: y,
            x_bound: x_bound,
            y_bound: y_bound,
        }
    }
}

impl Add<MapDisplacement> for MapPosition {
    type Output = MapPosition;

    fn add(self, other: MapDisplacement) -> MapPosition {
        MapPosition {
            x: self.x + other.x,
            y: self.y + other.y,
            x_bound: self.x_bound,
            y_bound: self.y_bound,
        }
    }
}

impl Eq for MapPosition {}

impl From<MapPosition> for bool {
    fn from(item: MapPosition) -> bool {
        (item.x <= item.x_bound) && (item.y <= item.y_bound)
    }
}

/* impl From<MapDisplacement> for MapPosition {
    fn from(item: MapDisplacement) -> MapPosition {
        MapPosition::new(item.x, item.y)
    }
} */

impl Iterator for MapPosition {
    type Item = MapPosition;
    fn next(&mut self) -> Option<Self::Item> {
        let output = *self;
        if (output.x >= output.x_bound) && (output.y >= output.y_bound) {
            return None;
        }

        self.x += 1;
        if self.x >= self.x_bound {
            self.x = 0;
            self.y += 1;
            if self.y >= self.y_bound {
                return None;
            }
        }

        Some(output)
    }
}

impl PartialEq<bool> for MapPosition {
    fn eq(&self, other: &bool) -> bool {
        bool::from(*self) == *other
    }
}

impl PartialEq<MapPosition> for MapPosition {
    fn eq(&self, other: &MapPosition) -> bool {
        (self.x == other.x)
            && (self.y == other.y)
            && (self.x_bound == other.x_bound)
            && (self.y_bound == other.y_bound)
    }
}

impl Sub<MapDisplacement> for MapPosition {
    type Output = MapPosition;

    fn sub(self, other: MapDisplacement) -> MapPosition {
        MapPosition {
            x: self.x - other.x,
            y: self.y - other.y,
            x_bound: self.x_bound,
            y_bound: self.y_bound,
        }
    }
}

impl Sub<MapPosition> for MapPosition {
    type Output = MapDisplacement;

    fn sub(self, other: MapPosition) -> MapDisplacement {
        MapDisplacement {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
