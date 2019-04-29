/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use enumflags2::BitFlags;
use specs::{Component, VecStorage};

// Standard includes.

// Internal includes.
use crate::bodies::{BodySlotFlags, BodySlotType, ImplementBodySlotFlags};

#[derive(Copy, Clone, PartialEq)]
pub struct Item {
    icon_id: u32,
    owned: bool,
    body_slot_flags: BitFlags<BodySlotFlags>,
    body_slot_type: BodySlotType,
}

impl Item {
    pub fn new(
        icon_id: u32,
        owned: bool,
        body_slot_flags: BitFlags<BodySlotFlags>,
        body_slot_type: BodySlotType,
    ) -> Self {
        Self {
            icon_id,
            owned,
            body_slot_flags,
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

impl ImplementBodySlotFlags for Item {
    fn is_attack(&self) -> bool {
        self.body_slot_flags.contains(BodySlotFlags::IsAttack)
    }

    fn make_attack(&mut self, value: bool) -> bool {
        let output = self.is_attack();
        if value {
            self.body_slot_flags |= BodySlotFlags::IsAttack;
        } else {
            self.body_slot_flags.remove(BodySlotFlags::IsAttack);
        }

        output
    }

    fn is_default(&self) -> bool {
        self.body_slot_flags.contains(BodySlotFlags::IsDefault)
    }

    fn make_default(&mut self, value: bool) -> bool {
        let output = self.is_default();
        if value {
            self.body_slot_flags |= BodySlotFlags::IsDefault;
        } else {
            self.body_slot_flags.remove(BodySlotFlags::IsDefault);
        }

        output
    }

    fn is_defence(&self) -> bool {
        self.body_slot_flags.contains(BodySlotFlags::IsDefence)
    }

    fn make_defence(&mut self, value: bool) -> bool {
        let output = self.is_defence();
        if value {
            self.body_slot_flags |= BodySlotFlags::IsDefence;
        } else {
            self.body_slot_flags.remove(BodySlotFlags::IsDefence);
        }

        output
    }
}
