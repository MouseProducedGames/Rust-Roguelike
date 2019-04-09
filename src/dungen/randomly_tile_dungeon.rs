/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

 **/
// External includes.

// Standard includes.
use std::cell::RefCell;
use std::rc::Rc;

// Internal includes.
use super::DungeonGenerator;
use crate::dungen::draw_funcs::{DrawTileShape, FillTileShape, FillTileShapeRandRange};
use crate::tiled_shapes_2d::TiledRect;
use crate::world::{Mapping, TiledArea, TiledAreaFilter};

pub struct _RandomlyTileDungeon {
    start_range: u32,
    end_range: u32,
}

impl _RandomlyTileDungeon {
    pub fn _new(start_range: u32, end_range: u32) -> Self {
        Self {
            start_range,
            end_range,
        }
    }
}

impl DungeonGenerator for _RandomlyTileDungeon {
    fn apply(&mut self, area: &mut dyn TiledArea)
    {
        FillTileShape::new(2).apply(area);
        DrawTileShape::new(1).apply(area);
        let (width, height) = (area.width(), area.height());
        let mut rect = TiledRect::with_absolute_bounds(1, 1, width - 1, height - 1);
        let mut filter_area =
            TiledAreaFilter::new(
                area,
                &mut rect,
            );
        FillTileShapeRandRange::new(self.start_range, self.end_range).apply(&mut filter_area);
    }
}
