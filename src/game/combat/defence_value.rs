/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use specs::World;

// Standard includes.

// Internal includes.
use crate::game::points::{BuildPointsValue, CostsBuildPoints};
use crate::game::GameValue;

#[derive(Copy, Clone, Default)]
pub struct DefenceMarker;

pub type DefenceValue = GameValue<DefenceMarker>;

impl CostsBuildPoints for DefenceValue {
    fn build_points_total(&self, _world: &World) -> BuildPointsValue {
        BuildPointsValue::from(i32::from(self) * 30)
    }
}
