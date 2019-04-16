/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use specs::{Component, Entity, VecStorage};

// Standard includes.
use std::sync::{Arc, Mutex, MutexGuard};

// internal includes.

#[derive(Clone)]
pub struct Inventory {
    values: Arc<Mutex<Vec<Entity>>>,
}

impl Inventory {
    pub fn new() -> Self {
        Self {
            values: Arc::new(Mutex::new(vec![])),
        }
    }

    pub fn get(&self) -> MutexGuard<Vec<Entity>> {
        self.values.lock().unwrap()
    }

    pub fn push(&mut self, item: Entity) {
        self.values.lock().unwrap().push(item)
    }

    pub fn remove(&mut self, index: usize) -> Entity {
        self.values.lock().unwrap().remove(index)
    }
}

impl Component for Inventory {
    type Storage = VecStorage<Self>;
}
