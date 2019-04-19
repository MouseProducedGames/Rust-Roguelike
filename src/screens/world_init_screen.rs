/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use specs::World;

// Standard includes.
use std::sync::{Arc, Mutex};

// Internal includes.
use super::{Screen, ScreenPushWrapper, ScreenState};
use crate::ai::maslow::MaslowTree;
use crate::ai::systems::{LogicMaslow, LogicPlayer};
use crate::ai::{Command, PlayerMarker, ViewpointMarker};
use crate::events::EventManager;
use crate::factions::Faction;
use crate::game::{EntityPositionTracker, GameState, Time};
use crate::items::{Inventory, Item, LightSource};
use crate::rrl_math::Position;
use crate::skills::SkillLookup;
use crate::stats::{CreatureStats, StatEventHandler};
use crate::talents::TalentLookup;
use crate::themes::ThemeLookup;
use crate::world::{PatternLookup, VisibilityMapLookup};

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
        world.add_resource(Arc::new(Mutex::new(EventManager::new())));
        {
            let event_manager = world.write_resource::<Arc<Mutex<EventManager>>>().clone();
            let mut event_manager = event_manager.lock().unwrap();
            world.add_resource(StatEventHandler::new(&mut event_manager));
        }
        world.add_resource(GameState::new());
        world.add_resource(Arc::new(Mutex::new(PatternLookup::new())));
        world.add_resource(Arc::new(Mutex::new(ThemeLookup::new())));
        world.add_resource(Time::new(0));
        world.register::<Command>();
        world.register::<CreatureStats>();
        world.register::<Faction>();
        world.register::<Inventory>();
        world.register::<Item>();
        world.register::<LightSource>();
        world.register::<LogicMaslow>();
        world.register::<LogicPlayer>();
        world.register::<MaslowTree>();
        world.register::<PlayerMarker>();
        world.register::<Position>();
        world.register::<SkillLookup>();
        world.register::<TalentLookup>();
        world.register::<ViewpointMarker>();
        world.register::<VisibilityMapLookup>();

        self.state = ScreenState::Stopped;
    }

    fn state(&self) -> ScreenState {
        self.state
    }
}
