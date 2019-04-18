/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.

// Standard includes.
use std::collections::HashMap;

// Internal includes.
use super::MaslowTree;

pub struct MaslowTreeLookup {
    values: HashMap<String, MaslowTree>,
}

impl MaslowTreeLookup {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub fn get(&self, name: &str) -> Option<&MaslowTree> {
        self.values.get(name)
    }

    pub fn insert(&mut self, item: MaslowTree) {
        self.values.insert(item.name().clone(), item);
    }
}
