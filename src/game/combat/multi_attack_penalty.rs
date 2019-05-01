/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use specs::{Component, VecStorage, World};

// Standard includes.

// Internal includes.
use super::AttackMarker;
use crate::game::points::{BuildPointsValue, CostsBuildPoints};
use crate::game::PenaltyValue;

#[derive(Copy, Clone, Default)]
pub struct MultiAttackMarker;

pub type MultiAttackPenalty = PenaltyValue<AttackMarker, MultiAttackMarker>;

impl Component for MultiAttackPenalty {
    type Storage = VecStorage<Self>;
}

impl CostsBuildPoints for MultiAttackPenalty {
    fn build_points_total(&self, _world: &World) -> BuildPointsValue {
        BuildPointsValue::from(i32::from(self) * 25)
    }
}
