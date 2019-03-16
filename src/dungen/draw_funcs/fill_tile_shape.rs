/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

 **/

// External includes.

// Internal includes.
use crate::tiled_shapes_2d::{ TiledShape2DSurfaceAreaIterator };
use crate::world::TiledArea ;

pub trait FillTileShape
{ 
    fn fill_tile_shape(
        mut self,
        tile_type: u32
    ) -> Box<dyn TiledArea>;
}

impl FillTileShape for Box<dyn TiledArea>
{
    fn fill_tile_shape(
        mut self,
        tile_type: u32
    ) -> Box<dyn TiledArea>
    {
        let mut iter_index: u32 = 0;
        let mut keep_going: bool = true;
        while keep_going
        {
            let ( x, y );
            match self.iter_surface_area( &mut iter_index ) {
                Some( ( it_x, it_y ) ) => { x = it_x; y = it_y },
                _ => { keep_going = false; continue; }
            }
            *self.tile_type_mut( x, y ) = tile_type;
        }

        self

        /* for ( x, y ) in TiledShape2DSurfaceAreaIterator::new( &self )
        {
            *self.tile_type_mut( x, y ) = tile_type;
        }
        
        self */
    }
}
