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
use crate::game::points::{CostsBuildPoints, CostsCurrency};
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
        let build_points_total = self.armour_template.build_points_total(world);
        let currency_total = self.armour_template.currency_total(world);
        world
            .create_entity()
            .with(Item::new(
                ITEM_ICON_INDEX_ARMOUR,
                false,
                BitFlags::empty(),
                self.body_slot_type,
            ))
            .with(build_points_total)
            .with(currency_total)
            .with(self.name.clone())
            .with(self.armour_template)
            .build()
    }
}
