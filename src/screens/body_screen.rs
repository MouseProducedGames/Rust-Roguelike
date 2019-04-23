/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use specs::{Component, Entity, World};

// Standard includes.
use std::sync::{Arc, Mutex};

// Internal includes.
use super::{InventoryScreen, Screen, ScreenPushWrapper, ScreenState};
use crate::bodies::Body;
use crate::data_types::Name;
use crate::game::GameState;
use crate::io::{Display, Input};
use crate::items::{Inventory, Item, TransferItem};

pub struct BodyScreen {
    body_index: Option<usize>,
    creature: Entity,
    state: ScreenState,
}

impl BodyScreen {
    pub fn new(creature: Entity) -> Self {
        Self {
            body_index: None,
            creature,
            state: ScreenState::Started,
        }
    }

    fn get_selected_key(&self, body: &Body, body_index: usize) -> Option<String> {
        let mut selected_key: Option<String> = None;
        for (i, name) in body.get().keys().enumerate() {
            if i == body_index {
                selected_key = Some(name.clone());
                break;
            }
        }

        selected_key
    }

    fn get_storage_item<T: Clone + Component>(&self, world: &mut World) -> T {
        let mut items = world.write_storage::<T>();
        let item_option = items.get_mut(self.creature);;

        item_option.unwrap().clone()
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
        let body: Body = self.get_storage_item(world);

        let mutex_display = world.write_resource::<Arc<Mutex<Display>>>();
        let mut display = mutex_display.lock().unwrap();
        display.blit_body(world.read_storage::<Name>(), &body);
    }

    fn update(&mut self, world: &mut World, screen_push_wrapper: &mut ScreenPushWrapper) {
        if world.read_resource::<GameState>().alive() == false {
            self.state = ScreenState::Stopped;
            return;
        }

        let body: Body = self.get_storage_item(world);

        let transfer_item: TransferItem = self.get_storage_item(world);

        let arc_mutex_input = world.read_resource::<Arc<Mutex<Input>>>().clone();
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
            self.body_index
        };

        if let Some(body_index) = self.body_index {
            if let Some(selected_key) = self.get_selected_key(&body, body_index) {
                if let Some(body_slot) = body.get().get_mut(&selected_key) {
                    match transfer_item {
                        TransferItem::FromInventory(item) => {
                            world
                                .write_storage::<Item>()
                                .get_mut(item)
                                .unwrap()
                                .owned_mut(true);
                            if let Some(item) = body_slot.hold_item(item) {
                                self.get_storage_item::<Inventory>(world).push(item);
                            }

                            self.body_index = None;
                            if let Some(transfer_item) =
                                world.write_storage::<TransferItem>().get_mut(self.creature)
                            {
                                *transfer_item = TransferItem::None;
                            }
                        }
                        TransferItem::None => {
                            if let Some(item) = body_slot.drop_item() {
                                self.get_storage_item::<Inventory>(world).push(item);
                                self.body_index = None;
                            } else {
                                let inventory_screen =
                                    Arc::new(Mutex::new(InventoryScreen::new(self.creature)));
                                screen_push_wrapper.push(inventory_screen);

                                if let Some(transfer_item) =
                                    world.write_storage::<TransferItem>().get_mut(self.creature)
                                {
                                    *transfer_item = TransferItem::Fetch;
                                }
                            }
                        }
                        _ => (),
                    }
                }
            }
        }
    }

    fn state(&self) -> ScreenState {
        self.state
    }
}