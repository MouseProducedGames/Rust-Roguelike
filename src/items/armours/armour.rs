/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use specs::{Component, VecStorage};

// Standard includes.

// Internal includes.
use super::ArmourGroup;
use crate::game::combat::{DefenceValue, ProtectionValue};

#[derive(Clone, Copy)]
pub struct Armour {
    armour_group: ArmourGroup,
    defence_value: DefenceValue,
    protection_value: ProtectionValue,
}

impl Armour {
    pub fn new(
        armour_group: ArmourGroup,
        defence_value: DefenceValue,
        protection_value: ProtectionValue,
    ) -> Self {
        Self {
            armour_group,
            defence_value,
            protection_value,
        }
    }

    pub fn armour_group(&self) -> ArmourGroup {
        self.armour_group
    }

    pub fn defence_value(&self) -> DefenceValue {
        self.defence_value
    }

    pub fn protection_value(&self) -> ProtectionValue {
        self.protection_value
    }
}

impl Component for Armour {
    type Storage = VecStorage<Self>;
}
