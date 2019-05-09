/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use specs::{Entity, World};

// Standard includes.
use std::default::Default;

// Internal includes.
use crate::creatures::factories::qualities::armaments::{
    ArmedShieldProcessor, ArmedSwordProcessor,
};
use crate::creatures::factories::qualities::skills::{SkillShieldProcessor, SkillSwordProcessor};
use crate::creatures::factories::LeveledCreatureProcessor;
use crate::game::points::BuildLevel;

#[derive(Clone)]
pub struct ProfessionSwordAndShieldProcessor;

impl ProfessionSwordAndShieldProcessor {}

impl Default for ProfessionSwordAndShieldProcessor {
    fn default() -> Self {
        Self {}
    }
}

impl LeveledCreatureProcessor for ProfessionSwordAndShieldProcessor {
    fn process(&self, world: &mut World, creature_entity: Entity, level: BuildLevel) -> Entity {
        let creature_entity =
            ArmedShieldProcessor::default().process(world, creature_entity, level);
        let creature_entity = ArmedSwordProcessor::default().process(world, creature_entity, level);
        let creature_entity =
            SkillShieldProcessor::default().process(world, creature_entity, level);
        SkillSwordProcessor::default().process(world, creature_entity, level)
    }
}
