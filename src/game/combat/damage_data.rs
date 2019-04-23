/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

 **/
// External includes.
use specs::Entity;

// Standard includes.

// Internal includes.
use super::DamageValue;
use crate::items::weapons::WeaponGroup;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DamageData {
    attacker: Entity,
    defender: Entity,
    damage: DamageValue,
    weapon_group: WeaponGroup,
}

impl DamageData {
    pub fn new(
        attacker: Entity,
        defender: Entity,
        damage: DamageValue,
        weapon_group: WeaponGroup,
    ) -> Self {
        Self {
            attacker,
            defender,
            damage,
            weapon_group,
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

    pub fn weapon_group(&self) -> WeaponGroup {
        self.weapon_group
    }

    pub fn weapon_group_mut(&mut self) -> &mut WeaponGroup {
        &mut self.weapon_group
    }
}
