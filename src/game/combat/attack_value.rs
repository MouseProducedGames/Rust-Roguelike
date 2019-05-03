/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use specs::World;

// Standard includes.
use std::fmt;
use std::ops::{Add, AddAssign, Sub, SubAssign};

// Internal includes.
use super::DefenceValue;
use crate::game::points::{BuildLevel, BuildPoints, CostsBuildPoints, HasBuildLevel};
use crate::game::GameValue;
use crate::skills::SkillValue;

#[derive(Copy, Clone, Default)]
pub struct AttackMarker;

pub type AttackValue = GameValue<AttackMarker>;

impl Add<SkillValue> for AttackValue {
    type Output = AttackValue;

    fn add(self, other: SkillValue) -> Self {
        self + AttackValue::from(i32::from(other))
    }
}

impl Add<&SkillValue> for AttackValue {
    type Output = AttackValue;

    fn add(self, other: &SkillValue) -> Self {
        self + *other
    }
}

impl AddAssign<SkillValue> for AttackValue {
    fn add_assign(&mut self, other: SkillValue) {
        *self += AttackValue::from(i32::from(other));
    }
}

impl AddAssign<&SkillValue> for AttackValue {
    fn add_assign(&mut self, other: &SkillValue) {
        *self += *other;
    }
}

impl CostsBuildPoints for AttackValue {
    fn build_points_total(&self, world: &World) -> BuildPoints {
        BuildPoints::from(self.build_level_total(world))
    }
}

impl fmt::Display for AttackValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", i32::from(self))
    }
}

impl HasBuildLevel for AttackValue {
    fn build_level_total(&self, _world: &World) -> BuildLevel {
        BuildLevel::from(i32::from(self) * 10)
    }
}

impl Sub<DefenceValue> for AttackValue {
    type Output = AttackValue;

    fn sub(self, other: DefenceValue) -> Self {
        self - AttackValue::from(i32::from(other))
    }
}

impl SubAssign<DefenceValue> for AttackValue {
    fn sub_assign(&mut self, other: DefenceValue) {
        *self -= AttackValue::from(i32::from(other));
    }
}
