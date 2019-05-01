/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use specs::World;

// Standard includes.
use std::convert::From;

// Internal includes.
use super::DamageValue;
use crate::game::points::{BuildPointsValue, CostsBuildPoints};
use crate::game::GameValue;

#[derive(Copy, Clone, Default)]
pub struct InjuryMarker;

pub type InjuryValue = GameValue<InjuryMarker>;

impl CostsBuildPoints for InjuryValue {
    fn build_points_total(&self, _world: &World) -> BuildPointsValue {
        BuildPointsValue::from(i32::from(self) * 30)
    }
}

impl From<DamageValue> for InjuryValue {
    fn from(value: DamageValue) -> Self {
        InjuryValue::from(i32::from(value))
    }
}
