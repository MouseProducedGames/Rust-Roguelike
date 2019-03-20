/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/

// External includes
use specs::{ Component, VecStorage };
use std::default::Default;

// internal include
use crate::stats::Attribute;

#[derive(Copy, Clone)]
pub struct CreatureStats
{
    strength: Attribute,
    agility: Attribute,
    coordination: Attribute,
    _endurance: Attribute,
    health: Attribute,
}

impl CreatureStats
{
    pub fn strength( &self ) -> Attribute
    {
        self.strength
    }

    pub fn _strength_mut( &mut self ) -> &mut Attribute
    {
        &mut self.strength
    }

    pub fn agility( &self ) -> Attribute
    {
        self.agility
    }

    pub fn _agility_mut( &mut self ) -> &mut Attribute
    {
        &mut self.agility
    }

    pub fn coordination( &self ) -> Attribute
    {
        self.coordination
    }

    pub fn _coordination_mut( &mut self ) -> &mut Attribute
    {
        &mut self.coordination
    }

    pub fn _endurance( &self ) -> Attribute
    {
        self._endurance
    }

    pub fn _endurance_mut( &mut self ) -> &mut Attribute
    {
        &mut self._endurance
    }

    pub fn health( &self ) -> Attribute
    {
        self.health
    }

    pub fn health_mut( &mut self ) -> &mut Attribute
    {
        &mut self.health
    }
}

impl Component for CreatureStats
{
    type Storage = VecStorage<Self>;
}

impl Default for CreatureStats
{
    fn default() -> Self
    {
        let ten = Attribute::new( 10 );
        Self {
            strength: ten,
            agility: ten,
            coordination: ten,
            _endurance: ten,
            health: ten,
        }
    }
}
