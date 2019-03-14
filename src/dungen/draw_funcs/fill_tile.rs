/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

 **/

// External includes.

// Internal includes.
use crate::world::Tilemap ;
use super::FillTileRect;

pub trait FillTile
{ 
    fn fill_tile( &mut self, tile_type: u32 ) -> &mut Tilemap;
}

impl FillTile for Tilemap
{
    fn fill_tile( &mut self, tile_type: u32 ) -> &mut Tilemap
    {
        self.fill_tile_rect( 0, 0, self.width(), self.height(), tile_type )
    }
}
