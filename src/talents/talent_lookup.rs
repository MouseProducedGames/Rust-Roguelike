/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

 **/
// External includes.
use specs::{Component, VecStorage};
use std::collections::HashMap;

// Internal includes.
use crate::abilities::{Ability, AbilityActivation};

pub struct TalentLookup {
    values: HashMap<AbilityActivation, Vec<Ability>>,
}

impl TalentLookup {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub fn get_set(&mut self, key: AbilityActivation) -> &Vec<Ability> {
        self.values.entry(key).or_insert_with(|| vec![])
    }

    pub fn get_set_mut(&mut self, key: AbilityActivation) -> &mut Vec<Ability> {
        self.values.entry(key).or_insert_with(|| vec![])
    }

    pub fn insert(&mut self, key: AbilityActivation, value: Ability) {
        self.get_set_mut(key).push(value);
    }
}

impl Component for TalentLookup {
    type Storage = VecStorage<Self>;
}
