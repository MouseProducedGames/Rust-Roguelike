/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use specs::World;

// Standard includes.
use std::ops::{Sub, SubAssign};

// Internal includes.
use super::ProtectionValue;
use crate::game::points::{BuildPointsValue, CostsBuildPoints};
use crate::game::GameValue;

#[derive(Copy, Clone, Default)]
pub struct DamageMarker;

pub type DamageValue = GameValue<DamageMarker>;

impl CostsBuildPoints for DamageValue {
    fn build_points_total(&self, _world: &World) -> BuildPointsValue {
        BuildPointsValue::from(i32::from(self) * 20)
    }
}

impl Sub<ProtectionValue> for DamageValue {
    type Output = DamageValue;

    fn sub(self, other: ProtectionValue) -> DamageValue {
        self - DamageValue::from(i32::from(other))
    }
}

impl SubAssign<ProtectionValue> for DamageValue {
    fn sub_assign(&mut self, other: ProtectionValue) {
        self.sub_assign(DamageValue::from(i32::from(other)));
    }
}
