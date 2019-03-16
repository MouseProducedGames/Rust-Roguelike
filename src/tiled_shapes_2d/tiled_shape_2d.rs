/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/

// External dependencies

// Internal dependencies.
use std::iter::Iterator;

pub trait TiledShape2D
{
    fn bottom( &self ) -> u32;
    
    fn circumference( &self ) -> u32;
    
    fn iter_circumference( &self, iter_index: &mut u32 ) -> Option< ( u32, u32) >;
    
    fn iter_surface_area( &self, iter_index: &mut u32 ) -> Option< ( u32, u32) >;

    fn left( &self ) -> u32;
    
    fn right( &self ) -> u32;
    
    fn surface_area( &self ) -> u32;

    fn top( &self ) -> u32;
}

pub struct TiledShape2DCircumferenceIterator<'a>
{
    shape: &'a TiledShape2D,
    iter_index: u32,
}

impl<'a> TiledShape2DCircumferenceIterator<'a>
{
    pub fn new( shape: &'a TiledShape2D ) -> Self
    {
        Self { shape, iter_index: 0 }
    }

    fn iter_next( &mut self ) -> Option< ( u32, u32 ) >
    {
        let mut iter_index = self.iter_index;
        let output = self.shape.iter_circumference( &mut iter_index );
        self.iter_index = iter_index;
        output
    }
}

impl<'a> Iterator for TiledShape2DCircumferenceIterator<'a>
{
    type Item = ( u32, u32 );

    fn next( &mut self ) -> Option< ( Self::Item ) >
    {
        self.iter_next()
    }
}

pub struct TiledShape2DSurfaceAreaIterator<'a>
{
    shape: &'a TiledShape2D,
   iter_index: u32,
}

impl<'a> TiledShape2DSurfaceAreaIterator<'a>
{
    pub fn new( shape: &'a TiledShape2D ) -> Self
    {
        Self { shape, iter_index: 0 }
    }

    fn iter_next( &mut self ) -> Option< ( u32, u32 ) >
    {
        let mut iter_index = self.iter_index;
        let output = self.shape.iter_surface_area( &mut iter_index );
        self.iter_index = iter_index;
        output
    }
}

impl<'a> Iterator for TiledShape2DSurfaceAreaIterator<'a>
{
    type Item = ( u32, u32 );

    fn next( &mut self ) -> Option< ( Self::Item ) >
    {
        self.iter_next()
    }
}
