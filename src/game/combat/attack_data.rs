/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

 **/
// External includes.
use specs::Entity;

// Standard includes.

// Internal includes.
use super::{AttackValue, DamageType, DefenceValue};
use crate::items::weapons::WeaponGroup;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AttackData {
    attacker: Entity,
    defender: Entity,
    attack_modifier: AttackValue,
    defence_modifier: DefenceValue,
    weapon_group: WeaponGroup,
    damage_type: DamageType,
}

impl AttackData {
    pub fn new(
        attacker: Entity,
        defender: Entity,
        attack_modifier: AttackValue,
        defence_modifier: DefenceValue,
        weapon_group: WeaponGroup,
        damage_type: DamageType,
    ) -> Self {
        Self {
            attacker,
            defender,
            attack_modifier,
            defence_modifier,
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

    pub fn attack_modifier(&self) -> AttackValue {
        self.attack_modifier
    }

    pub fn attack_modifier_mut(&mut self) -> &mut AttackValue {
        &mut self.attack_modifier
    }

    pub fn damage_type(&self) -> DamageType {
        self.damage_type
    }

    pub fn _damage_type_mut(&mut self) -> &mut DamageType {
        &mut self.damage_type
    }

    pub fn defence_modifier(&self) -> DefenceValue {
        self.defence_modifier
    }

    pub fn defence_modifier_mut(&mut self) -> &mut DefenceValue {
        &mut self.defence_modifier
    }

    pub fn weapon_group(&self) -> WeaponGroup {
        self.weapon_group
    }

    pub fn _weapon_group_mut(&mut self) -> &mut WeaponGroup {
        &mut self.weapon_group
    }
}
