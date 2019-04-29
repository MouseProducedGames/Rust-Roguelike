/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use enumflags2::BitFlags;
use specs::Entity;

// Standard includes.

// Internal includes.
use super::{BodySlotFlags, BodySlotType, ImplementBodySlotFlags};

#[derive(Clone, PartialEq)]
pub struct BodySlot {
    name: String,
    size: u32,
    slot_type: BodySlotType,
    flags: BitFlags<BodySlotFlags>,
    default_item: Entity,
    held_item: Option<Entity>,
}

impl BodySlot {
    pub fn new(name: &str, size: u32, slot_type: BodySlotType, default_item: Entity) -> Self {
        Self {
            name: String::from(name),
            size,
            slot_type,
            flags: BitFlags::<BodySlotFlags>::empty(),
            default_item,
            held_item: None,
        }
    }
    pub fn with_flags(
        name: &str,
        size: u32,
        slot_type: BodySlotType,
        flags: BitFlags<BodySlotFlags>,
        default_item: Entity,
    ) -> Self {
        Self {
            name: String::from(name),
            size,
            slot_type,
            flags,
            default_item,
            held_item: None,
        }
    }

    pub fn _with_held_item(
        name: &str,
        size: u32,
        slot_type: BodySlotType,
        default_item: Entity,
        held_item: Entity,
    ) -> Self {
        let mut output = Self::new(name, size, slot_type, default_item);
        output.hold_item(held_item);

        output
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn default_item(&self) -> Entity {
        self.default_item
    }

    pub fn item(&self) -> Entity {
        if let Some(item) = self.held_item {
            item
        } else {
            self.default_item
        }
    }

    pub fn held_item(&self) -> Option<Entity> {
        self.held_item
    }

    pub fn hold_item(&mut self, item: Entity) -> Option<Entity> {
        let output = self.held_item;
        self.held_item = Some(item);

        output
    }

    pub fn drop_item(&mut self) -> Option<Entity> {
        let output = self.held_item;
        self.held_item = None;

        output
    }

    pub fn size(&self) -> u32 {
        self.size
    }

    pub fn slot_type(&self) -> BodySlotType {
        self.slot_type
    }
}

impl ImplementBodySlotFlags for BodySlot {
    fn is_attack(&self) -> bool {
        self.flags.contains(BodySlotFlags::IsAttack)
    }

    fn make_attack(&mut self, value: bool) -> bool {
        let output = self.is_attack();
        if value {
            self.flags |= BodySlotFlags::IsAttack;
        } else {
            self.flags.remove(BodySlotFlags::IsAttack);
        }

        output
    }

    fn is_default(&self) -> bool {
        self.flags.contains(BodySlotFlags::IsDefault)
    }

    fn make_default(&mut self, value: bool) -> bool {
        let output = self.is_default();
        if value {
            self.flags |= BodySlotFlags::IsDefault;
        } else {
            self.flags.remove(BodySlotFlags::IsDefault);
        }

        output
    }

    fn is_defence(&self) -> bool {
        self.flags.contains(BodySlotFlags::IsDefence)
    }

    fn make_defence(&mut self, value: bool) -> bool {
        let output = self.is_defence();
        if value {
            self.flags |= BodySlotFlags::IsDefence;
        } else {
            self.flags.remove(BodySlotFlags::IsDefence);
        }

        output
    }
}
