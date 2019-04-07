/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.

// Standard includes.
use std::marker::PhantomData;

// Internal includes.

pub struct Immut<T> {
    value: T,
    phantom: PhantomData<T>,
}

impl<T> Immut<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            phantom: PhantomData,
        }
    }

    pub fn read(&self) -> &T {
        &self.value
    }
}
