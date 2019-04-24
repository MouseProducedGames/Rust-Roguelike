/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use specs::{Component, VecStorage};

// Standard includes.

// Internal includes.
use crate::bodies::BodySlotType;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Item {
    icon_id: u32,
    owned: bool,
    body_slot_type: BodySlotType,
}

impl Item {
    pub fn new(icon_id: u32, owned: bool, body_slot_type: BodySlotType) -> Self {
        Self {
            icon_id,
            owned,
            body_slot_type,
        }
    }

    pub fn icon_id(self) -> u32 {
        self.icon_id
    }

    pub fn owned(self) -> bool {
        self.owned
    }

    /// False if not previously owned.
    pub fn owned_mut(&mut self, owned: bool) -> bool {
        let output = self.owned();
        self.owned = owned;
        output
    }

    pub fn body_slot_type(self) -> BodySlotType {
        self.body_slot_type
    }
}

impl Component for Item {
    type Storage = VecStorage<Self>;
}
