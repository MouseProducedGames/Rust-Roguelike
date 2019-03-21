/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
use crate::rrl_math::Position;

pub trait Mapping {
    fn height(&self) -> u32;

    fn width(&self) -> u32;

    fn bounds(&self) -> (u32, u32) {
        (self.width(), self.height())
    }

    fn is_i32_in_bounds(&self, pos_x: i32, pos_y: i32) -> bool {
        self.is_i32_in_height(pos_y) && self.is_i32_in_width(pos_x)
    }

    fn is_in_bounds(&self, pos_x: u32, pos_y: u32) -> bool {
        self.is_in_height(pos_y) && self.is_in_width(pos_x)
    }

    fn is_pos_in_bounds(&self, pos: Position) -> bool {
        self.is_i32_in_bounds(pos.x, pos.y)
    }

    fn is_i32_in_height(&self, pos_y: i32) -> bool {
        pos_y >= 0 && self.is_in_height(pos_y as u32)
    }

    fn is_i32_in_width(&self, pos_x: i32) -> bool {
        pos_x >= 0 && self.is_in_width(pos_x as u32)
    }

    fn is_in_height(&self, pos_y: u32) -> bool {
        pos_y < self.height()
    }

    fn is_in_width(&self, pos_x: u32) -> bool {
        pos_x < self.width()
    }
}
