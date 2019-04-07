/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use specs::{Component, VecStorage};

// Standard includes.
use std::cmp::{Eq, PartialEq};
use std::convert::From;
use std::ops::{Add, Sub};

// Internal includes.
use super::DisplacementU32;

#[derive(Copy, Clone, Debug, Default)]
pub struct PositionU32 {
    pub x: u32,
    pub y: u32,
}

impl Component for PositionU32 {
    type Storage = VecStorage<Self>;
}

impl PositionU32 {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}

impl Add<DisplacementU32> for PositionU32 {
    type Output = PositionU32;

    fn add(self, other: DisplacementU32) -> PositionU32 {
        PositionU32 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Eq for PositionU32 {}

impl From<DisplacementU32> for PositionU32 {
    fn from(item: DisplacementU32) -> PositionU32 {
        PositionU32::new(item.x, item.y)
    }
}

impl PartialEq for PositionU32 {
    fn eq(&self, other: &PositionU32) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Sub<DisplacementU32> for PositionU32 {
    type Output = PositionU32;

    fn sub(self, other: DisplacementU32) -> PositionU32 {
        PositionU32 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Sub<PositionU32> for PositionU32 {
    type Output = DisplacementU32;

    fn sub(self, other: PositionU32) -> DisplacementU32 {
        DisplacementU32 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
