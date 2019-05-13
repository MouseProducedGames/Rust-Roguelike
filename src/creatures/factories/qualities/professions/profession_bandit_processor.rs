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
use crate::creatures::factories::qualities::professions::single_weapons::{
    ProfessionAxeProcessor, ProfessionMaceProcessor, ProfessionSpearProcessor,
    ProfessionSwordProcessor,
};
use crate::creatures::factories::qualities::professions::weapons_and_shields::{
    ProfessionAxeAndShieldProcessor, ProfessionMaceAndShieldProcessor,
    ProfessionSpearAndShieldProcessor, ProfessionSwordAndShieldProcessor,
};
use crate::creatures::factories::LeveledCreatureProcessor;
use crate::game::points::BuildLevel;

#[derive(Clone)]
pub struct ProfessionBanditProcessor {
    axe: ProfessionAxeProcessor,
    mace: ProfessionMaceProcessor,
    spear: ProfessionSpearProcessor,
    sword: ProfessionSwordProcessor,
    axe_and_shield: ProfessionAxeAndShieldProcessor,
    mace_and_shield: ProfessionMaceAndShieldProcessor,
    spear_and_shield: ProfessionSpearAndShieldProcessor,
    sword_and_shield: ProfessionSwordAndShieldProcessor,
}

impl ProfessionBanditProcessor {}

impl Default for ProfessionBanditProcessor {
    fn default() -> Self {
        Self {
            axe: ProfessionAxeProcessor::default(),
            mace: ProfessionMaceProcessor::default(),
            spear: ProfessionSpearProcessor::default(),
            sword: ProfessionSwordProcessor::default(),
            axe_and_shield: ProfessionAxeAndShieldProcessor::default(),
            mace_and_shield: ProfessionMaceAndShieldProcessor::default(),
            spear_and_shield: ProfessionSpearAndShieldProcessor::default(),
            sword_and_shield: ProfessionSwordAndShieldProcessor::default(),
        }
    }
}

impl LeveledCreatureProcessor for ProfessionBanditProcessor {
    fn process(&self, world: &mut World, creature_entity: Entity, level: BuildLevel) -> Entity {
        match thread_rng().gen_range(0, 8) {
            0 => self
                .axe
                .process(world, creature_entity, level + BuildLevel::new(1)),
            1 => self
                .mace
                .process(world, creature_entity, level + BuildLevel::new(1)),
            2 => self
                .spear
                .process(world, creature_entity, level + BuildLevel::new(1)),
            3 => self
                .sword
                .process(world, creature_entity, level + BuildLevel::new(1)),
            4 => self.axe_and_shield.process(world, creature_entity, level),
            5 => self.mace_and_shield.process(world, creature_entity, level),
            6 => self.spear_and_shield.process(world, creature_entity, level),
            7 => self.sword_and_shield.process(world, creature_entity, level),
            _ => panic!("Random number out of range!"),
        }
    }
}
