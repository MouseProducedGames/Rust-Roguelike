/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes

// internal includes
use crate::abilities::Ability;
use crate::rrl_math::Position;
use crate::stats::{CreatureStats, StatModifier};
use crate::world::{calculate_visibility, Tilemap, VisibilityMap};

pub fn ability_func(
    ability: Ability,
    stats: &mut CreatureStats,
    pos: &mut Position,
    map: &mut Tilemap,
    vis: &mut VisibilityMap,
)
{
    match ability {
        Ability::Light(range) => {
            let range = i32::from(range) + stats.perception().modifier();
            calculate_visibility(vis, *pos, range, map);
        },
    }
}