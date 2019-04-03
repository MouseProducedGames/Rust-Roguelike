/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/

// External dependencies

// Internal dependencies.
use crate::rrl_math::MapPosition;

pub trait TiledShape2D {
    fn bottom(&self) -> u16;

    fn circumference(&self) -> u16;

    fn iter_circumference(&self, iter_index: &mut u32) -> Option<(MapPosition)>;

    fn iter_surface_area(&self, iter_index: &mut u32) -> Option<(MapPosition)>;

    fn left(&self) -> u16;

    fn right(&self) -> u16;

    fn surface_area(&self) -> u16;

    fn top(&self) -> u16;
}

/* pub struct TiledShape2DCircumferenceIterator<'a>
{
    shape: &'a TiledShape2D,
    iter_index: u16,
}

impl<'a> TiledShape2DCircumferenceIterator<'a>
{
    pub fn new( shape: &'a TiledShape2D ) -> Self
    {
        Self { shape, iter_index: 0 }
    }

    fn iter_next( &mut self ) -> Option< MapPosition >
    {
        let mut iter_index = self.iter_index;
        let output = self.shape.iter_circumference( &mut iter_index );
        self.iter_index = iter_index;
        output
    }
}

impl<'a> Iterator for TiledShape2DCircumferenceIterator<'a>
{
    type Item = MapPosition;

    fn next( &mut self ) -> Option< Self::Item >
    {
        self.iter_next()
    }
} */

/* pub struct TiledShape2DSurfaceAreaIterator<'a>
{
    shape: &'a TiledShape2D,
   iter_index: u16,
}

impl<'a> TiledShape2DSurfaceAreaIterator<'a>
{
    pub fn new( shape: &'a TiledShape2D ) -> Self
    {
        Self { shape, iter_index: 0 }
    }

    fn iter_next( &mut self ) -> Option< MapPosition >
    {
        let mut iter_index = self.iter_index;
        let output = self.shape.iter_surface_area( &mut iter_index );
        self.iter_index = iter_index;
        output
    }
}

impl<'a> Iterator for TiledShape2DSurfaceAreaIterator<'a>
{
    type Item = MapPosition;

    fn next( &mut self ) -> Option< Self::Item >
    {
        self.iter_next()
    }
} */
