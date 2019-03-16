/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

 **/

// External includes.

// Internal includes.
use crate::world::TiledArea;
use super::FillTileShape;

pub trait FillTile
{ 
    fn fill_tile( mut self, tile_type: u32 ) ->  Box<dyn TiledArea>;
}

impl FillTile for Box<dyn TiledArea>
{
    fn fill_tile( mut self, tile_type: u32 ) -> Box<dyn TiledArea>
    {
        self.fill_tile_shape( tile_type )
    }
}
