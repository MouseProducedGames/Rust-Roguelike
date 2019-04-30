/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use specs::{Entity, World};

// Standard includes.
use std::default::Default;

// Internal includes.
use super::{CreatureFactory, CreatureProcessor};

#[derive(Clone)]
pub struct ProcessCreatureFactory<TCreatureFactory, TCreatureProcessor>(
    TCreatureFactory,
    TCreatureProcessor,
)
where
    TCreatureFactory: CreatureFactory + Default,
    TCreatureProcessor: CreatureProcessor + Default;

impl<TCreatureFactory, TCreatureProcessor>
    ProcessCreatureFactory<TCreatureFactory, TCreatureProcessor>
where
    TCreatureFactory: CreatureFactory + Default,
    TCreatureProcessor: CreatureProcessor + Default,
{
}

impl<TCreatureFactory, TCreatureProcessor> Default
    for ProcessCreatureFactory<TCreatureFactory, TCreatureProcessor>
where
    TCreatureFactory: CreatureFactory + Default,
    TCreatureProcessor: CreatureProcessor + Default,
{
    fn default() -> Self {
        Self {
            0: Default::default(),
            1: Default::default(),
        }
    }
}

impl<TCreatureFactory, TCreatureProcessor> CreatureFactory
    for ProcessCreatureFactory<TCreatureFactory, TCreatureProcessor>
where
    TCreatureFactory: CreatureFactory + Default,
    TCreatureProcessor: CreatureProcessor + Default,
{
    fn create(&self, world: &mut World) -> Entity {
        let item_entity = self.0.create(world);
        self.1.process(world, item_entity)
    }
}
