/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use specs::{Entity, WriteStorage};

// Standard includes.
use std::marker::Send;
use std::sync::{Arc, Mutex};

// Internal includes.
use crate::ai::Command;
use crate::factions::Faction;
use crate::game::EntityPositionTracker;
use crate::items::Inventory;
use crate::rrl_math::Position;
use crate::skills::SkillLookup;
use crate::stats::CreatureStats;
use crate::talents::TalentLookup;
use crate::world::{Tilemap, VisibilityMap};

pub type MaslowFn = Fn(
        &mut CreatureStats,
        Entity,
        &mut EntityPositionTracker,
        &mut WriteStorage<Faction>,
        &mut Inventory,
        &mut Position,
        &mut SkillLookup,
        &mut TalentLookup,
        &mut Tilemap,
        &mut VisibilityMap,
    ) -> Option<Command>
    + Send;

pub struct MaslowNode {
    name: String,
    call: Arc<Mutex<MaslowFn>>,
    sub_nodes: Vec<Arc<Mutex<MaslowNode>>>,
}

impl MaslowNode {
    pub fn new(
        name: &str,
        call: Arc<Mutex<MaslowFn>>,
        sub_nodes: &[Arc<Mutex<MaslowNode>>],
    ) -> Self {
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
        if let Some(command_value) = (&*self.call.lock().unwrap())(
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
