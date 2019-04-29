/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use specs::{Entity, WriteStorage};

// Standard includes.
use std::sync::{Arc, Mutex};

// Internal includes.
use super::MaslowFuncWrapper;
use crate::ai::Command;
use crate::factions::Faction;
use crate::game::EntityPositionTracker;
use crate::items::Inventory;
use crate::maps::{Tilemap, VisibilityMap};
use crate::rrl_math::Position;
use crate::skills::SkillLookup;
use crate::stats::CreatureStats;
use crate::talents::TalentLookup;

#[derive(Clone)]
pub struct MaslowNode {
    name: String,
    call: MaslowFuncWrapper,
    sub_nodes: Vec<Arc<Mutex<MaslowNode>>>,
}

impl MaslowNode {
    pub fn new(name: &str, call: MaslowFuncWrapper, sub_nodes: &[Arc<Mutex<MaslowNode>>]) -> Self {
        Self {
            name: String::from(name),
            call,
            sub_nodes: sub_nodes.to_vec(),
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    #[allow(clippy::too_many_arguments)]
    pub fn call(
        &self,
        creature_stats: &mut CreatureStats,
        entity: Entity,
        entity_position_tracker: &mut EntityPositionTracker,
        factions: &mut WriteStorage<Faction>,
        inventory: &mut Inventory,
        position: &mut Position,
        skills: &mut SkillLookup,
        talents: &mut TalentLookup,
        map: &mut Tilemap,
        visibility_map: &mut VisibilityMap,
    ) -> Option<Command> {
        if let Some(command_value) = (self.call).call(
            creature_stats,
            entity,
            entity_position_tracker,
            factions,
            inventory,
            position,
            skills,
            talents,
            map,
            visibility_map,
        ) {
            return Some(command_value);
        } else {
            for item in self.sub_nodes.iter() {
                if let Some(command_value) = item.lock().unwrap().call(
                    creature_stats,
                    entity,
                    entity_position_tracker,
                    factions,
                    inventory,
                    position,
                    skills,
                    talents,
                    map,
                    visibility_map,
                ) {
                    return Some(command_value);
                }
            }

            None
        }
    }
}
