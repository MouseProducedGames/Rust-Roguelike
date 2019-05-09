/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

 **/
// External includes.
use specs::{Component, VecStorage};

// Standard includes.
use std::collections::HashMap;

// Internal includes.
use crate::skills::{SkillActivation, SkillType};

pub struct SkillLookup {
    values: HashMap<SkillActivation, Vec<SkillType>>,
}

impl SkillLookup {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub fn get_set(&self, key: SkillActivation) -> Option<&Vec<SkillType>> {
        self.values.get(&key)
    }

    pub fn get_set_mut(&mut self, key: SkillActivation) -> Option<&mut Vec<SkillType>> {
        self.values.get_mut(&key)
    }

    pub fn get_or_add_set(&mut self, key: SkillActivation) -> &Vec<SkillType> {
        self.values.entry(key).or_insert_with(|| vec![])
    }

    pub fn get_or_add_set_mut(&mut self, key: SkillActivation) -> &mut Vec<SkillType> {
        self.values.entry(key).or_insert_with(|| vec![])
    }

    pub fn insert(&mut self, key: SkillActivation, value: SkillType) {
        self.values.entry(key).or_insert_with(|| vec![]).push(value);
    }
}

impl Component for SkillLookup {
    type Storage = VecStorage<Self>;
}
