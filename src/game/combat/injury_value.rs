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
use crate::game::points::{BuildLevel, HasBuildLevel};
use crate::game::GameValue;

#[derive(Copy, Clone, Default)]
pub struct InjuryMarker;

pub type InjuryValue = GameValue<InjuryMarker>;

impl HasBuildLevel for InjuryValue {
    fn build_level_total(&self, _world: &World) -> BuildLevel {
        BuildLevel::new(self.raw() * 10)
    }
}

impl From<DamageValue> for InjuryValue {
    fn from(value: DamageValue) -> Self {
        InjuryValue::new(value.raw())
    }
}
