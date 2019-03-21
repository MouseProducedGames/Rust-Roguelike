/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

 **/
// External dependencies

// Internal dependencies.
use crate::tiled_shapes_2d::TiledShape2D;

pub struct TiledRect {
    left: u32,
    top: u32,
    right: u32,
    bottom: u32,
}

impl TiledRect {
    pub fn with_absolute_bounds(left: u32, top: u32, right: u32, bottom: u32) -> Self {
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

    pub fn bottom(&self) -> u32 {
        self.bottom
    }

    pub fn height(&self) -> u32 {
        (self.bottom - self.top) + 1
    }

    pub fn left(&self) -> u32 {
        self.left
    }

    pub fn right(&self) -> u32 {
        self.right
    }

    pub fn top(&self) -> u32 {
        self.top
    }

    pub fn width(&self) -> u32 {
        (self.right - self.left) + 1
    }
}

impl TiledShape2D for TiledRect {
    fn bottom(&self) -> u32 {
        self.bottom()
    }

    fn circumference(&self) -> u32 {
        // Should be optimized: TWo gets, two adds, and a return.
        let half = self.height() + self.width();
        half + half
    }

    fn iter_circumference(&self, iter_index: &mut u32) -> Option<(u32, u32)> {
        let (width, height) = (self.width(), self.height());
        let index = *iter_index;
        *iter_index += 1;
        if index < width {
            Some((self.left + index, self.top))
        } else if index < (width + height) {
            Some((self.right, self.top + (index - width)))
        } else if index < (width + height + width) {
            let temp = index - (width + height);
            Some((self.left + temp, self.bottom))
        } else if index < (width + height + width + height) {
            let temp = index - (width + height + width);
            Some((self.left, self.top + temp))
        } else {
            None
        }
    }

    fn iter_surface_area(&self, iter_index: &mut u32) -> Option<(u32, u32)> {
        let (width, height) = (self.width(), self.height());
        let index = *iter_index;
        *iter_index += 1;
        let x = index % width;
        let y = index / width;
        if y == height {
            return None;
        }

        Some(((self.left + x), (self.top + y)))
    }

    fn left(&self) -> u32 {
        self.left()
    }

    fn right(&self) -> u32 {
        self.right()
    }

    fn surface_area(&self) -> u32 {
        self.height() * self.width()
    }

    fn top(&self) -> u32 {
        self.top()
    }
}
