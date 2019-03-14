/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

 **/

// External includes.
extern crate rand;
use rand::Rng;
use rand::rngs::ThreadRng;

// Internal includes.
use crate::world::Tilemap ;

pub trait FillTileRectRandRange
{ 
    fn fill_tile_rect_rand_range(
        &mut self,
        left: usize, top: usize,
        width: usize, height: usize,
        start_range: u32, end_range: u32,
        rnd: &mut ThreadRng
    ) -> &mut Tilemap;
}

impl FillTileRectRandRange for Tilemap
{
    fn fill_tile_rect_rand_range(
        &mut self,
        left: usize, top: usize,
        width: usize, height: usize,
        start_range: u32, end_range: u32,
        rnd: &mut ThreadRng
    ) -> &mut Tilemap
    {
        let ( right, bottom ) = ( left + width, top + height );
        
        for y in top..bottom
        {
            for x in left..right
            {
                *self.tile_type_mut( x, y ) = rnd.gen_range( start_range, end_range );
            }
        }
        
        self
    }
}
