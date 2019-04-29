/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use specs::{Component, VecStorage};

// Standard includes.
use std::collections::hash_map::HashMap;

// Internal includes.
use super::{Tilemap, VisibilityMap};
use crate::rrl_math::calculate_hash;

pub struct VisibilityMapLookup {
    values: HashMap<u64, VisibilityMap>,
}

impl VisibilityMapLookup {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub fn get_or_add(&mut self, map: &Tilemap) -> &VisibilityMap {
        let map_hash = calculate_hash(map);

        self.values
            .entry(map_hash)
            .or_insert_with(|| VisibilityMap::new(map.width(), map.height()))
    }

    pub fn get_or_add_mut(&mut self, map: &Tilemap) -> &mut VisibilityMap {
        let map_hash = calculate_hash(map);

        self.values
            .entry(map_hash)
            .or_insert_with(|| VisibilityMap::new(map.width(), map.height()))
    }
}

impl Component for VisibilityMapLookup {
    type Storage = VecStorage<Self>;
}
