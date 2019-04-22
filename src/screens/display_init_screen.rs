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
use crate::io::{Display, DisplayOption, Input};

enum DisplayInitState {
    ChoosingDisplay,
    Finished,
}

pub struct DisplayInitScreen {
    state: ScreenState,
    display_init_state: DisplayInitState,
}

impl DisplayInitScreen {
    pub fn new() -> Self {
        Self {
            state: ScreenState::Started,
            display_init_state: DisplayInitState::ChoosingDisplay,
        }
    }
}

impl Screen for DisplayInitScreen {
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
        self.display_init_state = match self.display_init_state {
            DisplayInitState::ChoosingDisplay => {
                world.add_resource(Arc::new(Mutex::new(Input::new())));

                {
                    let mut display = crate::io::console::ConsoleDisplay::new();

                    world.add_resource::<Arc<Mutex<dyn Display>>>(
                        match display.choose_display_option(&[DisplayOption::Console]) {
                            DisplayOption::Console => {
                                let display: Arc<Mutex<dyn Display>> =
                                    Arc::new(Mutex::new(crate::io::console::ConsoleDisplay::new()));

                                display
                            }
                        },
                    );
                }

                self.state = ScreenState::Stopped;

                DisplayInitState::Finished
            }
            DisplayInitState::Finished => {
                // This state is a placeholder that exists due to the
                // necessity of returning something from the MapInitState
                // match. As such, we should never actually reach it.
                panic!("We should have exited before getting here!");
            }
        };
    }

    fn state(&self) -> ScreenState {
        self.state
    }
}
