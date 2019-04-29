/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use specs::{Entity, WriteStorage};

// Standard includes.

// Internal includes.
use crate::ai::Command;
use crate::factions::Faction;
use crate::game::EntityPositionTracker;
use crate::items::Inventory;
use crate::maps::{Tilemap, VisibilityMap};
use crate::rrl_math::{Displacement, Position};
use crate::skills::SkillLookup;
use crate::stats::CreatureStats;
use crate::talents::TalentLookup;

#[allow(clippy::too_many_arguments)]
pub fn faction_reaction(
    _creature_stats: &mut CreatureStats,
    entity: Entity,
    entity_position_tracker: &mut EntityPositionTracker,
    factions: &mut WriteStorage<Faction>,
    _inventory: &mut Inventory,
    position: &mut Position,
    _skills: &mut SkillLookup,
    _talents: &mut TalentLookup,
    _map: &mut Tilemap,
    visibility_map: &mut VisibilityMap,
) -> Option<Command> {
    let target_move;
    if let Some(faction) = factions.get(entity) {
        if let Some((_enemy, enemy_pos)) =
            entity_position_tracker.get_nearest_enemy(*faction, &*factions, visibility_map)
        {
            let disp = enemy_pos - *position;
            target_move = Displacement::new(disp.x.signum(), disp.y.signum());
            return Some(Command::Move(target_move));
        } else if let Some((_friend, friend_pos)) =
            entity_position_tracker.get_nearest_friend(entity, *faction, &*factions, visibility_map)
        {
            let disp = friend_pos - *position;
            target_move = Displacement::new(disp.x.signum(), disp.y.signum());
            return Some(Command::Move(target_move));
        }
    }

    None
}
