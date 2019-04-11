/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.

// Standard includes.

// Internal includes.
use super::MaslowNode;
use crate::ai::Command;
use crate::game::EntityPositionTracker;
use crate::items::Inventory;
use crate::rrl_math::Position;
use crate::skills::SkillLookup;
use crate::stats::CreatureStats;
use crate::talents::TalentLookup;
use crate::world::{Tilemap, VisibilityMap};

pub struct MaslowTree<'a> {
    name: &'a str,
    sub_nodes: Vec<&'a MaslowNode<'a>>,
}

impl<'a> MaslowTree<'a> {
    pub fn new(name: &'a str, sub_nodes: &[&'a MaslowNode<'a>]) -> Self {
        Self {
            name,
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
    ) -> Command {
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
                return command_value;
            }
        }

        Command::None
    }
}
