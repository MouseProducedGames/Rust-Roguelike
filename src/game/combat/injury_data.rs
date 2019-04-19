/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

 **/
// External includes.
use specs::Entity;

// Standard includes.

// Internal includes.
use super::InjuryValue;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct InjuryData {
    attacker: Entity,
    defender: Entity,
    pub injury: InjuryValue,
}

impl InjuryData {
    pub fn new(attacker: Entity, defender: Entity, injury: InjuryValue) -> Self {
        Self {
            attacker,
            defender,
            injury,
        }
    }

    pub fn _attacker(&self) -> Entity {
        self.attacker
    }

    pub fn defender(&self) -> Entity {
        self.defender
    }

    pub fn injury(&self) -> InjuryValue {
        self.injury
    }

    pub fn injury_mut(&mut self) -> &mut InjuryValue {
        &mut self.injury
    }
}
