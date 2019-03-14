/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

 **/

// External includes.

// Internal includes.
use crate::world::Tilemap ;

pub trait DrawTileRect
{ 
    fn draw_tile_rect(
        &mut self,
        left: usize, top: usize,
        width: usize, height: usize,
        tile_type: u32
    ) -> &mut Tilemap;
}

impl DrawTileRect for Tilemap
{
    fn draw_tile_rect(
        &mut self,
        left: usize, top: usize,
        width: usize, height: usize,
        tile_type: u32
    ) -> &mut Tilemap
    {
        let ( right, bottom ) = ( left + width, top + height );

        {
            for x in left..right
            {
                *self.tile_type_mut( x, top ) = tile_type;
                // Range is exclusive on bottom.
                *self.tile_type_mut( x, bottom - 1 ) = tile_type;
            }
        }

        {
            for y in ( top + 1 )..( bottom - 1 )
            {
                *self.tile_type_mut( left, y ) = tile_type;
                // Range is exclusive on right.
                *self.tile_type_mut( right - 1, y ) = tile_type;
            }
        }

        self
    }
}
