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
use crate::bodies::Body;
use crate::creatures::factories::traits::UndeadProcessor;
use crate::creatures::factories::{CreatureFactory, CreatureProcessor, TemplateCreatureFactory};
use crate::factions::Faction;
use crate::game::points::BuildLevel;
use crate::items::weapons::factories::specific::LeveledWeaponGenerator;
use crate::items::weapons::factories::WeaponGenerator;
use crate::stats::CreatureStats;

#[derive(Clone)]
pub struct ZombieFactory(
    LeveledWeaponGenerator,
    TemplateCreatureFactory,
    UndeadProcessor,
);

impl ZombieFactory {
    fn creature_factory(&self) -> &TemplateCreatureFactory {
        &self.1
    }

    fn generate_leveled_weapon(&self, world: &mut World) -> Entity {
        self.0.create(world, BuildLevel::from(weapon_level_func()))
    }

    fn apply_undead(&self, world: &mut World, creature_entity: Entity) -> Entity {
        self.2.process(world, creature_entity)
    }
}

impl Default for ZombieFactory {
    fn default() -> Self {
        Self {
            0: LeveledWeaponGenerator::default(),
            1: TemplateCreatureFactory::new(
                Faction::new(1),
                SpeciesType::Human,
                OriginType::Farmer,
            ),
            2: UndeadProcessor::default(),
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

        let creature_entity = self.apply_undead(world, creature_entity);

        {
            *world
                .write_storage::<CreatureStats>()
                .get_mut(creature_entity)
                .unwrap() += CreatureStats::new(8, -4, -4, 0, -4, 0);
        }

        {
            let weapon_entity = self.generate_leveled_weapon(world);

            let mut body_storage = world.write_storage::<Body>();
            let body = body_storage.get_mut(creature_entity).unwrap();
            let mut body_data = body.get();
            let body_slot = body_data.get_mut("Right Palm").unwrap();
            body_slot.hold_item(weapon_entity);
        }

        creature_entity
    }
}
