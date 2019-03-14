/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

 **/

// External includes.
extern crate rand;
use rand::Rng;
use rand::rngs::ThreadRng;

// Internal includes.
use crate::tiled_shapes_2d::{ TiledShapeDef2D, TiledShape2DSurfaceAreaIterator };
use crate::world::Tilemap ;

pub trait FillTileShapeRandRange
{ 
    fn fill_tile_shape_rand_range(
        &mut self,
        shape: &TiledShapeDef2D,
        start_range: u32, end_range: u32,
        rnd: &mut ThreadRng
    ) -> &mut Tilemap;
}

impl FillTileShapeRandRange for Tilemap
{
    fn fill_tile_shape_rand_range(
        &mut self,
        shape: &TiledShapeDef2D,
        start_range: u32, end_range: u32,
        rnd: &mut ThreadRng
    ) -> &mut Tilemap
    {
        for ( x, y ) in TiledShape2DSurfaceAreaIterator::new( shape )
        {
            *self.tile_type_mut( x as usize, y as usize ) = rnd.gen_range( start_range, end_range );
        }
        
        self
    }
}
