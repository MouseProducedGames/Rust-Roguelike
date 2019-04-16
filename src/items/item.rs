/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use specs::{Component, VecStorage};

// Standard includes.

// Internal includes.

pub struct Item {
    name: String,
    icon_id: u32,
}

impl Item {
    pub fn new(name: &str, icon_id: u32) -> Self {
        Self {
            name: String::from(name),
            icon_id,
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn name_mut(&mut self, name: String) -> &String {
        self.name = name;
        self.name()
    }

    pub fn icon_id(&self) -> u32 {
        self.icon_id
    }

    pub fn icon_id_mut(&mut self, icon_id: u32) -> u32 {
        self.icon_id = icon_id;
        self.icon_id()
    }
}

impl Component for Item {
    type Storage = VecStorage<Self>;
}
