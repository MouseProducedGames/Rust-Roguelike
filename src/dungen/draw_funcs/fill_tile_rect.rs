/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

 **/

// External includes.

// Internal includes.
use crate::world::Tilemap ;

pub trait FillTileRect
{ 
    fn fill_tile_rect(
        &mut self,
        left: usize, top: usize,
        width: usize, height: usize,
        tile_type: u32
    ) -> &mut Tilemap;
}

impl FillTileRect for Tilemap
{
    fn fill_tile_rect(
        &mut self,
        left: usize, top: usize,
        width: usize, height: usize,
        tile_type: u32
    ) -> &mut Tilemap
    {
        let ( right, bottom ) = ( left + width, top + height );
        
        for y in top..bottom
        {
            for x in left..right
            {
                *self.tile_type_mut( x, y ) = tile_type;
            }
        }
        
        self
    }
}
