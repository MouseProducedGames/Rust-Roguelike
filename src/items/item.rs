/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use specs::{Component, VecStorage};

// Standard includes.

// Internal includes.

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Item {
    icon_id: u32,
    owned: bool,
}

impl Item {
    pub fn new(icon_id: u32, owned: bool) -> Self {
        Self { icon_id, owned }
    }

    pub fn icon_id(&self) -> u32 {
        self.icon_id
    }

    pub fn icon_id_mut(&mut self, icon_id: u32) -> u32 {
        self.icon_id = icon_id;
        self.icon_id()
    }

    pub fn owned(&self) -> bool {
        self.owned
    }

    /// False if not previously owned.
    pub fn owned_mut(&mut self, owned: bool) -> bool {
        let output = self.owned();
        self.owned = owned;
        output
    }
}

impl Component for Item {
    type Storage = VecStorage<Self>;
}
