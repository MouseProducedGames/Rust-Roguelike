/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

 **/
// External includes.
use rand::{thread_rng, Rng};

// Standard includes.

// Internal includes.
use super::DungeonGenerator;
use crate::dungen::draw_funcs::{DrawTileShape, FillTileShape};
use crate::rrl_math::{Bounds, Position};
use crate::tiled_shapes_2d::TiledRect;
use crate::world::{
    MapPosition, Mapping, TiledArea, TiledAreaFilter, TILE_TYPE_INDEX_DOOR, TILE_TYPE_INDEX_FLOOR,
    TILE_TYPE_INDEX_WALL,
};

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum SplitType {
    LongestDimension,
    _Random,
}

pub struct SplitDungeon {
    split_type: SplitType,
    min_bounds: Bounds,
}

impl SplitDungeon {
    pub fn new(split_type: SplitType, min_bounds: Bounds) -> Self {
        Self {
            split_type,
            min_bounds,
        }
    }
}

impl DungeonGenerator for SplitDungeon {
    fn apply(
        &mut self,
        area: &mut dyn TiledArea,
        generation_areas: &mut Vec<(Position, Position)>,
    ) {
        let (left, top, right, bottom) = (area.left(), area.top(), area.right(), area.bottom());
        let (width, height) = (area.width(), area.height());

        if ((width > self.min_bounds.width) && (height > self.min_bounds.height))
            || ((self.split_type == SplitType::LongestDimension)
                && ((width > self.min_bounds.width) || (height > self.min_bounds.height)))
        {
        } else {
            let top_left = Position::new(1, 1);
            let top_left = area.get_global_position_from_local_position(top_left);
            let bottom_right =
                Position::new(i32::from(area.width() - 1), i32::from(area.height() - 1));
            let bottom_right = area.get_global_position_from_local_position(bottom_right);
            generation_areas.push((top_left, bottom_right));

            /* for pos in area.get_position(0, 0) {
                let create_positon = Position::new(i32::from(pos.x()), i32::from(pos.y()));
                creature_factory(create_positon);
            } */

            return;
        }

        let split_width;
        // let split_on;
        match self.split_type {
            SplitType::LongestDimension => {
                if width > height {
                    split_width = true;
                } else if height > width {
                    split_width = false;
                } else {
                    split_width = thread_rng().gen_bool(0.5);
                }
            }
            SplitType::_Random => {
                split_width = thread_rng().gen_bool(0.5);
            }
        }

        let split_min;
        let split_max;
        let put_door: MapPosition;
        if split_width {
            split_min = self.min_bounds.width;
            split_max = width - self.min_bounds.width;
        } else {
            split_min = self.min_bounds.height;
            split_max = height - self.min_bounds.height;
        }

        let split_on;
        if split_max == split_min {
            split_on = split_min;
        } else if split_max < split_min {
            let top_left = Position::new(1, 1);
            let top_left = area.get_global_position_from_local_position(top_left);
            let bottom_right =
                Position::new(i32::from(area.width() - 1), i32::from(area.height() - 1));
            let bottom_right = area.get_global_position_from_local_position(bottom_right);
            generation_areas.push((top_left, bottom_right));

            /* for pos in area.get_position(0, 0) {
                let create_positon = Position::new(i32::from(pos.x()), i32::from(pos.y()));
                creature_factory(create_positon);
            } */

            return;
        } else {
            split_on = thread_rng().gen_range(split_min, split_max);
        }

        let mut split_line;
        let (room_left0, room_top0, room_right0, room_bottom0);
        let (room_left1, room_top1, room_right1, room_bottom1);
        if split_width {
            split_line =
                TiledRect::with_absolute_bounds(left + split_on, top, left + split_on, height - 1);
            put_door = area
                .get_position(split_on, thread_rng().gen_range(1, height - 2))
                .unwrap();
            room_left0 = left;
            room_top0 = top;
            room_right0 = left + split_on;
            room_bottom0 = bottom;
            room_left1 = left + split_on;
            room_top1 = top;
            room_right1 = right;
            room_bottom1 = bottom;
        } else {
            split_line =
                TiledRect::with_absolute_bounds(left, top + split_on, width - 1, top + split_on);
            put_door = area
                .get_position(thread_rng().gen_range(1, width - 2), split_on)
                .unwrap();
            room_left0 = left;
            room_top0 = top;
            room_right0 = right;
            room_bottom0 = top + split_on;
            room_left1 = left;
            room_top1 = top + split_on;
            room_right1 = right;
            room_bottom1 = bottom;
        }

        {
            FillTileShape::new(TILE_TYPE_INDEX_FLOOR).apply(area, generation_areas);
            DrawTileShape::new(TILE_TYPE_INDEX_WALL).apply(area, generation_areas);

            let mut temp_area = TiledAreaFilter::new(area, &mut split_line);
            FillTileShape::new(TILE_TYPE_INDEX_WALL).apply(&mut temp_area, generation_areas);
        }

        {
            let mut rect =
                TiledRect::with_absolute_bounds(room_left0, room_top0, room_right0, room_bottom0);
            let mut temp_area = TiledAreaFilter::new(area, &mut rect);
            SplitDungeon::new(self.split_type, self.min_bounds)
                .apply(&mut temp_area, generation_areas);
        }

        {
            let mut rect =
                TiledRect::with_absolute_bounds(room_left1, room_top1, room_right1, room_bottom1);
            let mut temp_area = TiledAreaFilter::new(area, &mut rect);
            SplitDungeon::new(self.split_type, self.min_bounds)
                .apply(&mut temp_area, generation_areas);
        }
        {
            *area.tile_type_mut(put_door) = TILE_TYPE_INDEX_DOOR;
        }
    }
}
