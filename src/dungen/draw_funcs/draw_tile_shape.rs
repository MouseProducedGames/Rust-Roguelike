/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

 **/

// External includes.

// Internal includes.
use crate::world::Tilemap ;
use crate::tiled_shapes_2d::{ TiledShape2DCircumferenceIterator, TiledShape2D };

pub trait DrawTileShape
{ 
    fn draw_tile_shape(
        &mut self,
        shape: &TiledShape2D,
        tile_type: u32
    ) -> &mut Tilemap;
}

impl DrawTileShape for Tilemap
{
    fn draw_tile_shape(
        &mut self,
        shape: &TiledShape2D,
        tile_type: u32
    ) -> &mut Tilemap
    {
        for ( x, y ) in TiledShape2DCircumferenceIterator::new( shape )
        {
            *self.tile_type_mut( x as usize, y as usize ) = tile_type;
        }

        self
    }
}
