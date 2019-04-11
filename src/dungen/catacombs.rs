/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

 **/
// External includes.

// Standard includes.

// Internal includes.
use super::DungeonGenerator;
use crate::rrl_math::{Displacement, Position};
use crate::world::{
    Mapping, TiledArea, TILE_TYPE_INDEX_DOOR, TILE_TYPE_INDEX_FLOOR, TILE_TYPE_INDEX_WALL,
};
pub struct Catacombs {}

impl Catacombs {
    pub fn new() -> Self {
        Self {}
    }
}

impl DungeonGenerator for Catacombs {
    fn apply(
        &mut self,
        area: &mut dyn TiledArea,
        generation_areas: &mut Vec<(Position, Position)>,
    ) {
        let top_left = Position::new(1, 1);
        let top_left = area.get_global_position_from_local_position(top_left);
        let bottom_right = Position::new(i32::from(area.width() - 1), i32::from(area.height() - 1));
        let bottom_right = area.get_global_position_from_local_position(bottom_right);
        generation_areas.push((top_left, bottom_right));

        for position in area.get_position(0, 0) {
            *area.tile_type_mut(position) = TILE_TYPE_INDEX_WALL;
        }

        let mut expansion_list: Vec<Position> = vec![];
        expansion_list.push(Position::new(1, 1));
        *area.tile_type_mut(area.get_position(1, 1).unwrap()) = TILE_TYPE_INDEX_FLOOR;

        fn do_the_thing(
            area: &mut dyn TiledArea,
            expansion_list: &mut Vec<Position>,
            current_position: Position,
            move_pos: Displacement,
        ) -> bool {
            let check_position = current_position + move_pos;
            let (x, y) = (check_position.x as u16, check_position.y as u16);
            let map_scan_pos = area.get_position(x, y);
            if map_scan_pos == true {
                let map_pos = map_scan_pos.unwrap();
                if (map_pos.x() < (area.right() - 1))
                    && (map_pos.y() < (area.bottom() - 1))
                    && area.tile_type(map_pos) == TILE_TYPE_INDEX_WALL
                {
                    *area.tile_type_mut(map_pos) = TILE_TYPE_INDEX_FLOOR;

                    expansion_list.push(check_position);

                    let entrance_move = move_pos / 2;
                    let entrance_position = current_position + entrance_move;
                    let (x, y) = (entrance_position.x as u16, entrance_position.y as u16);
                    let map_pos = area.get_position(x, y).unwrap();
                    *area.tile_type_mut(map_pos) = TILE_TYPE_INDEX_DOOR;

                    true
                } else {
                    false
                }
            } else {
                false
            }
        }

        while expansion_list.is_empty() == false {
            let current_position = expansion_list[expansion_list.len() - 1];
            let mut found = false;
            for y_move in (-2_i32..=2_i32).step_by(2) {
                for x_move in (-2_i32..=2_i32).step_by(2) {
                    if ((x_move != 0) || (y_move != 0)) && x_move.abs() != y_move.abs() && do_the_thing(
                        area,
                        &mut expansion_list,
                        current_position,
                        Displacement::new(x_move, y_move),
                    ) {
                        found = true;
                        break;
                    }
                }
            }

            if found == false {
                expansion_list.pop();
            }
        }
    }
}
