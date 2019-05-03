/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use specs::World;

// Standard includes.
use std::fmt;
use std::ops::{Add, AddAssign};

// Internal includes.
use crate::game::points::{BuildLevel, BuildPoints, CostsBuildPoints, HasBuildLevel};
use crate::game::GameValue;
use crate::skills::SkillValue;

#[derive(Copy, Clone, Default)]
pub struct DefenceMarker;

pub type DefenceValue = GameValue<DefenceMarker>;

impl Add<SkillValue> for DefenceValue {
    type Output = DefenceValue;

    fn add(self, other: SkillValue) -> Self {
        self + DefenceValue::from(i32::from(other))
    }
}

impl Add<&SkillValue> for DefenceValue {
    type Output = DefenceValue;

    fn add(self, other: &SkillValue) -> Self {
        self + *other
    }
}

impl AddAssign<SkillValue> for DefenceValue {
    fn add_assign(&mut self, other: SkillValue) {
        *self += DefenceValue::from(i32::from(other));
    }
}

impl AddAssign<&SkillValue> for DefenceValue {
    fn add_assign(&mut self, other: &SkillValue) {
        *self += *other;
    }
}

impl CostsBuildPoints for DefenceValue {
    fn build_points_total(&self, world: &World) -> BuildPoints {
        BuildPoints::from(self.build_level_total(world))
    }
}

impl fmt::Display for DefenceValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", i32::from(self))
    }
}

impl HasBuildLevel for DefenceValue {
    fn build_level_total(&self, _world: &World) -> BuildLevel {
        BuildLevel::from(i32::from(self) * 10)
    }
}
