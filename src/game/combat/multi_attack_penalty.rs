/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use specs::{Component, VecStorage, World};

// Standard includes.

// Internal includes.
use super::AttackMarker;
use crate::game::points::{BuildLevel, HasBuildLevel};
use crate::game::PenaltyValue;

#[derive(Copy, Clone, Default)]
pub struct MultiAttackMarker;

pub type MultiAttackPenalty = PenaltyValue<AttackMarker, MultiAttackMarker>;

impl Component for MultiAttackPenalty {
    type Storage = VecStorage<Self>;
}

impl HasBuildLevel for MultiAttackPenalty {
    fn build_level_total(&self, _world: &World) -> BuildLevel {
        BuildLevel::from((i32::from(self) * 10) - 10)
    }
}
