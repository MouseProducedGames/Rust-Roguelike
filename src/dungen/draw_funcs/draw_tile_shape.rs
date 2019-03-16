/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

 **/

// External includes.

// Internal includes.
use crate::tiled_shapes_2d::{ TiledShape2DCircumferenceIterator };
use crate::world::TiledArea;

pub trait DrawTileShape
{ 
    fn draw_tile_shape(
        mut self,
        tile_type: u32
    ) -> Box<dyn TiledArea>;
}

impl DrawTileShape for Box<dyn TiledArea>
{
    fn draw_tile_shape(
        mut self,
        tile_type: u32
    ) -> Box<dyn TiledArea>
    {
        // let temp: &mut TiledShape2D = self;
        // for ( x, y ) in TiledShape2DCircumferenceIterator::new( self )
        let mut iter_index: u32 = 0;
        let mut keep_going: bool = true;
        while keep_going
        {
            let ( x, y );
            match self.iter_circumference( &mut iter_index ) {
                Some( ( it_x, it_y ) ) => { x = it_x; y = it_y },
                _ => { keep_going = false; continue; }
            }
            *self.tile_type_mut( x, y ) = tile_type;
        }

        self
    }
}
