/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes

// Standard includes.

// Internal includes.
use super::ability::Ability;
use crate::rrl_math::Position;
use crate::stats::CreatureStats;
use crate::world::{calculate_light_level, Lightmap, Tilemap, VisibilityMap};

pub fn ability_func(
    ability: Ability,
    _stats: &mut CreatureStats,
    lightmap: &mut Lightmap,
    pos: &mut Position,
    map: &mut Tilemap,
    _vis: &mut VisibilityMap,
) {
    match ability {
        Ability::Light(value) => {
            calculate_light_level(lightmap, *pos, value, map);
        }
    }
}
