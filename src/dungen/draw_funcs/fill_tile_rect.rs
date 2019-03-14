/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

 **/

// External includes.

// Internal includes.
use std::iter::Iterator;
// use crate::tiled_shapes_2d::TiledShape2DIterator;
use crate::world::Tilemap ;

pub trait FillTileShape
{ 
    fn fill_tile_shape(
        &mut self,
        shape: &mut Iterator< Item = ( u32, u32 )>,
        tile_type: u32
    ) -> &mut Tilemap;
}

impl FillTileShape for Tilemap
{
    fn fill_tile_shape(
        &mut self,
        shape: &mut Iterator< Item = ( u32, u32 )>,
        tile_type: u32
    ) -> &mut Tilemap
    {
        for ( x, y ) in shape
        {
            *self.tile_type_mut( x as usize, y as usize ) = tile_type;
        }
        
        self
    }
}
