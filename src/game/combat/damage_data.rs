/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

 **/
// External includes.
use specs::Entity;

// Standard includes.

// Internal includes.
use super::{DamageType, DamageValue};
use crate::items::weapons::WeaponGroup;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DamageData {
    attacker: Entity,
    defender: Entity,
    damage: DamageValue,
    weapon_group: WeaponGroup,
    damage_type: DamageType,
}

impl DamageData {
    pub fn new(
        attacker: Entity,
        defender: Entity,
        damage: DamageValue,
        weapon_group: WeaponGroup,
        damage_type: DamageType,
    ) -> Self {
        Self {
            attacker,
            defender,
            damage,
            weapon_group,
            damage_type,
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

    pub fn damage_type(&self) -> DamageType {
        self.damage_type
    }

    pub fn _damage_type_mut(&mut self) -> &mut DamageType {
        &mut self.damage_type
    }

    pub fn weapon_group(&self) -> WeaponGroup {
        self.weapon_group
    }

    pub fn weapon_group_mut(&mut self) -> &mut WeaponGroup {
        &mut self.weapon_group
    }
}
