/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use specs::{Entity, World};

// Standard includes.
use std::default::Default;

// Internal includes.
use crate::creatures::factories::LeveledCreatureProcessor;
use crate::game::points::BuildLevel;

#[derive(Clone)]
pub struct TemplateProfessionShieldProcessor<TArmamentProcessor, TSkillProcessor>
where
    TArmamentProcessor: LeveledCreatureProcessor + Default,
    TSkillProcessor: LeveledCreatureProcessor + Default,
{
    armament_processor: TArmamentProcessor,
    skill_processor: TSkillProcessor,
}

impl<TArmamentProcessor, TSkillProcessor>
    TemplateProfessionShieldProcessor<TArmamentProcessor, TSkillProcessor>
where
    TArmamentProcessor: LeveledCreatureProcessor + Default,
    TSkillProcessor: LeveledCreatureProcessor + Default,
{
}

impl<TArmamentProcessor, TSkillProcessor> Default
    for TemplateProfessionShieldProcessor<TArmamentProcessor, TSkillProcessor>
where
    TArmamentProcessor: LeveledCreatureProcessor + Default,
    TSkillProcessor: LeveledCreatureProcessor + Default,
{
    fn default() -> Self {
        Self {
            armament_processor: TArmamentProcessor::default(),
            skill_processor: TSkillProcessor::default(),
        }
    }
}

impl<TArmamentProcessor, TSkillProcessor> LeveledCreatureProcessor
    for TemplateProfessionShieldProcessor<TArmamentProcessor, TSkillProcessor>
where
    TArmamentProcessor: LeveledCreatureProcessor + Default,
    TSkillProcessor: LeveledCreatureProcessor + Default,
{
    fn process(&self, world: &mut World, creature_entity: Entity, level: BuildLevel) -> Entity {
        let creature_entity = self
            .armament_processor
            .process(world, creature_entity, level);
        self.skill_processor.process(world, creature_entity, level)
    }
}
