/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use specs::{Component, VecStorage};

// Standard includes.
use std::sync::{Arc, Mutex, MutexGuard};

// internal includes.
use super::Item;

#[derive(Clone)]
pub struct Inventory {
    values: Arc<Mutex<Vec<Item>>>,
}

impl Inventory {
    pub fn new() -> Self {
        Self {
            values: Arc::new(Mutex::new(vec![])),
        }
    }

    pub fn get(&self) -> MutexGuard<Vec<Item>> {
        self.values.lock().unwrap()
    }

    pub fn push(&mut self, item: Item) {
        self.values.lock().unwrap().push(item)
    }

    pub fn remove(&mut self, index: usize) -> Item {
        self.values.lock().unwrap().remove(index)
    }
}

impl Component for Inventory {
    type Storage = VecStorage<Self>;
}
