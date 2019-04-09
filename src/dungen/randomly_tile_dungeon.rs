/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

 **/
// External includes.

// Standard includes.

// Internal includes.
use super::DungeonGenerator;
use crate::dungen::draw_funcs::{DrawTileShape, FillTileShape, FillTileShapeRandRange};
use crate::rrl_math::{Bounds, Position};
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
    fn apply(
        &mut self,
        area: &mut dyn TiledArea,
        generation_areas: &mut Vec<(Position, Position)>,
    ) {
        FillTileShape::new(2).apply(area, generation_areas);
        DrawTileShape::new(1).apply(area, generation_areas);
        let (width, height) = (area.width(), area.height());
        let mut rect = TiledRect::with_absolute_bounds(1, 1, width - 1, height - 1);
        let mut filter_area = TiledAreaFilter::new(area, &mut rect);
        FillTileShapeRandRange::new(self.start_range, self.end_range)
            .apply(&mut filter_area, generation_areas);

        let top_left = Position::new(1, 1);
        let top_left = area.get_global_position_from_local_position(top_left);
        let bottom_right = Position::new(i32::from(area.width() - 1), i32::from(area.height() - 1));
        let bottom_right = area.get_global_position_from_local_position(bottom_right);
        generation_areas.push((top_left, bottom_right));

        /* for pos in area.get_position(0, 0) {
            creature_factory(Position::new(i32::from(pos.x()), i32::from(pos.y())));
        } */
    }
}
