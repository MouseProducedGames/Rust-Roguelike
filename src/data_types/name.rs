/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use specs::{Component, VecStorage};

// Standard include.
use std::ops::{Deref, DerefMut};

// Internal include.

#[derive(Clone, Display, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Name(String);

impl Name {
    pub fn new(name: &str) -> Self {
        Self {
            0: String::from(name),
        }
    }
}

impl Component for Name {
    type Storage = VecStorage<Self>;
}

impl Deref for Name {
    type Target = String;

    fn deref(&self) -> &String {
        &self.0
    }
}

impl DerefMut for Name {
    fn deref_mut(&mut self) -> &mut String {
        &mut self.0
    }
}
