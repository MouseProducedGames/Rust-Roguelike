/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

 **/

// External includes

// internal include
use crate::stats::Stat;

pub trait Attribute : Stat
{
    fn modifier( &self ) -> i32
    {
        ( self.value() / 2 ) - 5
    }

/*     fn value( &self ) -> i32;

    fn value_mut( &mut self ) -> &mut i32; */
}

/* impl Attribute for Stat
{
    fn modifier( &self ) -> i32
    {
        ( self.value() / 2 ) - 5
    }

    fn value( &self ) -> i32
    {
        Stat::value( self )
    }

    fn value_mut( &mut self ) -> &mut i32
    {
        Stat::value_mut( self )
    }
}
*/
