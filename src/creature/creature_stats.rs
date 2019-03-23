/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes
use specs::{Component, VecStorage};
use std::default::Default;
use std::ops::{Add, AddAssign};

// internal include
use crate::stats::{Attribute, Stat};

#[derive(Copy, Clone)]
pub struct CreatureStats {
    strength: Attribute,
    agility: Attribute,
    coordination: Attribute,
    endurance: Attribute,
    health: Attribute,
}

impl CreatureStats {
    pub fn new(
        strength: i32,
        agility: i32,
        coordination: i32,
        endurance: i32,
        health: i32,
    ) -> Self {
        Self {
            strength: Attribute::new(strength),
            agility: Attribute::new(agility),
            coordination: Attribute::new(coordination),
            endurance: Attribute::new(endurance),
            health: Attribute::new(health),
        }
    }

    pub fn strength(&self) -> Attribute {
        self.strength
    }

    pub fn _strength_mut(&mut self) -> &mut Attribute {
        &mut self.strength
    }

    pub fn agility(&self) -> Attribute {
        self.agility
    }

    pub fn _agility_mut(&mut self) -> &mut Attribute {
        &mut self.agility
    }

    pub fn coordination(&self) -> Attribute {
        self.coordination
    }

    pub fn _coordination_mut(&mut self) -> &mut Attribute {
        &mut self.coordination
    }

    pub fn endurance(&self) -> Attribute {
        self.endurance
    }

    pub fn _endurance_mut(&mut self) -> &mut Attribute {
        &mut self.endurance
    }

    pub fn health(&self) -> Attribute {
        self.health
    }

    pub fn health_mut(&mut self) -> &mut Attribute {
        &mut self.health
    }
}

impl Add<CreatureStats> for CreatureStats {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(
            self.strength().value() + other.strength().value(),
            self.agility().value() + other.agility().value(),
            self.coordination().value() + other.coordination().value(),
            self.endurance().value() + other.endurance().value(),
            self.health().value() + other.health().value(),
        )
    }
}

impl AddAssign<CreatureStats> for CreatureStats {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl Component for CreatureStats {
    type Storage = VecStorage<Self>;
}

impl Default for CreatureStats {
    fn default() -> Self {
        let ten = Attribute::new(10);
        Self {
            strength: ten,
            agility: ten,
            coordination: ten,
            endurance: ten,
            health: ten,
        }
    }
}
