/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

 **/

// External includes.

// Internal includes.
use crate::tiled_shapes_2d::TiledRect;
use crate::world::Tilemap ;
use super::FillTileShape;

pub trait FillTile
{ 
    fn fill_tile( &mut self, tile_type: u32 ) -> &mut Tilemap;
}

impl FillTile for Tilemap
{
    fn fill_tile( &mut self, tile_type: u32 ) -> &mut Tilemap
    {
        self.fill_tile_shape(
            &TiledRect::with_absolute_bounds( 0, 0, self.width() as u32, self.height() as u32 ),
            tile_type
        )
    }
}
