/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/

// External includes

// Internal includes

pub static TILE_FUNC_DATA: [TileFunc; 4] = [
    TileFunc::None,
    TileFunc::OnEnterTile(TileFuncOp::ChangeTileType(4, TILE_FUNC_INDEX_VOID)),
    TileFunc::OnEnterTile(TileFuncOp::DiscoverTileType(6, TILE_FUNC_INDEX_DISCOVERED_SECRET_DOOR)),
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
pub enum TileFunc
{
    None,
    OnEnterTile(TileFuncOp),
}

#[derive(Copy, Clone)]
pub enum TileFuncOp
{
    ChangeTileType(u32, u32),
    DiscoverTileType(u32, u32),
}
