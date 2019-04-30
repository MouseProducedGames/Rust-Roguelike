/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use specs::{Builder, Entity, World};

// Standard includes.
use std::sync::{Arc, Mutex};

// Internal includes.
use super::CreatureFactory;
use crate::ai::maslow::MaslowTreeLookup;
use crate::ai::systems::LogicMaslow;
use crate::ai::Command;
use crate::background::{OriginType, SpeciesType};
use crate::bodies::BodyFactory;
use crate::data_types::Name;
use crate::factions::Faction;
use crate::game::combat::MultiAttackPenalty;
use crate::items::Inventory;
use crate::maps::VisibilityMapLookup;
use crate::rrl_math::Position;
use crate::skills::SkillLookup;
use crate::stats::CreatureStats;
use crate::talents::TalentLookup;

#[derive(Clone)]
pub struct TemplateCreatureFactory {
    faction: Faction,
    species_type: SpeciesType,
    origin_type: OriginType,
}

impl TemplateCreatureFactory {
    pub fn new(faction: Faction, species_type: SpeciesType, origin_type: OriginType) -> Self {
        Self {
            faction,
            species_type,
            origin_type,
        }
    }
}

impl CreatureFactory for TemplateCreatureFactory {
    fn create(&self, world: &mut World) -> Entity {
        let maslow_tree_lookup = world
            .read_resource::<Arc<Mutex<MaslowTreeLookup>>>()
            .clone();
        let maslow_tree_lookup = maslow_tree_lookup.lock().unwrap();

        let body = SpeciesType::Human.create_body(world);

        world
            .create_entity()
            .with(body)
            .with(Command::None)
            .with(LogicMaslow)
            .with(CreatureStats::from(self.species_type) + CreatureStats::from(self.origin_type))
            .with(self.faction)
            .with(Inventory::new())
            .with(maslow_tree_lookup.get("Faction/Wander").unwrap().clone())
            .with(MultiAttackPenalty::from(0))
            .with(Name::new(
                format!("{} {}", self.species_type, self.origin_type).as_str(),
            ))
            .with(Position::new(1, 1))
            .with(SkillLookup::new())
            .with(TalentLookup::new())
            .with(VisibilityMapLookup::new())
            .build()
    }
}
