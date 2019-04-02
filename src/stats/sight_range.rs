/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes
use specs::{Component, VecStorage};

// internal includes
use crate::abilities::Ability;
use crate::data_types::Range;

pub struct SightRange {
    value: Ability,
}

impl SightRange {
    pub fn new(value: i32) -> Self {
        Self { value: Ability::Light(Range::new(value)) }
    }

    pub fn sight_range(&self) -> Ability {
        self.value
    }
}

impl Component for SightRange {
    type Storage = VecStorage<Self>;
}
