/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

 **/
// External includes.
use specs::Entity;

// Standard includes.

// Internal includes.
use super::DamageValue;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DamageData {
    attacker: Entity,
    defender: Entity,
    damage: DamageValue,
}

impl DamageData {
    pub fn new(attacker: Entity, defender: Entity, damage: DamageValue) -> Self {
        Self {
            attacker,
            defender,
            damage,
        }
    }

    pub fn attacker(&self) -> Entity {
        self.attacker
    }

    pub fn defender(&self) -> Entity {
        self.defender
    }

    pub fn damage(&self) -> DamageValue {
        self.damage
    }

    pub fn damage_mut(&mut self) -> &mut DamageValue {
        &mut self.damage
    }
}
