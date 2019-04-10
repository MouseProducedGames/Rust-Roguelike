/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.

// Standard includes.

// Internal includes.
use crate::abilities::Ability;
use crate::rrl_math::Position;
use crate::skills::SkillLookup;
use crate::stats::CreatureStats;
use crate::world::{Lightmap, Tilemap, VisibilityMap};

pub enum Item {
    // Name, Ability.
    Generic(String, Ability),
}

impl Item {
    pub fn apply(
        &self,
        lightmap: &mut Lightmap,
        pos: &mut Position,
        skills: &mut SkillLookup,
        stats: &mut CreatureStats,
        map: &mut Tilemap,
        visibility_map: &mut VisibilityMap,
    ) {
        match self {
            Item::Generic(_, ability) => {
                ability.apply(lightmap, pos, skills, stats, map, visibility_map)
            }
        }
    }
}
