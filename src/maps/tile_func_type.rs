/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.

// Standard includes.

// Internal includes.
use super::{Mapping, Tilemap, VisibilityType};
use crate::dice::roll_success;
use crate::rrl_math::Position;

pub static TILE_FUNC_DATA: [TileFunc; 4] = [
    TileFunc::None,
    TileFunc::OnEnterTile(TileFuncOp::ChangeTileType(4, TILE_FUNC_INDEX_VOID)),
    TileFunc::OnEnterTile(TileFuncOp::DiscoverTileType(
        6,
        TILE_FUNC_INDEX_DISCOVERED_SECRET_DOOR,
    )),
    TileFunc::OnEnterTile(TileFuncOp::ChangeTileType(7, TILE_FUNC_INDEX_VOID)),
];

pub static TILE_FUNC_INDEX_VOID: u32 = 0;
pub static TILE_FUNC_INDEX_DOOR: u32 = 1;
pub static TILE_FUNC_INDEX_SECRET_DOOR: u32 = 2;
pub static TILE_FUNC_INDEX_DISCOVERED_SECRET_DOOR: u32 = 3;

/* pub struct TileFunc
{
    pub trigger: TileFuncTrigger,
    pub op: TileFuncOp,
} */

#[derive(Copy, Clone)]
pub enum TileFunc {
    None,
    OnEnterTile(TileFuncOp),
}

#[derive(Copy, Clone)]
pub enum TileFuncOp {
    ChangeTileType(u32, u32),
    DiscoverTileType(u32, u32),
}

pub fn execute_tile_func(
    harmless: bool,
    skill_bonus: i32,
    map: &mut Tilemap,
    visibility_type: VisibilityType,
    pos: Position,
) {
    let map_pos = map.get_position(pos.x as u16, pos.y as u16).unwrap();
    match map.tile_func_pos(pos) {
        TileFunc::None => (),
        TileFunc::OnEnterTile(tile_func_op) => match tile_func_op {
            TileFuncOp::ChangeTileType(new_tile_type, new_tile_func_type) => {
                if harmless == false && roll_success(skill_bonus).is_success() {
                    *map.tile_type_mut(map_pos) = new_tile_type;
                    *map.tile_func_type_mut(map_pos) = new_tile_func_type;
                }
            }
            TileFuncOp::DiscoverTileType(new_tile_type, new_tile_func_type) => {
                if visibility_type == VisibilityType::Visible
                    && roll_success(skill_bonus).is_success()
                {
                    *map.tile_type_mut(map_pos) = new_tile_type;
                    *map.tile_func_type_mut(map_pos) = new_tile_func_type;
                }
            }
        },
    }
}
