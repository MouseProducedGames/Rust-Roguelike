/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/

// External includes
use specs::{ Component, VecStorage };
use std::ops::{ Add, AddAssign, Sub, SubAssign };

// internal include
use crate::stats::{ Attribute, Stat };

#[derive(Copy, Clone )]
pub struct Health
{
    value: i32,
}

impl Health
{
    pub fn new( value: i32 ) -> Self
    {
        Self { value: value }
    }
}

impl Component for Health
{
    type Storage = VecStorage<Self>;
}

impl Attribute for Health
{
/*     fn modifier( &self ) -> i32
    {
        Stat::modifier( self )
    } */
}

impl Stat for Health
{
    fn value( &self ) -> i32
    {
        self.value
    }

    fn value_mut( &mut self ) -> &mut i32
    {
        &mut self.value
    }
}

impl Sub<i32> for Health
{
    type Output = Health;
    
    fn sub( self, other: i32 ) -> Health
    {
        Self { value: self.value() - other }
    }
}
