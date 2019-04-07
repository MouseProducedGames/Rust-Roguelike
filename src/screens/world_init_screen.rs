/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use specs::World;

// Standard includes.

// Internal includes.
use super::{Screen, ScreenPushWrapper, ScreenState};
use crate::ai::{
    Command, LogicFaction, LogicPlayer, LogicWander, LogicWanderAttack,
    PlayerMarker, ViewpointMarker,
};
use crate::factions::Faction;
use crate::game::{EntityPositionTracker, GameState};
use crate::rrl_math::Position;
use crate::skills::SkillLookup;
use crate::stats::{CreatureStats, SightRange};
use crate::talents::TalentLookup;
use crate::world::Visibility;

pub struct WorldInitScreen {
    state: ScreenState,
}

impl WorldInitScreen {
    pub fn new() -> Self {
        Self {
            state: ScreenState::Started,
        }
    }
}

impl Screen for WorldInitScreen {
    fn init(&mut self) {
        self.state = match self.state {
            ScreenState::Inactive => ScreenState::Inactive,
            ScreenState::Started => ScreenState::Running,
            ScreenState::Running => ScreenState::Running,
            ScreenState::Stopped => ScreenState::Inactive,
        }
    }

    fn close(&mut self) {
        self.state = match self.state {
            ScreenState::Inactive => ScreenState::Inactive,
            ScreenState::Started => ScreenState::Inactive,
            ScreenState::Running => ScreenState::Inactive,
            ScreenState::Stopped => ScreenState::Inactive,
        }
    }

    fn blocks_draw(&self) -> bool {
        true
    }

    fn blocks_update(&self) -> bool {
        true
    }

    fn draw(&mut self, _world: &mut World) {}

    fn update(&mut self, world: &mut World, _screen_push_wrapper: &mut ScreenPushWrapper) {
        world.add_resource(EntityPositionTracker::new());
        world.add_resource(GameState::new());
        world.register::<Command>();
        world.register::<LogicFaction>();
        world.register::<LogicPlayer>();
        world.register::<LogicWander>();
        world.register::<LogicWanderAttack>();
        world.register::<CreatureStats>();
        world.register::<Faction>();
        world.register::<PlayerMarker>();
        world.register::<Position>();
        world.register::<SightRange>();
        world.register::<SkillLookup>();
        world.register::<TalentLookup>();
        world.register::<ViewpointMarker>();
        world.register::<Visibility>();

        self.state = ScreenState::Stopped;
    }

    fn state(&self) -> ScreenState {
        self.state
    }
}
