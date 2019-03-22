/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/

// External includes

// Internal includes

pub static TILE_FUNC_DATA: [TileFunc; 2] = [
    TileFunc::None,
    TileFunc::OnEnterTile(TileFuncOp::ChangeTileType(4))
];

pub static TILE_FUNC_INDEX_VOID: u32 = 0;
pub static TILE_FUNC_INDEX_DOOR: u32 = 1;

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
    ChangeTileType(u32),
}
