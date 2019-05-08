/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use specs::{Entity, World};

// Standard includes.
use std::default::Default;

// Internal includes.
use super::WeaponGenerator;
use crate::game::points::{
    BuildLevel, BuildPoints, CostsBuildPoints, CostsCurrency, CurrencyValue,
};
use crate::items::weapons::Weapon;

#[derive(Clone)]
pub struct GenerateWeaponGenerator<TWeaponGenerator>(TWeaponGenerator)
where
    TWeaponGenerator: WeaponGenerator + Default;

impl<TWeaponGenerator> GenerateWeaponGenerator<TWeaponGenerator> where
    TWeaponGenerator: WeaponGenerator + Default
{
}

impl<TWeaponGenerator> Default for GenerateWeaponGenerator<TWeaponGenerator>
where
    TWeaponGenerator: WeaponGenerator + Default,
{
    fn default() -> Self {
        Self {
            0: Default::default(),
        }
    }
}

impl<TWeaponGenerator> WeaponGenerator for GenerateWeaponGenerator<TWeaponGenerator>
where
    TWeaponGenerator: WeaponGenerator + Default,
{
    fn create(&self, world: &mut World, level: BuildLevel) -> Entity {
        let item_entity = self.0.create(world, level);

        let weapon = *world.read_storage::<Weapon>().get(item_entity).unwrap();
        let build_points_total = weapon.build_points_total(world);
        let currency_total = weapon.currency_total(world);
        *world
            .write_storage::<BuildPoints>()
            .get_mut(item_entity)
            .unwrap() = build_points_total;
        *world
            .write_storage::<CurrencyValue>()
            .get_mut(item_entity)
            .unwrap() = currency_total;

        item_entity
    }
}
