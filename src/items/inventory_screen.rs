/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use specs::{Component, Entity, World};

// Standard includes.
use std::sync::{Arc, Mutex};

// Internal includes.
use super::{Inventory, Item, TransferItem};
use crate::game::GameState;
use crate::io::{Display, Input};
use crate::screens::{Screen, ScreenPushWrapper, ScreenState};

pub struct InventoryScreen {
    creature: Entity,
    state: ScreenState,
    selected_index: Option<usize>,
}

impl InventoryScreen {
    pub fn new(creature: Entity) -> Self {
        Self {
            creature,
            state: ScreenState::Started,
            selected_index: None,
        }
    }

    fn get_storage_item<T: Clone + Component>(&self, world: &mut World) -> T {
        let mut items = world.write_storage::<T>();
        let item_option = items.get_mut(self.creature);;

        item_option.unwrap().clone()
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
        let inventory: Inventory = self.get_storage_item(world);

        let mutex_display = world.write_resource::<Arc<Mutex<Display>>>();
        let mut display = mutex_display.lock().unwrap();
        display.blit_inventory(world, &inventory);
    }

    fn update(&mut self, world: &mut World, _screen_push_wrapper: &mut ScreenPushWrapper) {
        if world.read_resource::<GameState>().alive() == false {
            self.state = ScreenState::Stopped;
            return;
        }

        let mut inventory: Inventory = self.get_storage_item(world);
        let transfer_item: TransferItem = self.get_storage_item(world);
        if let TransferItem::ToInventory(item) = transfer_item {
            world
                .write_storage::<Item>()
                .get_mut(item)
                .unwrap()
                .owned_mut(true);
            inventory.push(item);

            if let Some(transfer_item) =
                world.write_storage::<TransferItem>().get_mut(self.creature)
            {
                *transfer_item = TransferItem::None;
            }
        }

        {
            let arc_mutex_input = world.read_resource::<Arc<Mutex<Input>>>().clone();
            let mut input = arc_mutex_input.lock().unwrap();
            let ch = input.instance_mut().consume_char();
            if ch == 13 as char {
                self.state = ScreenState::Stopped;
                return;
            };

            self.selected_index = if (ch >= 'a') && (ch <= 'z') {
                let index = (ch as usize) - ('a' as usize);
                if index < inventory.get().len() {
                    Some(index)
                } else {
                    None
                }
            } else if (ch >= 'A') && (ch <= 'Z') {
                let index = ((ch as usize) - ('A' as usize)) + 26;
                if index < inventory.get().len() {
                    Some(index)
                } else {
                    None
                }
            } else {
                None
            };

            if let Some(selected_index) = self.selected_index {
                if let Some(transfer_item) =
                    world.write_storage::<TransferItem>().get_mut(self.creature)
                {
                    let item = inventory.remove(selected_index);
                    world
                        .write_storage::<Item>()
                        .get_mut(item)
                        .unwrap()
                        .owned_mut(false);
                    if TransferItem::Fetch == *transfer_item {
                        *transfer_item = TransferItem::FromInventory(item);
                        self.state = ScreenState::Stopped;
                    }
                    return;
                }
            }
        }
    }

    fn state(&self) -> ScreenState {
        self.state
    }
}
