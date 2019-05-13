/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

 **/
// External includes.

// Standard includes.

// Internal includes.
use crate::game::combat::{AttackValue, DefenceValue};
use crate::game::points::BuildLevel;

#[derive(Clone, Copy)]
pub struct WeaponSkillTypeData {
    attack_modifier: AttackValue,
    cost_modifier: BuildLevel,
    defence_modifier: DefenceValue,
}

impl WeaponSkillTypeData {
    pub fn new(
        attack_modifier: AttackValue,
        cost_modifier: BuildLevel,
        defence_modifier: DefenceValue,
    ) -> Self {
        Self {
            attack_modifier,
            cost_modifier,
            defence_modifier,
        }
    }

    pub fn attack_modifier(&self) -> AttackValue {
        self.attack_modifier
    }

    pub fn cost_modifier(&self) -> BuildLevel {
        self.cost_modifier
    }

    pub fn defence_modifier(&self) -> DefenceValue {
        self.defence_modifier
    }
}
