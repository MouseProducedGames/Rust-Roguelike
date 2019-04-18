/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.

// Standard includes.
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// Internal includes.
use super::MaslowNode;

pub struct MaslowNodeLookup {
    values: HashMap<String, Arc<Mutex<MaslowNode>>>,
}

impl MaslowNodeLookup {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub fn get(&self, name: &str) -> Option<&Arc<Mutex<MaslowNode>>> {
        self.values.get(name)
    }

    pub fn insert(&mut self, item: Arc<Mutex<MaslowNode>>) {
        let name = item.lock().unwrap().name().clone();
        self.values.insert(name, item);
    }
}
