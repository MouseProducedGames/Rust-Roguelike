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
pub struct TemplateProfessionWeaponAndShieldProcessor<
    TProfessionShieldProcessor,
    TProfessionWeaponProcessor,
> where
    TProfessionShieldProcessor: LeveledCreatureProcessor + Default,
    TProfessionWeaponProcessor: LeveledCreatureProcessor + Default,
{
    profession_shield_processor: TProfessionShieldProcessor,
    profession_weapon_processor: TProfessionWeaponProcessor,
}

impl<TProfessionShieldProcessor, TProfessionWeaponProcessor>
    TemplateProfessionWeaponAndShieldProcessor<
        TProfessionShieldProcessor,
        TProfessionWeaponProcessor,
    >
where
    TProfessionShieldProcessor: LeveledCreatureProcessor + Default,
    TProfessionWeaponProcessor: LeveledCreatureProcessor + Default,
{
}

impl<TProfessionShieldProcessor, TProfessionWeaponProcessor> Default
    for TemplateProfessionWeaponAndShieldProcessor<
        TProfessionShieldProcessor,
        TProfessionWeaponProcessor,
    >
where
    TProfessionShieldProcessor: LeveledCreatureProcessor + Default,
    TProfessionWeaponProcessor: LeveledCreatureProcessor + Default,
{
    fn default() -> Self {
        Self {
            profession_shield_processor: TProfessionShieldProcessor::default(),
            profession_weapon_processor: TProfessionWeaponProcessor::default(),
        }
    }
}

impl<TProfessionShieldProcessor, TProfessionWeaponProcessor> LeveledCreatureProcessor
    for TemplateProfessionWeaponAndShieldProcessor<
        TProfessionShieldProcessor,
        TProfessionWeaponProcessor,
    >
where
    TProfessionShieldProcessor: LeveledCreatureProcessor + Default,
    TProfessionWeaponProcessor: LeveledCreatureProcessor + Default,
{
    fn process(&self, world: &mut World, creature_entity: Entity, level: BuildLevel) -> Entity {
        let creature_entity =
            self.profession_shield_processor
                .process(world, creature_entity, level);
        self.profession_weapon_processor
            .process(world, creature_entity, level)
    }
}
