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
use crate::background::{OriginType, SpeciesType};
use crate::creatures::factories::qualities::professions::ProfessionBanditProcessor;
use crate::creatures::factories::{
    CreatureFactory, LeveledCreatureProcessor, TemplateCreatureFactory,
};
use crate::factions::Faction;
use crate::game::points::BuildLevel;

#[derive(Clone)]
pub struct BanditFactory(ProfessionBanditProcessor, TemplateCreatureFactory);

impl BanditFactory {
    fn apply_profession(&self, creature_entity: Entity, world: &mut World) -> Entity {
        self.0
            .process(world, creature_entity, creature_level_func())
    }

    fn creature_factory(&self) -> &TemplateCreatureFactory {
        &self.1
    }
}

impl Default for BanditFactory {
    fn default() -> Self {
        Self {
            0: ProfessionBanditProcessor::default(),
            1: TemplateCreatureFactory::new(
                Faction::new(1),
                SpeciesType::Human,
                OriginType::Farmer,
            ),
        }
    }
}

fn creature_level_func() -> BuildLevel {
    let chance = thread_rng().gen_range(1, 9);
    BuildLevel::from(if chance <= 1 {
        2
    } else if chance <= 2 {
        1
    } else if chance <= 4 {
        0
    } else {
        -1
    })
}

impl CreatureFactory for BanditFactory {
    fn create(&self, world: &mut World) -> Entity {
        let creature_entity = self.creature_factory().create(world);
        self.apply_profession(creature_entity, world)
    }
}
