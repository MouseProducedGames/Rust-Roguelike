/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/

// External includes
use specs::{ Component, VecStorage };
use std::ops::{ Add, AddAssign, Sub, SubAssign };

// internal include
use crate::stats::{ Stat, StatModifier };

#[derive(Copy, Clone )]
pub struct Attribute
{
    value: i32,
}

impl Attribute
{
    pub fn new( value: i32 ) -> Self
    {
        Self { value }
    }
}

impl Component for Attribute
{
    type Storage = VecStorage<Self>;
}

impl StatModifier for Attribute
{
}

impl Stat for Attribute
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

impl Add<i32> for Attribute
{
    type Output = Attribute;
    
    fn add( self, other: i32 ) -> Attribute
    {
        Attribute::new( self.value() + other )
    }
}

impl AddAssign<i32> for Attribute
{
    fn add_assign( &mut self, other: i32 )
    {
        *self.value_mut() += other;
    }
}

impl Sub<i32> for Attribute
{
    type Output = Attribute;
    
    fn sub( self, other: i32 ) -> Attribute
    {
        Attribute::new( self.value() - other )
    }
}

impl SubAssign<i32> for Attribute
{
    fn sub_assign( &mut self, other: i32 )
    {
        *self.value_mut() -= other;
    }
}
