/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use specs::{Entity, World};

// Standard includes.
use std::default::Default;

// Internal includes.
use super::{WeaponFactory, WeaponProcessor};
use crate::game::points::{BuildPoints, CostsBuildPoints, CostsCurrency, CurrencyValue};
use crate::items::weapons::Weapon;

#[derive(Clone)]
pub struct ProcessWeaponFactory<TWeaponFactory, TWeaponProcessor>(TWeaponFactory, TWeaponProcessor)
where
    TWeaponFactory: WeaponFactory + Default,
    TWeaponProcessor: WeaponProcessor + Default;

impl<TWeaponFactory, TWeaponProcessor> ProcessWeaponFactory<TWeaponFactory, TWeaponProcessor>
where
    TWeaponFactory: WeaponFactory + Default,
    TWeaponProcessor: WeaponProcessor + Default,
{
}

impl<TWeaponFactory, TWeaponProcessor> Default
    for ProcessWeaponFactory<TWeaponFactory, TWeaponProcessor>
where
    TWeaponFactory: WeaponFactory + Default,
    TWeaponProcessor: WeaponProcessor + Default,
{
    fn default() -> Self {
        Self {
            0: Default::default(),
            1: Default::default(),
        }
    }
}

impl<TWeaponFactory, TWeaponProcessor> WeaponFactory
    for ProcessWeaponFactory<TWeaponFactory, TWeaponProcessor>
where
    TWeaponFactory: WeaponFactory + Default,
    TWeaponProcessor: WeaponProcessor + Default,
{
    fn create(&self, world: &mut World) -> Entity {
        let item_entity = self.0.create(world);
        let item_entity = self.1.process(world, item_entity);
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
