/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use specs::World;

// Standard includes.
use std::sync::{Arc, Mutex};

// Internal includes.
use super::{InventoryScreen, Screen, ScreenPushWrapper, ScreenState};
use crate::bodies::Body;
use crate::game::GameState;
use crate::io::{Display, Input};
use crate::items::{Inventory, Item};

enum BodyScreenState {
    Viewing,
    SelectingItemFromInventory,
}

pub struct BodyScreen {
    body: Body,
    body_index: Option<usize>,
    body_screen_state: BodyScreenState,
    inventory: Inventory,
    inventory_screen: Arc<Mutex<InventoryScreen>>,
    state: ScreenState,
}

impl BodyScreen {
    pub fn new(body: Body, inventory: Inventory) -> Self {
        Self {
            body,
            body_index: None,
            body_screen_state: BodyScreenState::Viewing,
            inventory: inventory.clone(),
            inventory_screen: Arc::new(Mutex::new(InventoryScreen::new(inventory))),
            state: ScreenState::Started,
        }
    }
}

impl Screen for BodyScreen {
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
            display.blit_body(world.read_storage::<Item>(), &self.body);
        }
    }

    fn update(&mut self, world: &mut World, screen_push_wrapper: &mut ScreenPushWrapper) {
        if world.read_resource::<GameState>().alive() == false {
            self.state = ScreenState::Stopped;
            return;
        }

        match self.body_screen_state {
            BodyScreenState::Viewing => {
                let arc_mutex_input = world.read_resource::<Arc<Mutex<Input>>>();
                let mut input = arc_mutex_input.lock().unwrap();
                let ch = input.instance_mut().consume_char();
                if ch == 13 as char {
                    self.state = ScreenState::Stopped;
                    return;
                };

                self.body_index = if (ch >= 'a') && (ch <= 'z') {
                    Some((ch as usize) - ('a' as usize))
                } else if (ch >= 'A') && (ch <= 'Z') {
                    Some(((ch as usize) - ('A' as usize)) + 26)
                } else {
                    None
                };

                if let Some(body_index) = self.body_index {
                    let mut selected_key: Option<String> = None;
                    for (i, name) in self.body.get().keys().enumerate() {
                        if i == body_index {
                            selected_key = Some(name.clone());
                            break;
                        }
                    }

                    if let Some(selected_key) = selected_key {
                        if let Some(body_slot) = self.body.get().get_mut(&selected_key) {
                            if let Some(item) = body_slot.drop_item() {
                                self.body_index = None;
                                self.inventory.push(item);
                            } else {
                                self.inventory_screen = Arc::new(Mutex::new(InventoryScreen::new(
                                    self.inventory.clone(),
                                )));
                                screen_push_wrapper.push(self.inventory_screen.clone());

                                self.body_screen_state =
                                    BodyScreenState::SelectingItemFromInventory;
                            }
                        } else {
                            panic!("Named body slot disappeared while we were using it!");
                        }
                    }
                }
            }
            BodyScreenState::SelectingItemFromInventory => {
                let inventory_screen = self.inventory_screen.clone();
                let inventory_screen = inventory_screen.lock().unwrap();
                if (inventory_screen.state() == ScreenState::Stopped)
                    || (inventory_screen.state() == ScreenState::Inactive)
                {
                    if let Some(selected_index) = inventory_screen.selected_index() {
                        if let Some(body_index) = self.body_index {
                            if selected_index < self.inventory.get().len() {
                                let item = self.inventory.get().remove(selected_index);

                                let mut selected_key: Option<String> = None;
                                for (i, name) in self.body.get().keys().enumerate() {
                                    if i == body_index {
                                        selected_key = Some(name.clone());
                                        break;
                                    }
                                }

                                if let Some(selected_key) = selected_key {
                                    if let Some(body_slot) = self.body.get().get_mut(&selected_key)
                                    {
                                        if let Some(item) = body_slot.hold_item(item) {
                                            self.inventory.push(item)
                                        }
                                    } else {
                                        panic!(
                                            "Named body slot disappeared while we were using it!"
                                        );
                                    }
                                }
                            }
                        }
                    }

                    self.inventory_screen =
                        Arc::new(Mutex::new(InventoryScreen::new(self.inventory.clone())));

                    self.body_screen_state = BodyScreenState::Viewing;
                    self.body_index = None;
                }
            }
        }
    }

    fn state(&self) -> ScreenState {
        self.state
    }
}
