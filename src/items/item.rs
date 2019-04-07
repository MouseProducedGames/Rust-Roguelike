/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.

// Standard includes.

// Internal includes.
use crate::abilities::Ability;
use crate::rrl_math::Position;
use crate::stats::CreatureStats;
use crate::world::{Lightmap, Tilemap, VisibilityMap};

pub enum Item {
    // Name, Ability.
    Generic(String, Ability),
}

impl Item {
    pub fn apply(
        &self,
        _stats: &mut CreatureStats,
        lightmap: &mut Lightmap,
        pos: &mut Position,
        map: &mut Tilemap,
        _vis: &mut VisibilityMap,
    ) {
        match self {
            Item::Generic(_, ability) => ability.apply(_stats, lightmap, pos, map, _vis),
        }
    }
}
