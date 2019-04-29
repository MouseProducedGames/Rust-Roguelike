/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.

// Standard includes.

// Internal includes.
mod calculate_light;
mod lightmap;
mod line_of_sight;
mod map_displacement;
mod map_position;
mod map_processor;
mod map_scan_position;
mod mapping;
mod multimap;
mod pattern_matching;
mod tile_func_type;
mod tile_type;
mod tilemap;
mod tileroom;
mod visibility_map;
mod visibility_map_lookup;
mod visibility_system;
pub use calculate_light::calculate_light_level;
pub use lightmap::Lightmap;
pub use line_of_sight::calculate_visibility;
pub use map_displacement::MapDisplacement;
pub use map_position::MapPosition;
pub use map_processor::MapProcessor;
pub use map_scan_position::MapScanPosition;
pub use mapping::Mapping;
pub use multimap::Multimap;
pub use pattern_matching::{match_pattern, PatternFlags, PatternLookup};
pub use tile_func_type::{
    execute_tile_func, TileFunc, TileFuncOp, TILE_FUNC_DATA, TILE_FUNC_INDEX_DOOR,
    TILE_FUNC_INDEX_SECRET_DOOR, TILE_FUNC_INDEX_VOID,
};
pub use tile_type::{
    TileTypeData, TILE_TYPE_DATA, TILE_TYPE_INDEX_DOOR, TILE_TYPE_INDEX_FLOOR,
    TILE_TYPE_INDEX_VOID, TILE_TYPE_INDEX_WALL,
};
pub use tilemap::Tilemap;
pub use tileroom::{TiledArea, TiledAreaFilter};
pub use visibility_map::{VisibilityMap, VisibilityType};
pub use visibility_map_lookup::VisibilityMapLookup;
pub use visibility_system::VisibilitySystem;
