/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.

// Standard includes.

// internal includes.
use super::func::ability_func;
use crate::rrl_math::Position;
use crate::skills::SkillLookup;
use crate::stats::CreatureStats;
use crate::world::{Lightmap, Tilemap, VisibilityMap};

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub enum AbilityActivation {
    Passive(AbilityActivationOp),
}

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub enum AbilityActivationOp {
    EveryRound,
}

#[derive(Copy, Clone)]
pub enum Ability {
    Light(f64),
    ScanForSecrets(i32, AbilityRange),
}

#[derive(Copy, Clone)]
pub enum AbilityRange {
    Radius(u32),
}

impl Ability {
    pub fn apply(
        self,
        lightmap: &mut Lightmap,
        pos: &mut Position,
        skills: &mut SkillLookup,
        stats: &mut CreatureStats,
        map: &mut Tilemap,
        visibility_map: &mut VisibilityMap,
    ) {
        ability_func(self, lightmap, pos, skills, stats, map, visibility_map);
    }
}
