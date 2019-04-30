/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

 **/
// External includes.
use specs::Entity;

// Standard includes.

// Internal includes.
use super::{DamageType, InjuryValue};
use crate::items::weapons::WeaponGroup;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct InjuryData {
    attacker: Entity,
    defender: Entity,
    injury: InjuryValue,
    weapon_group: WeaponGroup,
    damage_type: DamageType,
}

impl InjuryData {
    pub fn new(
        attacker: Entity,
        defender: Entity,
        injury: InjuryValue,
        weapon_group: WeaponGroup,
        damage_type: DamageType,
    ) -> Self {
        Self {
            attacker,
            defender,
            injury,
            weapon_group,
            damage_type,
        }
    }

    pub fn _attacker(&self) -> Entity {
        self.attacker
    }

    pub fn defender(&self) -> Entity {
        self.defender
    }

    pub fn damage_type(&self) -> DamageType {
        self.damage_type
    }

    pub fn _damage_type_mut(&mut self) -> &mut DamageType {
        &mut self.damage_type
    }

    pub fn injury(&self) -> InjuryValue {
        self.injury
    }

    pub fn injury_mut(&mut self) -> &mut InjuryValue {
        &mut self.injury
    }

    pub fn _weapon_group(&self) -> WeaponGroup {
        self.weapon_group
    }

    pub fn _weapon_group_mut(&mut self) -> &mut WeaponGroup {
        &mut self.weapon_group
    }
}
