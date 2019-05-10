/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use specs::{Entity, World};

// Standard includes.
use std::default::Default;

// Internal includes.
use super::{ProfessionShieldProcessor, ProfessionSwordProcessor};
use crate::creatures::factories::LeveledCreatureProcessor;
use crate::game::points::BuildLevel;

#[derive(Clone)]
pub struct ProfessionSwordAndShieldProcessor {
    profession_shield_processor: ProfessionShieldProcessor,
    profession_weapon_processor: ProfessionSwordProcessor,
}

impl ProfessionSwordAndShieldProcessor {}

impl Default for ProfessionSwordAndShieldProcessor {
    fn default() -> Self {
        Self {
            profession_shield_processor: ProfessionShieldProcessor::default(),
            profession_weapon_processor: ProfessionSwordProcessor::default(),
        }
    }
}

impl LeveledCreatureProcessor for ProfessionSwordAndShieldProcessor {
    fn process(&self, world: &mut World, creature_entity: Entity, level: BuildLevel) -> Entity {
        let creature_entity =
            self.profession_shield_processor
                .process(world, creature_entity, level);
        self.profession_weapon_processor
            .process(world, creature_entity, level)
    }
}
