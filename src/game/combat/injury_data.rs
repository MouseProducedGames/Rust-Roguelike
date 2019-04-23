/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

 **/
// External includes.
use specs::Entity;

// Standard includes.

// Internal includes.
use super::InjuryValue;
use crate::items::weapons::WeaponType;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct InjuryData {
    attacker: Entity,
    defender: Entity,
    injury: InjuryValue,
    weapon_type: WeaponType,
}

impl InjuryData {
    pub fn new(
        attacker: Entity,
        defender: Entity,
        injury: InjuryValue,
        weapon_type: WeaponType,
    ) -> Self {
        Self {
            attacker,
            defender,
            injury,
            weapon_type,
        }
    }

    pub fn _attacker(&self) -> Entity {
        self.attacker
    }

    pub fn defender(&self) -> Entity {
        self.defender
    }

    pub fn injury(&self) -> InjuryValue {
        self.injury
    }

    pub fn injury_mut(&mut self) -> &mut InjuryValue {
        &mut self.injury
    }

    pub fn _weapon_type(&self) -> WeaponType {
        self.weapon_type
    }

    pub fn _weapon_type_mut(&mut self) -> &mut WeaponType {
        &mut self.weapon_type
    }
}
