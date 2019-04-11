/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.

// Standard includes.

// Internal includes.
use crate::ai::Command;
use crate::game::EntityPositionTracker;
use crate::items::Inventory;
use crate::rrl_math::Position;
use crate::skills::SkillLookup;
use crate::stats::CreatureStats;
use crate::talents::TalentLookup;
use crate::world::{Tilemap, VisibilityMap};

pub type MaslowFn = Fn(
    &mut CreatureStats,
    &mut EntityPositionTracker,
    &mut Inventory,
    &mut Position,
    &mut SkillLookup,
    &mut TalentLookup,
    &mut Tilemap,
    &mut VisibilityMap,
) -> Option<Command>;

pub struct MaslowNode<'a> {
    name: &'a str,
    call: &'a MaslowFn,
    sub_nodes: Vec<&'a MaslowNode<'a>>,
}

impl<'a> MaslowNode<'a> {
    pub fn new(name: &'a str, call: &'a MaslowFn, sub_nodes: &[&'a MaslowNode<'a>]) -> Self {
        Self {
            name,
            call,
            sub_nodes: sub_nodes.to_vec(),
        }
    }

    pub fn name(&self) -> &'a str {
        self.name
    }

    #[allow(clippy::too_many_arguments)]
    pub fn call(
        &self,
        creature_stats: &mut CreatureStats,
        entity_position_lookup: &mut EntityPositionTracker,
        inventory: &mut Inventory,
        position: &mut Position,
        skills: &mut SkillLookup,
        talents: &mut TalentLookup,
        map: &mut Tilemap,
        visibility_map: &mut VisibilityMap,
    ) -> Option<Command> {
        if let Some(command_value) = (&*self.call)(
            creature_stats,
            entity_position_lookup,
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
                if let Some(command_value) = item.call(
                    creature_stats,
                    entity_position_lookup,
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
