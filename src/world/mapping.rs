/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.

// Standard includes.

// Internal includes.
use super::MapScanPosition;
use crate::rrl_math::Position;

pub trait Mapping<'a> {
    fn height(&'a self) -> u16;

    fn width(&'a self) -> u16;

    fn bounds(&'a self) -> (u16, u16) {
        (self.width(), self.height())
    }

    fn get_position(&'a self, x: u16, y: u16) -> MapScanPosition {
        MapScanPosition::new(x, y, self.width(), self.height())
    }

    fn is_i32_in_bounds(&'a self, pos_x: i32, pos_y: i32) -> bool {
        self.is_i32_in_height(pos_y) && self.is_i32_in_width(pos_x)
    }

    fn is_in_bounds(&'a self, pos_x: u16, pos_y: u16) -> bool {
        self.is_in_height(pos_y) && self.is_in_width(pos_x)
    }

    fn is_pos_in_bounds(&'a self, pos: Position) -> bool {
        self.is_i32_in_bounds(pos.x, pos.y)
    }

    fn is_i32_in_height(&'a self, pos_y: i32) -> bool {
        pos_y >= 0 && self.is_in_height(pos_y as u16)
    }

    fn is_i32_in_width(&'a self, pos_x: i32) -> bool {
        pos_x >= 0 && self.is_in_width(pos_x as u16)
    }

    fn is_in_height(&'a self, pos_y: u16) -> bool {
        pos_y < self.height()
    }

    fn is_in_width(&'a self, pos_x: u16) -> bool {
        pos_x < self.width()
    }
}
