/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

 **/
// External includes.
use specs::Entity;

// Standard includes.

// Internal includes.

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DamageData {
    attacker: Entity,
    defender: Entity,
    damage: i32,
}

impl DamageData {
    pub fn new(attacker: Entity, defender: Entity, damage: i32) -> Self {
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

    pub fn damage(&self) -> i32 {
        self.damage
    }

    pub fn damage_mut(&mut self) -> &mut i32 {
        &mut self.damage
    }
}
