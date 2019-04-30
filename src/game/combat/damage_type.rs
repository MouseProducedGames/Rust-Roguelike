/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.

// Standard includes.

// Internal includes.
use super::{DamageValue, InjuryValue};

#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum DamageType {
    Blunt,
    Crushing,
    Piercing,
    Slashing,
}

impl DamageType {
    pub fn damage_to_damage(self, damage_value: DamageValue) -> DamageValue {
        DamageValue::from(match self {
            DamageType::Blunt => i32::from(damage_value) * 2,
            DamageType::Crushing => i32::from(damage_value),
            DamageType::Piercing => i32::from(damage_value) / 2,
            DamageType::Slashing => (i32::from(damage_value) * 2) / 3,
        })
    }

    pub fn damage_to_injury(self, damage_value: DamageValue) -> InjuryValue {
        InjuryValue::from(match self {
            DamageType::Blunt => i32::from(damage_value) / 2,
            DamageType::Crushing => i32::from(damage_value),
            DamageType::Piercing => i32::from(damage_value) * 2,
            DamageType::Slashing => (i32::from(damage_value) * 3) / 2,
        })
    }
}
