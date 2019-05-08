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
use crate::creatures::factories::qualities::ArmedWeaponProcessor;
use crate::creatures::factories::traits::ZombieProcessor;
use crate::creatures::factories::{
    CreatureFactory, CreatureProcessor, LeveledCreatureProcessor, TemplateCreatureFactory,
};
use crate::factions::Faction;
use crate::game::points::BuildLevel;

#[derive(Clone)]
pub struct ZombieFactory(
    ArmedWeaponProcessor,
    TemplateCreatureFactory,
    ZombieProcessor,
);

impl ZombieFactory {
    fn creature_factory(&self) -> &TemplateCreatureFactory {
        &self.1
    }

    fn generate_leveled_weapon(&self, world: &mut World, creature_entity: Entity) -> Entity {
        self.0.process(
            world,
            creature_entity,
            BuildLevel::from(weapon_level_func()),
        )
    }

    fn apply_zombie(&self, world: &mut World, creature_entity: Entity) -> Entity {
        self.2.process(world, creature_entity)
    }
}

impl Default for ZombieFactory {
    fn default() -> Self {
        Self {
            0: ArmedWeaponProcessor::default(),
            1: TemplateCreatureFactory::new(
                Faction::new(1),
                SpeciesType::Human,
                OriginType::Farmer,
            ),
            2: ZombieProcessor::default(),
        }
    }
}

fn weapon_level_func() -> i32 {
    let chance = thread_rng().gen_range(1, 9);
    if chance <= 1 {
        2
    } else if chance <= 2 {
        1
    } else if chance <= 4 {
        0
    } else {
        -1
    }
}

impl CreatureFactory for ZombieFactory {
    fn create(&self, world: &mut World) -> Entity {
        let creature_entity = self.creature_factory().create(world);

        let creature_entity = self.apply_zombie(world, creature_entity);

        self.generate_leveled_weapon(world, creature_entity)
    }
}
