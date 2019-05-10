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
use crate::creatures::factories::qualities::professions::weapons_and_shields::{
    ProfessionAxeAndShieldProcessor, ProfessionMaceAndShieldProcessor,
    ProfessionSpearAndShieldProcessor, ProfessionSwordAndShieldProcessor,
};
use crate::creatures::factories::LeveledCreatureProcessor;
use crate::game::points::BuildLevel;

#[derive(Clone)]
pub struct ProfessionBanditProcessor {
    axe_and_shield: ProfessionAxeAndShieldProcessor,
    mace_and_shield: ProfessionMaceAndShieldProcessor,
    spear_and_shield: ProfessionSpearAndShieldProcessor,
    sword_and_shield: ProfessionSwordAndShieldProcessor,
}

impl ProfessionBanditProcessor {}

impl Default for ProfessionBanditProcessor {
    fn default() -> Self {
        Self {
            axe_and_shield: ProfessionAxeAndShieldProcessor::default(),
            mace_and_shield: ProfessionMaceAndShieldProcessor::default(),
            spear_and_shield: ProfessionSpearAndShieldProcessor::default(),
            sword_and_shield: ProfessionSwordAndShieldProcessor::default(),
        }
    }
}

impl LeveledCreatureProcessor for ProfessionBanditProcessor {
    fn process(&self, world: &mut World, creature_entity: Entity, level: BuildLevel) -> Entity {
        match thread_rng().gen_range(0, 4) {
            0 => self.axe_and_shield.process(world, creature_entity, level),
            1 => self.mace_and_shield.process(world, creature_entity, level),
            2 => self.spear_and_shield.process(world, creature_entity, level),
            3 => self.sword_and_shield.process(world, creature_entity, level),
            _ => panic!("Random number out of range!"),
        }
    }
}
