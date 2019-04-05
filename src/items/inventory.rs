/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes
use std::iter::Iterator;
use std::slice::Iter;

// internal include
use super::item::Item;

pub struct Inventory {
    values: Vec<Item>,
}

impl Inventory {
    pub fn new() -> Self {
        Self { values: vec![] }
    }
    
    fn iter(&mut self) -> Iter<Item> {
        self.values.iter()
    }
    
    pub fn push(&mut self, item: Item) {
        self.values.push(item)
    }
    
    pub fn remove(&mut self, index: usize) -> Item {
        self.values.remove(index)
    }
}