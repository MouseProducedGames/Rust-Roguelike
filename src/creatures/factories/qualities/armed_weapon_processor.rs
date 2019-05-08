/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use rand::{thread_rng, Rng};
use specs::{Entity, World};

// Standard includes.
use std::default::Default;

// Internal includes.
use super::{ArmedAxeProcessor, ArmedMaceProcessor, ArmedSpearProcessor, ArmedSwordProcessor};
use crate::creatures::factories::LeveledCreatureProcessor;
use crate::game::points::BuildLevel;

#[derive(Clone)]
pub struct ArmedWeaponProcessor;

impl ArmedWeaponProcessor {}

impl Default for ArmedWeaponProcessor {
    fn default() -> Self {
        Self {}
    }
}

impl LeveledCreatureProcessor for ArmedWeaponProcessor {
    fn process(&self, world: &mut World, creature_entity: Entity, level: BuildLevel) -> Entity {
        match thread_rng().gen_range(0, 4) {
            0 => ArmedAxeProcessor::default().process(world, creature_entity, level),
            1 => ArmedMaceProcessor::default().process(world, creature_entity, level),
            2 => ArmedSpearProcessor::default().process(world, creature_entity, level),
            3 => ArmedSwordProcessor::default().process(world, creature_entity, level),
            _ => panic!("Unhandled match case!"),
        }
    }
}
