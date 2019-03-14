/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

The purpose of Multimap is to eliminated confusion when translating between
world and local positions (in the form of ( x,  y ) ) and bounds
(in the form of ( Width, Height )), and the underlying two-dimensional array,
which is accessed in the form of ( y, x ) and
has bounds in the form of ( Height, Width ).

This prevents duplicating work, and a number of positional argument errors
in dungeon and world maps.

Multimap is a base class for dungeon and world map information.
As such, it is composed ( Width, Height ) and accessed in the order ( x, y ).

Multimap is implemented as a thin wrapper over Multidim.

**/

// External includes

// Internal includes
use super::multidim::Multidim;
use super::world::Mapping;

pub struct Multimap<T> where T: Copy + Clone + Default
{
    values: Multidim<T>,
}

impl<T> Multimap<T> where T: Copy + Clone + Default
{
    pub fn new(width: usize, height: usize) -> Self
    {
        Self {
            values: Multidim::new( height, width )
        }
    }

    pub fn height(&self) -> usize
    {
        self.values.len0()
    }
    
    pub fn width(&self) -> usize
    {
        self.values.len1()
    }
    
    pub fn value(&self, pos_x: usize, pos_y: usize) -> &T
    {
        self.values.value( pos_y, pos_x )
    }

    pub fn value_mut(&mut self, pos_x: usize, pos_y: usize) -> &mut T
    {
        self.values.value_mut( pos_y, pos_x )
    }
}

impl<T> Mapping for Multimap<T> where T: Copy + Clone + Default
{
    fn height( &self ) -> usize
    {
        Multimap::height ( self )
    }

    fn width( &self ) -> usize
    {
        Multimap::width ( self )
    }
}
