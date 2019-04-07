/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.

// Standard includes.
use std::default::Default;

// Internal includes.
use super::{Mapping, MapPosition};
use crate::multimap::Multimap;
use crate::rrl_math::Position;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum VisibilityType {
    None,
    Seen,
    Visible,
}

impl Default for VisibilityType {
    fn default() -> Self {
        VisibilityType::None
    }
}

pub struct VisibilityMap {
    values: Multimap<VisibilityType>,
}

impl VisibilityMap {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            values: Multimap::new(width, height),
        }
    }

    pub fn clear(&mut self) {
        for pos in self.get_position(0, 0) {
            if *self.values.value(pos) == VisibilityType::Visible {
                *self.values.value_mut(pos) = VisibilityType::Seen;
            }
        }
    }

    pub fn height(&self) -> u16 {
        self.values.height()
    }

    pub fn width(&self) -> u16 {
        self.values.width()
    }

    pub fn value(&self, pos: MapPosition) -> VisibilityType {
        *self.values.value(pos)
    }

    pub fn value_pos(&self, pos: Position) -> VisibilityType {
        if self.is_pos_in_bounds(pos) {
            self.value(MapPosition::new(pos.x as u16, pos.y as u16))
        } else {
            VisibilityType::None
        }
    }

    pub fn value_mut(&mut self, pos: MapPosition) -> &mut VisibilityType {
        self.values.value_mut(pos)
    }
}

impl Mapping for VisibilityMap {
    fn height(&self) -> u16 {
        VisibilityMap::height(self)
    }

    fn width(&self) -> u16 {
        VisibilityMap::width(self)
    }
}
