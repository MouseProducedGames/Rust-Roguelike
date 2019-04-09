/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use specs::{Builder, World};

// Standard includes.
use std::sync::{Arc, Mutex};

// Internal includes.
use super::{
    CharacterCreationScreen, GameScreen, MapInitScreen, Screen, ScreenPushWrapper, ScreenState,
    WorldInitScreen,
};
use crate::ai::{Command, LogicFaction};
use crate::factions::Faction;
use crate::io::{Display, Input};
use crate::rrl_math::Position;
use crate::stats::CreatureStats;
use crate::talents::TalentLookup;
use crate::world::VisibilityMapLookup;

enum StartState {
    SetupDisplay,
    InitializeWorld,
    CharacterCreation,
    MapCreation,
    MonsterCreation,
    StartGame,
    Finished,
}

pub struct StartScreen {
    start_state: StartState,
    state: ScreenState,
}

impl StartScreen {
    pub fn new() -> Self {
        Self {
            start_state: StartState::SetupDisplay,
            state: ScreenState::Started,
        }
    }

    fn create_monsters(&mut self, world: &mut World) {
        world
            .create_entity()
            .with(Command::None)
            .with(LogicFaction)
            .with(Faction::new(0))
            .with(CreatureStats::default())
            .with(Position::new(12, 8))
            .with(TalentLookup::new())
            .with(VisibilityMapLookup::new())
            .build();
    }

    fn setup_display(&mut self, world: &mut World) {
        let display: Arc<Mutex<dyn Display>> =
            Arc::new(Mutex::new(crate::io::console::ConsoleDisplay::new()));

        // Window::init();

        world.add_resource(display);
        world.add_resource(Arc::new(Mutex::new(Input::new())));
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
        self.start_state = match self.start_state {
            StartState::SetupDisplay => {
                self.setup_display(world);

                StartState::InitializeWorld
            }
            StartState::InitializeWorld => {
                let world_init_screen = Arc::new(Mutex::new(WorldInitScreen::new()));
                screen_push_wrapper.push(world_init_screen);

                StartState::CharacterCreation
            }
            StartState::CharacterCreation => {
                let character_creation_screen =
                    Arc::new(Mutex::new(CharacterCreationScreen::new()));
                screen_push_wrapper.push(character_creation_screen);

                StartState::MapCreation
            }
            StartState::MapCreation => {
                let map_init_screen = Arc::new(Mutex::new(MapInitScreen::new()));
                screen_push_wrapper.push(map_init_screen);

                StartState::MonsterCreation
            }
            StartState::MonsterCreation => {
                self.create_monsters(world);

                StartState::StartGame
            }
            StartState::StartGame => {
                let game_screen = Arc::new(Mutex::new(GameScreen::new()));
                screen_push_wrapper.push(game_screen);

                self.state = ScreenState::Stopped;

                StartState::Finished
            }
            StartState::Finished => {
                // This state is a placeholder that exists due to the
                // necessity of returning something from the StartGame
                // match. As such, we should never actually reach it.
                panic!("We should have exited before getting here!");
            }
        }
    }

    fn state(&self) -> ScreenState {
        self.state
    }
}
