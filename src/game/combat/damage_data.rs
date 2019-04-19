/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

 **/
// External includes.
use specs::Entity;

// Standard includes.

// Internal includes.
use super::DamageValue;
use crate::items::WeaponType;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DamageData {
    attacker: Entity,
    defender: Entity,
    damage: DamageValue,
    weapon_type: WeaponType,
}

impl DamageData {
    pub fn new(
        attacker: Entity,
        defender: Entity,
        damage: DamageValue,
        weapon_type: WeaponType,
    ) -> Self {
        Self {
            attacker,
            defender,
            damage,
            weapon_type,
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

    pub fn weapon_type(&self) -> WeaponType {
        self.weapon_type
    }

    pub fn weapon_type_mut(&mut self) -> &mut WeaponType {
        &mut self.weapon_type
    }
}
