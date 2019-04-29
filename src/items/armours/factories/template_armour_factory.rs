/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use enumflags2::BitFlags;
use specs::{Builder, Entity, World};

// Standard includes.

// Internal includes.
use super::ArmourFactory;
use crate::bodies::BodySlotType;
use crate::data_types::Name;
use crate::items::armours::Armour;
use crate::items::{Item, ITEM_ICON_INDEX_ARMOUR};

#[derive(Clone)]
pub struct TemplateArmourFactory {
    name: Name,
    body_slot_type: BodySlotType,
    armour_template: Armour,
}

impl TemplateArmourFactory {
    pub fn new(name: Name, body_slot_type: BodySlotType, armour_template: Armour) -> Self {
        Self {
            name,
            body_slot_type,
            armour_template,
        }
    }
}

impl ArmourFactory for TemplateArmourFactory {
    fn create(&self, world: &mut World) -> Entity {
        world
            .create_entity()
            .with(Item::new(
                ITEM_ICON_INDEX_ARMOUR,
                false,
                BitFlags::empty(),
                self.body_slot_type,
            ))
            .with(self.name.clone())
            .with(self.armour_template)
            .build()
    }
}
