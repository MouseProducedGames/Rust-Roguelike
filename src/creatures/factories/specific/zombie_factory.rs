/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use specs::{Entity, World};

// Standard includes.
use std::default::Default;

// Internal includes.
use crate::abilities::Undead;
use crate::background::{OriginType, SpeciesType};
use crate::creatures::factories::{CreatureFactory, TemplateCreatureFactory};
use crate::factions::Faction;
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

impl CreatureFactory for ZombieFactory {
    fn create(&self, world: &mut World) -> Entity {
        let creature_entity = self.0.create(world);
        let mut undead_storage = world.write_storage::<Undead>();
        if undead_storage.get(creature_entity).is_none() {
            if let Err(e) = undead_storage.insert(creature_entity, Default::default()) {
                panic!(e);
            }
        }

        *world
            .write_storage::<CreatureStats>()
            .get_mut(creature_entity)
            .unwrap() += CreatureStats::new(8, -4, -4, 0, -4, 0);

        creature_entity
    }
}
