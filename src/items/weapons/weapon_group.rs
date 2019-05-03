/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.

// Standard includes.
use std::fmt;

// Internal includes.

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
#[repr(C)]
pub enum WeaponGroup {
    Axes,
    Maces,
    Shields,
    Spears,
    Swords,
    Unarmed,
}

impl fmt::Display for WeaponGroup {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                WeaponGroup::Axes => "Axes",
                WeaponGroup::Maces => "Maces",
                WeaponGroup::Shields => "Shields",
                WeaponGroup::Spears => "Spears",
                WeaponGroup::Swords => "Swords",
                WeaponGroup::Unarmed => "Unarmed",
            },
        )
    }
}
