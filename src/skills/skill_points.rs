/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use specs::{Component, VecStorage};

// Standard includes.
use std::convert::From;
use std::fmt;

// Internal includes.
use crate::game::points::BuildPoints;
use crate::game::GameValue;

#[derive(Copy, Clone, Default)]
pub struct SkillPointsMarker;

pub type SkillPoints = GameValue<SkillPointsMarker>;

impl Component for SkillPoints {
    type Storage = VecStorage<Self>;
}

impl fmt::Display for SkillPoints {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.raw())
    }
}

impl From<BuildPoints> for SkillPoints {
    fn from(build_points: BuildPoints) -> Self {
        SkillPoints::new(build_points.raw())
    }
}

impl From<&BuildPoints> for SkillPoints {
    fn from(build_points: &BuildPoints) -> Self {
        SkillPoints::from(*build_points)
    }
}

impl From<SkillPoints> for BuildPoints {
    fn from(skill_points: SkillPoints) -> Self {
        BuildPoints::new(skill_points.raw())
    }
}

impl From<&SkillPoints> for BuildPoints {
    fn from(skill_points: &SkillPoints) -> Self {
        BuildPoints::from(*skill_points)
    }
}
