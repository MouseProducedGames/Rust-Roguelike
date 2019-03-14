/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

 **/

// External includes.
extern crate rand;
use rand::rngs::ThreadRng;

// Internal includes.
use crate::world::{ Mapping, Tilemap };
use crate::dungen::draw_funcs::{ DrawTileRect, FillTile, FillTileRectRandRange };

pub trait RandomlyTileDungeon
{
    fn randomly_tile_dungeon(
        &mut self,
        start_range: u32, end_range: u32,
        rnd: &mut ThreadRng
    ) -> &mut Tilemap;
}

impl RandomlyTileDungeon for Tilemap
{
    fn randomly_tile_dungeon(
        &mut self,
        start_range: u32, end_range: u32,
        rnd: &mut ThreadRng
    ) -> &mut Tilemap
    {
        let ( width, height ) = self.bounds();
        self
            .fill_tile( 2 )
            .draw_tile_rect( 0, 0, width, height, 1 )
            .fill_tile_rect_rand_range(
                1, 1,
                width - 2, height - 2,
                1, 3,
                rnd
            )
    }
}
