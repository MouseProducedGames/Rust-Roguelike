/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

 **/
// External includes.
use specs::Entity;

// Standard includes.

// Internal includes.
use super::{DamageType, DamageValue, ProtectionValue};
use crate::items::armours::ArmourGroup;
use crate::items::weapons::WeaponGroup;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ProtectionData {
    attacker: Entity,
    defender: Entity,
    damage: DamageValue,
    protection: ProtectionValue,
    armour_group: ArmourGroup,
    weapon_group: WeaponGroup,
    damage_type: DamageType,
}

impl ProtectionData {
    pub fn new(
        attacker: Entity,
        defender: Entity,
        damage: DamageValue,
        protection: ProtectionValue,
        armour_group: ArmourGroup,
        weapon_group: WeaponGroup,
        damage_type: DamageType,
    ) -> Self {
        Self {
            attacker,
            defender,
            damage,
            protection,
            armour_group,
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

    pub fn protection(&self) -> ProtectionValue {
        self.protection
    }

    pub fn protection_mut(&mut self) -> &mut ProtectionValue {
        &mut self.protection
    }

    pub fn armour_group(&self) -> ArmourGroup {
        self.armour_group
    }

    pub fn armour_group_mut(&mut self) -> &mut ArmourGroup {
        &mut self.armour_group
    }

    pub fn weapon_group(&self) -> WeaponGroup {
        self.weapon_group
    }

    pub fn damage_type(&self) -> DamageType {
        self.damage_type
    }

    pub fn _damage_type_mut(&mut self) -> &mut DamageType {
        &mut self.damage_type
    }
}
