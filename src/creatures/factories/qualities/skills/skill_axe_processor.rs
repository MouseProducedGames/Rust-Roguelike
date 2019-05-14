/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use specs::{Entity, World};

// Standard includes.
use std::default::Default;

// Internal includes.
use super::SpecifiedWeaponSkillProcessor;
use crate::creatures::factories::LeveledCreatureProcessor;
use crate::game::points::BuildLevel;
use crate::items::weapons::WeaponGroup;

#[derive(Clone)]
pub struct SkillAxeProcessor;

impl SkillAxeProcessor {}

impl Default for SkillAxeProcessor {
    fn default() -> Self {
        Self {}
    }
}

impl LeveledCreatureProcessor for SkillAxeProcessor {
    fn process(&self, world: &mut World, creature_entity: Entity, level: BuildLevel) -> Entity {
        SpecifiedWeaponSkillProcessor::new(WeaponGroup::Axes).process(world, creature_entity, level)
    }
}
