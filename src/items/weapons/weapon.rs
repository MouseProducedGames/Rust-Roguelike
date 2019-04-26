/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use specs::{Component, VecStorage};

// Standard includes.

// Internal includes.
use super::WeaponGroup;
use crate::game::combat::{AttackValue, DamageValue, DefenceValue};

#[derive(Clone, Copy)]
pub struct Weapon {
    weapon_type: WeaponGroup,
    attack_value: AttackValue,
    damage_value: DamageValue,
    defence_value: DefenceValue,
}

impl Weapon {
    pub fn new(
        weapon_type: WeaponGroup,
        attack_value: AttackValue,
        damage_value: DamageValue,
        defence_value: DefenceValue,
    ) -> Self {
        Self {
            weapon_type,
            attack_value,
            damage_value,
            defence_value,
        }
    }

    pub fn weapon_group(&self) -> WeaponGroup {
        self.weapon_type
    }

    pub fn attack_value(&self) -> AttackValue {
        self.attack_value
    }

    pub fn attack_value_mut(&mut self) -> &mut AttackValue {
        &mut self.attack_value
    }

    pub fn damage_value(&self) -> DamageValue {
        self.damage_value
    }

    pub fn damage_value_mut(&mut self) -> &mut DamageValue {
        &mut self.damage_value
    }

    pub fn defence_value(&self) -> DefenceValue {
        self.defence_value
    }

    pub fn defence_value_mut(&mut self) -> &mut DefenceValue {
        &mut self.defence_value
    }
}

impl Component for Weapon {
    type Storage = VecStorage<Self>;
}
