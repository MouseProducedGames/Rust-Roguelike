/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

 **/

// External includes.
extern crate rand;
use rand::rngs::ThreadRng;

// Internal includes.
use crate::dungen::draw_funcs::{ DrawTileShape, FillTile, FillTileShapeRandRange };
use crate::tiled_shapes_2d::TiledRect;
use crate::world::{ Mapping, Tilemap };

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
            .draw_tile_shape( &TiledRect::with_absolute_bounds( 0, 0, width as u32, height as u32 ), 1 )
            .fill_tile_shape_rand_range(
                &TiledRect::with_absolute_bounds( 1, 1, ( width - 1 ) as u32, ( height - 1 ) as u32 ),
                start_range, end_range,
                rnd
            )
    }
}
