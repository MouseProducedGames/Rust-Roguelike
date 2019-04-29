/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use specs::{Entity, WriteStorage};

// Standard includes.

// Internal includes.
use crate::ai::Command;
use crate::dice::get_random_move;
use crate::factions::Faction;
use crate::game::EntityPositionTracker;
use crate::items::Inventory;
use crate::maps::{Tilemap, VisibilityMap};
use crate::rrl_math::Position;
use crate::skills::SkillLookup;
use crate::stats::CreatureStats;
use crate::talents::TalentLookup;

#[allow(clippy::too_many_arguments)]
pub fn random_wander(
    _creature_stats: &mut CreatureStats,
    _entity: Entity,
    _entity_position_tracker: &mut EntityPositionTracker,
    _factions: &mut WriteStorage<Faction>,
    _inventory: &mut Inventory,
    _position: &mut Position,
    _skills: &mut SkillLookup,
    _talents: &mut TalentLookup,
    _map: &mut Tilemap,
    _visibility_map: &mut VisibilityMap,
) -> Option<Command> {
    let target_move = get_random_move();

    Some(Command::Move(target_move))
}
