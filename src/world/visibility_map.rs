// External includes
use std::default::Default;

// Internal includes
use super::super::multimap::Multimap;
use crate::rrl_math::Position;
use crate::world::Mapping;

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
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            values: Multimap::new(width, height),
        }
    }

    pub fn clear(&mut self) {
        for y in 0..self.values.height() {
            for x in 0..self.values.width() {
                if self.values.is_in_bounds(x, y)
                    && *self.values.value(x, y) == VisibilityType::Visible
                {
                    *self.values.value_mut(x, y) = VisibilityType::Seen;
                }
            }
        }
    }

    pub fn height(&self) -> u32 {
        self.values.height()
    }

    pub fn width(&self) -> u32 {
        self.values.width()
    }

    pub fn value(&self, pos_x: u32, pos_y: u32) -> VisibilityType {
        if self.is_in_bounds(pos_x, pos_y) {
            *self.values.value(pos_x, pos_y)
        } else {
            VisibilityType::None
        }
    }

    pub fn value_pos(&self, pos: Position) -> VisibilityType {
        if self.is_pos_in_bounds(pos) {
            self.value(pos.x as u32, pos.y as u32)
        } else {
            VisibilityType::None
        }
    }

    pub fn value_mut(&mut self, pos_x: u32, pos_y: u32) -> &mut VisibilityType {
        self.values.value_mut(pos_x, pos_y)
    }
}

impl Mapping for VisibilityMap {
    fn height(&self) -> u32 {
        VisibilityMap::height(self)
    }

    fn width(&self) -> u32 {
        VisibilityMap::width(self)
    }
}
