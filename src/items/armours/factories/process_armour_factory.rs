/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use specs::{Entity, World};

// Standard includes.
use std::default::Default;

// Internal includes.
use super::{ArmourFactory, ArmourProcessor};
use crate::game::points::{BuildPointsValue, CostsBuildPoints, CostsCurrency, CurrencyValue};
use crate::items::armours::Armour;

#[derive(Clone)]
pub struct ProcessArmourFactory<TArmourFactory, TArmourProcessor>(TArmourFactory, TArmourProcessor)
where
    TArmourFactory: ArmourFactory + Default,
    TArmourProcessor: ArmourProcessor + Default;

impl<TArmourFactory, TArmourProcessor> ProcessArmourFactory<TArmourFactory, TArmourProcessor>
where
    TArmourFactory: ArmourFactory + Default,
    TArmourProcessor: ArmourProcessor + Default,
{
}

impl<TArmourFactory, TArmourProcessor> Default
    for ProcessArmourFactory<TArmourFactory, TArmourProcessor>
where
    TArmourFactory: ArmourFactory + Default,
    TArmourProcessor: ArmourProcessor + Default,
{
    fn default() -> Self {
        Self {
            0: Default::default(),
            1: Default::default(),
        }
    }
}

impl<TArmourFactory, TArmourProcessor> ArmourFactory
    for ProcessArmourFactory<TArmourFactory, TArmourProcessor>
where
    TArmourFactory: ArmourFactory + Default,
    TArmourProcessor: ArmourProcessor + Default,
{
    fn create(&self, world: &mut World) -> Entity {
        let item_entity = self.0.create(world);
        let item_entity = self.1.process(world, item_entity);
        let armour = *world.read_storage::<Armour>().get(item_entity).unwrap();
        let build_points_total = armour.build_points_total(world);
        let currency_total = armour.currency_total(world);
        *world
            .write_storage::<BuildPointsValue>()
            .get_mut(item_entity)
            .unwrap() = build_points_total;
        *world
            .write_storage::<CurrencyValue>()
            .get_mut(item_entity)
            .unwrap() = currency_total;

        item_entity
    }
}
