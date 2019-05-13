/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use specs::{Component, VecStorage, World};

// Standard includes.

// Internal includes.
use super::ArmourGroup;
use crate::game::combat::{DefenceValue, ProtectionValue};
use crate::game::points::{
    BuildLevel, BuildPoints, CostsBuildPoints, CostsCurrency, CurrencyValue, HasBuildLevel,
};

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

    pub fn defence_value_mut(&mut self) -> &mut DefenceValue {
        &mut self.defence_value
    }

    pub fn protection_value(&self) -> ProtectionValue {
        self.protection_value
    }

    pub fn protection_value_mut(&mut self) -> &mut ProtectionValue {
        &mut self.protection_value
    }
}

impl Component for Armour {
    type Storage = VecStorage<Self>;
}

impl CostsBuildPoints for Armour {
    fn build_points_total(&self, world: &World) -> BuildPoints {
        BuildPoints::from(self.build_level_total(world))
    }
}

impl CostsCurrency for Armour {
    fn currency_total(&self, world: &World) -> CurrencyValue {
        CurrencyValue::from(self.build_points_total(world))
    }
}

impl HasBuildLevel for Armour {
    fn build_level_total(&self, world: &World) -> BuildLevel {
        let base_level = self.protection_value().build_level_total(world);
        let raw_base_level = base_level.raw();
        let raw_defence_level = self.defence_value().build_level_total(world).raw();
        let raw_restriction_level = raw_defence_level / 3;
        BuildLevel::new(raw_base_level - raw_restriction_level)
    }
}
