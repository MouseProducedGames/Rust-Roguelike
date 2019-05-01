/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use specs::{Component, VecStorage, World};

// Standard includes.

// Internal includes.
use super::WeaponGroup;
use crate::game::combat::{AttackValue, DamageType, DamageValue, DefenceValue};
use crate::game::points::{BuildPointsValue, CostsBuildPoints, CostsCurrency, CurrencyValue};

#[derive(Clone, Copy)]
pub struct Weapon {
    weapon_type: WeaponGroup,
    attack_value: AttackValue,
    damage_type: DamageType,
    damage_value: DamageValue,
    defence_value: DefenceValue,
}

impl Weapon {
    pub fn new(
        weapon_type: WeaponGroup,
        attack_value: AttackValue,
        damage_type: DamageType,
        damage_value: DamageValue,
        defence_value: DefenceValue,
    ) -> Self {
        Self {
            weapon_type,
            attack_value,
            damage_type,
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

    pub fn damage_type(&self) -> DamageType {
        self.damage_type
    }

    pub fn _damage_type_mut(&mut self) -> &mut DamageType {
        &mut self.damage_type
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

impl CostsBuildPoints for Weapon {
    fn build_points_total(&self, world: &World) -> BuildPointsValue {
        self.attack_value().build_points_total(world)
            + self.damage_value().build_points_total(world)
            + self.defence_value().build_points_total(world)
    }
}

impl CostsCurrency for Weapon {
    fn currency_total(&self, world: &World) -> CurrencyValue {
        CurrencyValue::from(self.attack_value().build_points_total(world))
            + CurrencyValue::from(self.damage_value().build_points_total(world))
            + CurrencyValue::from(self.defence_value().build_points_total(world))
    }
}
