/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

The purpose of Multidim is to provide a standard generic two-dimensional array.

This prevents duplicating work, and the errors that would pop up if each type
that nneded a two-dimensional array, had to compose one themselves.

**/
// External includes.

// Standard includes.
use std::default::Default;

// Internal includes.

pub struct Multidim<T>
where
    T: Copy + Clone + Default,
{
    values: Vec<T>,
    len0: usize,
    len1: usize,
}

impl<T> Multidim<T>
where
    T: Copy + Clone + Default,
{
    pub fn new(len0: usize, len1: usize) -> Self {
        let mut output: Self = Self {
            values: Vec::<T>::new(),
            len0,
            len1,
        };
        output.values.resize(len0 * len1, Default::default());
        output
    }

    pub fn bounds(&self) -> (usize, usize) {
        (self.len0(), self.len1())
    }

    pub fn _is_in_bounds(&self, pos0: usize, pos1: usize) -> bool {
        ((pos0 >= self.len0()) || (pos1 >= self.len1())) == false
    }

    pub fn _is_i32_in_bounds(&self, pos0: i32, pos1: i32) -> bool {
        ((pos0 < 0) || (pos1 < 0)) == false && self._is_in_bounds(pos0 as usize, pos1 as usize)
    }

    pub fn len0(&self) -> usize {
        self.len0
    }
    pub fn len1(&self) -> usize {
        self.len1
    }

    pub fn value(&self, pos0: usize, pos1: usize) -> &T {
        &self.values[self.get_index(pos0, pos1)]
    }

    pub fn value_mut(&mut self, pos0: usize, pos1: usize) -> &mut T {
        let index = self.get_index(pos0, pos1);
        &mut self.values[index]
    }

    fn get_index(&self, pos0: usize, pos1: usize) -> usize {
        (pos0 * self.len1) + pos1
    }
}
