/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.

// Standard includes.
use std::collections::HashMap;

// Internal includes.
use super::MaslowFuncWrapper;

pub struct MaslowFuncWrapperLookup {
    values: HashMap<String, MaslowFuncWrapper>,
}

impl MaslowFuncWrapperLookup {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub fn get(&self, name: &str) -> Option<&MaslowFuncWrapper> {
        self.values.get(name)
    }

    pub fn insert(&mut self, item: MaslowFuncWrapper) {
        self.values.insert(item.name().clone(), item);
    }
}
