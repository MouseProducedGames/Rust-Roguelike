/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use specs::{Entity, World};

// Standard includes.
use std::default::Default;

// Internal includes.
use crate::background::{OriginType, SpeciesType};
use crate::creatures::factories::{CreatureFactory, TemplateCreatureFactory};
use crate::factions::Faction;

#[derive(Clone)]
pub struct BanditFactory(TemplateCreatureFactory);

impl BanditFactory {}

impl Default for BanditFactory {
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

impl CreatureFactory for BanditFactory {
    fn create(&self, world: &mut World) -> Entity {
        self.0.create(world)
    }
}
