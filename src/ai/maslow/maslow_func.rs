/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use specs::{Entity, WriteStorage};

// Standard includes.
use std::marker::Send;

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
