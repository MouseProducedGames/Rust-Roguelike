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
        DamageValue::new(match self {
            DamageType::Blunt => damage_value.raw() * 2,
            DamageType::Crushing => damage_value.raw(),
            DamageType::Piercing => damage_value.raw() / 2,
            DamageType::Slashing => (damage_value.raw() * 2) / 3,
        })
    }

    pub fn damage_to_injury(self, damage_value: DamageValue) -> InjuryValue {
        InjuryValue::new(match self {
            DamageType::Blunt => damage_value.raw() / 2,
            DamageType::Crushing => damage_value.raw(),
            DamageType::Piercing => damage_value.raw() * 2,
            DamageType::Slashing => (damage_value.raw() * 3) / 2,
        })
    }
}
