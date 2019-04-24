/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use specs::{Builder, Entity, World};

// Standard includes.

// Internal includes.
use super::WeaponFactory;
use crate::bodies::BodySlotType;
use crate::data_types::Name;
use crate::items::weapons::Weapon;
use crate::items::{Item, ITEM_ICON_INDEX_WEAPON};

#[derive(Clone)]
pub struct TemplateWeaponFactory {
    name: Name,
    weapon_template: Weapon,
}

impl TemplateWeaponFactory {
    pub fn new(name: Name, weapon_template: Weapon) -> Self {
        Self {
            name,
            weapon_template,
        }
    }
}

impl WeaponFactory for TemplateWeaponFactory {
    fn create(&self, world: &mut World) -> Entity {
        world
            .create_entity()
            .with(Item::new(ITEM_ICON_INDEX_WEAPON, false, BodySlotType::Hand))
            .with(self.name.clone())
            .with(self.weapon_template)
            .build()
    }
}
