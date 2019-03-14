/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/

// External dependencies

// Internal dependencies.
use std::iter::Iterator;

pub trait TiledShapeDef2D
{
    fn circumference( &self ) -> u32;
    
    fn surface_area( &self ) -> u32;

    fn iter_next( &mut self, iter_index: &mut ( u32, u32 ) ) -> Option< ( u32, u32) >;
}

pub struct TiledShape2DIterator<'a>
{
    shape: &'a mut TiledShapeDef2D,
    iter_index: ( u32, u32 ),
}

impl<'a> TiledShape2DIterator<'a>
{
    pub fn new( shape: &'a mut TiledShapeDef2D ) -> Self
    {
        Self { shape: shape, iter_index: ( 0, 0 ) }
    }

    fn iter_next( &mut self ) -> Option< ( u32, u32 ) >
    {
        let mut iter_index = self.iter_index;
        let output = self.shape.iter_next( &mut iter_index );
        self.iter_index = iter_index;
        output
    }
}

impl<'a> Iterator for TiledShape2DIterator<'a>
{
    type Item = ( u32, u32 );

    fn next( &mut self ) -> Option< ( Self::Item ) >
    {
        self.iter_next()
    }
}
