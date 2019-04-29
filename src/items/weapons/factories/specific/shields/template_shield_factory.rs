/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use enumflags2::BitFlags;
use specs::{Builder, Entity, World};

// Standard includes.

// Internal includes.
use crate::bodies::{BodySlotFlags, BodySlotType};
use crate::data_types::Name;
use crate::items::weapons::factories::WeaponFactory;
use crate::items::weapons::Weapon;
use crate::items::{Item, ITEM_ICON_INDEX_WEAPON};

#[derive(Clone)]
pub struct TemplateShieldFactory {
    name: Name,
    shield_template: Weapon,
}

impl TemplateShieldFactory {
    pub fn new(name: Name, shield_template: Weapon) -> Self {
        Self {
            name,
            shield_template,
        }
    }
}

impl WeaponFactory for TemplateShieldFactory {
    fn create(&self, world: &mut World) -> Entity {
        world
            .create_entity()
            .with(Item::new(
                ITEM_ICON_INDEX_WEAPON,
                false,
                BitFlags::from(BodySlotFlags::IsDefence),
                BodySlotType::Palm,
            ))
            .with(self.name.clone())
            .with(self.shield_template)
            .build()
    }
}
