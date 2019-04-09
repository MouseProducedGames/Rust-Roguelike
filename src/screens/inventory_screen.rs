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
use crate::game::GameState;
use crate::io::{Display, Input};
use crate::items::Inventory;

pub struct InventoryScreen {
    inventory: Inventory,
    state: ScreenState,
}

impl InventoryScreen {
    pub fn new(inventory: Inventory) -> Self {
        Self {
            inventory,
            state: ScreenState::Started,
        }
    }
}

impl Screen for InventoryScreen {
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
        false
    }

    fn draw(&mut self, world: &mut World) {
        {
            let mutex_display = world.write_resource::<Arc<Mutex<Display>>>();
            let mut display = mutex_display.lock().unwrap();
            display.blit_inventory(&self.inventory);
        }
    }

    fn update(&mut self, world: &mut World, _screen_push_wrapper: &mut ScreenPushWrapper) {
        if world.read_resource::<GameState>().alive() == false {
            self.state = ScreenState::Stopped;
            return;
        }

        {
            let arc_mutex_input = world.read_resource::<Arc<Mutex<Input>>>();
            let input = arc_mutex_input.lock().unwrap();
            let ch = input.instance().get_char();
            if ch == 13 as char {
                self.state = ScreenState::Stopped;
            }
        }
    }

    fn state(&self) -> ScreenState {
        self.state
    }
}
