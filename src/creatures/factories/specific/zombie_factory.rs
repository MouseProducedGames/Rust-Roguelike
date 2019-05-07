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
use crate::abilities::Undead;
use crate::background::{OriginType, SpeciesType};
use crate::bodies::Body;
use crate::creatures::factories::{CreatureFactory, TemplateCreatureFactory};
use crate::factions::Faction;
use crate::items::weapons::factories::specific::LeveledWeaponFactory;
use crate::items::weapons::factories::WeaponFactory;
use crate::stats::CreatureStats;

#[derive(Clone)]
pub struct ZombieFactory(TemplateCreatureFactory);

impl ZombieFactory {}

impl Default for ZombieFactory {
    fn default() -> Self {
        Self {
            0: TemplateCreatureFactory::new(
                Faction::new(1),
                SpeciesType::Human,
                OriginType::Farmer,
            ),
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
        let creature_entity = self.0.create(world);

        {
            let mut undead_storage = world.write_storage::<Undead>();
            if undead_storage.get(creature_entity).is_none() {
                if let Err(e) = undead_storage.insert(creature_entity, Default::default()) {
                    panic!(e);
                }
            }
        }

        {
            *world
                .write_storage::<CreatureStats>()
                .get_mut(creature_entity)
                .unwrap() += CreatureStats::new(8, -4, -4, 0, -4, 0);
        }

        {
            let weapon_level = weapon_level_func();
            let weapon_entity = LeveledWeaponFactory::new(weapon_level).create(world);

            let mut body_storage = world.write_storage::<Body>();
            let body = body_storage.get_mut(creature_entity).unwrap();
            let mut body_data = body.get();
            let body_slot = body_data.get_mut("Right Palm").unwrap();
            body_slot.hold_item(weapon_entity);
        }

        creature_entity
    }
}
