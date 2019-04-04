/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

 **/
// External dependencies

// Internal dependencies.
use crate::tiled_shapes_2d::TiledShape2D;
use crate::world::{MapPosition, Mapping};

pub struct TiledRect {
    left: u16,
    top: u16,
    right: u16,
    bottom: u16,
}

impl TiledRect {
    pub fn with_absolute_bounds(left: u16, top: u16, right: u16, bottom: u16) -> Self {
        let (use_left, use_right, use_top, use_bottom);
        if left > right {
            use_right = left;
            use_left = right;
        } else {
            use_left = left;
            use_right = right;
        }

        if top > bottom {
            use_top = bottom;
            use_bottom = top;
        } else {
            use_top = top;
            use_bottom = bottom;
        }

        Self {
            left: use_left,
            top: use_top,
            right: use_right,
            bottom: use_bottom,
        }
    }

    pub fn bottom(&self) -> u16 {
        self.bottom
    }

    pub fn height(&self) -> u16 {
        (self.bottom - self.top) + 1
    }

    pub fn left(&self) -> u16 {
        self.left
    }

    pub fn right(&self) -> u16 {
        self.right
    }

    pub fn top(&self) -> u16 {
        self.top
    }

    pub fn width(&self) -> u16 {
        (self.right - self.left) + 1
    }
}

impl Mapping for TiledRect {
    fn height(&self) -> u16 {
        TiledRect::height(self)
    }

    fn width(&self) -> u16 {
        TiledRect::width(self)
    }
}

impl TiledShape2D for TiledRect {
    fn bottom(&self) -> u16 {
        self.bottom()
    }

    fn circumference(&self) -> u16 {
        // Should be optimized: TWo gets, two adds, and a return.
        let half = self.height() + self.width();
        half + half
    }

    fn iter_circumference(&self, iter_index: &mut u32) -> Option<MapPosition> {
        let (width, height) = (self.width() as u32, self.height() as u32);
        let index = *iter_index;
        *iter_index += 1;
        if index < width {
            Some(self.get_position(index as u16, 0).unwrap())
        } else if index < (width + height) {
            let temp = (index - width) as u16;
            Some(self.get_position(self.width() - 1, temp).unwrap())
        } else if index < (width + height + width) {
            let temp = (index - (width + height)) as u16;
            Some(self.get_position(temp, self.height() - 1).unwrap())
        } else if index < (width + height + width + height) {
            let temp = (index - (width + height + width)) as u16;
            Some(self.get_position(0, temp).unwrap())
        } else {
            None
        }
    }

    fn iter_surface_area(&self, iter_index: &mut u32) -> Option<MapPosition> {
        let (width, height) = (self.width() as u32, self.height() as u32);
        let index = *iter_index;
        *iter_index += 1;
        let x = (index % width) as u16;
        let y = (index / width) as u16;
        if y == (height as u16) {
            return None;
        }

        Some(self.get_position(x, y).unwrap())
    }

    fn left(&self) -> u16 {
        self.left()
    }

    fn right(&self) -> u16 {
        self.right()
    }

    fn surface_area(&self) -> u16 {
        self.height() * self.width()
    }

    fn top(&self) -> u16 {
        self.top()
    }
}
