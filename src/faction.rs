/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/

// External includes
use specs::{ Component, VecStorage };

// Internal includes
use std::cmp::{ Eq, PartialEq };

pub struct Faction
{
    id: u32
}

impl Faction
{
    pub fn new( id: u32 ) -> Self
    {
        Self { id }
    }
}

impl Component for Faction
{
    type Storage = VecStorage< Self >;
}

impl Eq for Faction {}

impl PartialEq for Faction
{
    fn eq( &self, other: &Faction ) -> bool
    {
        self.id == other.id
    }
}
