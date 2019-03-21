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

pub struct Multimap<T>
where
    T: Copy + Clone + Default,
{
    values: Multidim<T>,
}

impl<T> Multimap<T>
where
    T: Copy + Clone + Default,
{
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            values: Multidim::new(height as usize, width as usize),
        }
    }

    pub fn height(&self) -> u32 {
        self.values.len0() as u32
    }

    pub fn width(&self) -> u32 {
        self.values.len1() as u32
    }

    pub fn value(&self, pos_x: u32, pos_y: u32) -> &T {
        self.values.value(pos_y as usize, pos_x as usize)
    }

    pub fn value_mut(&mut self, pos_x: u32, pos_y: u32) -> &mut T {
        self.values.value_mut(pos_y as usize, pos_x as usize)
    }
}

impl<T> Mapping for Multimap<T>
where
    T: Copy + Clone + Default,
{
    fn height(&self) -> u32 {
        Multimap::height(self)
    }

    fn width(&self) -> u32 {
        Multimap::width(self)
    }
}
