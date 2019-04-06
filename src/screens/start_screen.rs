/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes
use specs::{Builder, World};
use std::sync::{Arc, Mutex};

// Internal includes
use super::screen::ScreenState;
use super::screen_manager::ScreenPushWrapper;
use super::{CharacterCreationScreen, Screen};
use crate::ai::{
    Command, CreatureLogicFaction, CreatureLogicPlayer, CreatureLogicWander,
    CreatureLogicWanderAttack, CreatureTracker, PlayerMarker, ViewpointMarker, Visibility,
};
use crate::factions::Faction;
use crate::game::GameState;
use crate::io::Display;
use crate::rrl_math::Position;
use crate::skills::SkillLookup;
use crate::stats::{CreatureStats, SightRange};
use crate::talents::TalentLookup;

pub struct StartScreen {
    state: ScreenState,
}

impl StartScreen {
    pub fn new() -> Self {
        Self {
            state: ScreenState::Started,
        }
    }
}

impl Screen for StartScreen {
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

    fn update(&mut self, world: &mut World, screen_push_wrapper: &mut ScreenPushWrapper) {
        let display: Arc<Mutex<dyn Display>> =
            Arc::new(Mutex::new(crate::io::console::ConsoleDisplay::new()));

        // Window::init();

        world.add_resource(CreatureTracker::new());
        world.add_resource(GameState::new());
        world.add_resource(display);
        world.register::<Command>();
        world.register::<CreatureLogicFaction>();
        world.register::<CreatureLogicPlayer>();
        world.register::<CreatureLogicWander>();
        world.register::<CreatureLogicWanderAttack>();
        world.register::<CreatureStats>();
        world.register::<Faction>();
        world.register::<PlayerMarker>();
        world.register::<Position>();
        world.register::<SightRange>();
        world.register::<SkillLookup>();
        world.register::<TalentLookup>();
        world.register::<ViewpointMarker>();
        world.register::<Visibility>();

        world
            .create_entity()
            .with(Command::None)
            .with(CreatureLogicFaction)
            .with(Faction::new(0))
            .with(CreatureStats::default())
            .with(Position::new(12, 8))
            .with(SightRange::new(5.0))
            .with(TalentLookup::new())
            .with(Visibility::new())
            .build();

        world
            .create_entity()
            .with(Command::None)
            .with(CreatureLogicFaction)
            .with(Faction::new(1))
            .with(CreatureStats::default())
            .with(Position::new(8, 12))
            .with(SightRange::new(5.0))
            .with(TalentLookup::new())
            .with(Visibility::new())
            .build();

        let map_init_screen = Arc::new(Mutex::new(CharacterCreationScreen::new()));

        screen_push_wrapper.push(map_init_screen);

        self.state = ScreenState::Stopped;
    }

    fn state(&self) -> ScreenState {
        self.state
    }
}
