/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes

// internal includes
use super::func::ability_func;
use crate::rrl_math::Position;
use crate::stats::CreatureStats;
use crate::world::{Lightmap, Tilemap, VisibilityMap};

#[derive(Copy, Clone)]
pub enum Ability {
    Light(f64),
}

impl Ability {
    pub fn apply(
        self,
        _stats: &mut CreatureStats,
        lightmap: &mut Lightmap,
        pos: &mut Position,
        map: &mut Tilemap,
        _vis: &mut VisibilityMap,
    ) {
        ability_func(self, _stats, lightmap, pos, map, _vis);
    }
}
