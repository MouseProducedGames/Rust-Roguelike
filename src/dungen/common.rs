/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

 **/

// External includes.

// Internal includes.
use crate::world::Tilemap ;

pub trait DungenCommon
{ 
    fn create_new( width: usize, height: usize ) -> Tilemap
    {
        Tilemap::new( width, height )
    }
    
    fn finish( &mut self ) -> Tilemap;
}

impl DungenCommon for Tilemap
{
    fn finish( &mut self ) -> Tilemap
    {
        let mut output = Tilemap::new( self.width(), self.height() );
        for y in 0..self.height()
        {
            for x in 0..self.width()
            {
                *output.tile_type_mut( x, y ) = self.tile_type( x, y );
            }
        }

        output
    }
}
