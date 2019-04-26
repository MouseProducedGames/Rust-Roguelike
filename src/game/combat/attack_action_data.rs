/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

 **/
// External includes.
use specs::Entity;

// Standard includes.

// Internal includes.

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AttackActionData {
    attacker: Entity,
    defender: Entity,
}

impl AttackActionData {
    pub fn new(attacker: Entity, defender: Entity) -> Self {
        Self { attacker, defender }
    }

    pub fn attacker(&self) -> Entity {
        self.attacker
    }

    pub fn defender(&self) -> Entity {
        self.defender
    }
}
