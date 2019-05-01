/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use enumflags2::BitFlags;
use specs::{Builder, Entity, World};

// Standard includes.

// Internal includes.
use super::WeaponFactory;
use crate::bodies::{BodySlotFlags, BodySlotType};
use crate::data_types::Name;
use crate::game::points::{CostsBuildPoints, CostsCurrency};
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
        let build_points_total = self.weapon_template.build_points_total(world);
        let currency_total = self.weapon_template.currency_total(world);
        world
            .create_entity()
            .with(Item::new(
                ITEM_ICON_INDEX_WEAPON,
                false,
                BitFlags::from(BodySlotFlags::IsAttack),
                BodySlotType::Palm,
            ))
            .with(build_points_total)
            .with(currency_total)
            .with(self.name.clone())
            .with(self.weapon_template)
            .build()
    }
}
