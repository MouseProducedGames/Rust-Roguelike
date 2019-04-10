/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes
use specs::{Component, VecStorage};

// Standard includes.
use std::cmp::{Eq, PartialEq};
use std::convert::From;
use std::iter::Iterator;
use std::ops::{Add, Sub};

// Internal includes
use super::{MapDisplacement, MapPosition};

#[derive(Copy, Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct MapScanPosition {
    pub x: u16,
    pub y: u16,
    x_bound: u16,
    y_bound: u16,
}

impl Component for MapScanPosition {
    type Storage = VecStorage<Self>;
}

impl MapScanPosition {
    pub(crate) fn new(x: u16, y: u16, x_bound: u16, y_bound: u16) -> Self {
        Self {
            x,
            y,
            x_bound,
            y_bound,
        }
    }

    pub fn unwrap(self) -> MapPosition {
        if bool::from(self) {
            MapPosition::new(self.x, self.y)
        } else {
            panic!("Tried to access an invalid map position!");
        }
    }
}

impl Add<MapDisplacement> for MapScanPosition {
    type Output = MapScanPosition;

    fn add(self, other: MapDisplacement) -> MapScanPosition {
        MapScanPosition {
            x: self.x + other.x,
            y: self.y + other.y,
            x_bound: self.x_bound,
            y_bound: self.y_bound,
        }
    }
}

impl From<MapScanPosition> for bool {
    fn from(item: MapScanPosition) -> bool {
        (item.x <= item.x_bound) && (item.y <= item.y_bound)
    }
}

impl Iterator for MapScanPosition {
    type Item = MapPosition;

    fn next(&mut self) -> Option<Self::Item> {
        if *self == false {
            return None;
        }
        let output = self.unwrap();

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

impl PartialEq<bool> for MapScanPosition {
    fn eq(&self, other: &bool) -> bool {
        bool::from(*self) == *other
    }
}

impl Sub<MapDisplacement> for MapScanPosition {
    type Output = MapScanPosition;

    fn sub(self, other: MapDisplacement) -> MapScanPosition {
        MapScanPosition {
            x: self.x - other.x,
            y: self.y - other.y,
            x_bound: self.x_bound,
            y_bound: self.y_bound,
        }
    }
}

impl Sub<MapScanPosition> for MapScanPosition {
    type Output = MapDisplacement;

    fn sub(self, other: MapScanPosition) -> MapDisplacement {
        MapDisplacement {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
