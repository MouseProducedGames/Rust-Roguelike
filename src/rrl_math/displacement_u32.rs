use specs::{Component, VecStorage};
/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes

// Standard includes.
use std::cmp::{Eq, PartialEq};
use std::convert::From;
use std::ops::{Add, Sub};

// Internal includes
use super::PositionU32;

#[derive(Copy, Clone, Debug)]
pub struct DisplacementU32 {
    pub x: u32,
    pub y: u32,
}

impl Component for DisplacementU32 {
    type Storage = VecStorage<Self>;
}

impl DisplacementU32 {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }

    pub fn _length_sqr(self) -> u32 {
        self.x * self.x + self.y * self.y
    }
}

impl Add<DisplacementU32> for DisplacementU32 {
    type Output = DisplacementU32;

    fn add(self, other: DisplacementU32) -> DisplacementU32 {
        DisplacementU32 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Eq for DisplacementU32 {}

impl From<PositionU32> for DisplacementU32 {
    fn from(item: PositionU32) -> DisplacementU32 {
        DisplacementU32::new(item.x, item.y)
    }
}

impl PartialEq for DisplacementU32 {
    fn eq(&self, other: &DisplacementU32) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Sub for DisplacementU32 {
    type Output = DisplacementU32;

    fn sub(self, other: DisplacementU32) -> DisplacementU32 {
        DisplacementU32 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
