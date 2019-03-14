/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/

// External includes
use specs::{ Component, VecStorage };

// internal includes

pub struct SightRange
{
    value: i32,
}

impl SightRange
{
    pub fn new( value: i32 ) -> Self
    {
        Self { value: value }
    }
    
    pub fn sight_range( &self ) -> i32
    {
        self.value
    }
}

impl Component for SightRange
{
    type Storage = VecStorage<Self>;
}
