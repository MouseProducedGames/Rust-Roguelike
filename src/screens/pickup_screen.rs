/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use specs::{Component, Entity, World};

// Standard includes.
use std::sync::{Arc, Mutex};

// Internal includes.
use super::{Screen, ScreenPushWrapper, ScreenState};
use crate::game::GameState;
use crate::io::{Display, Input};
use crate::items::{Inventory, Item};
use crate::rrl_math::Position;

pub struct PickupScreen {
    creature: Entity,
    items_for_pickup: Vec<Entity>,
    state: ScreenState,
    selected_index: Option<usize>,
}

impl PickupScreen {
    pub fn new(creature: Entity) -> Self {
        Self {
            creature,
            items_for_pickup: vec![],
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

impl Screen for PickupScreen {
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
            let creature_position: Position = self.get_storage_item(world);
            self.items_for_pickup.clear();
            let positions = world.read_storage::<Position>();

            use specs::Join;
            for (item_entity, item_data) in
                (&world.entities(), &world.read_storage::<Item>()).join()
            {
                if item_data.owned() == false {
                    if let Some(item_position) = positions.get(item_entity) {
                        let item_position = *item_position;
                        if item_position == creature_position {
                            self.items_for_pickup.push(item_entity);
                        }
                    }
                }
            }
        }

        {
            let mutex_display = world.write_resource::<Arc<Mutex<Display>>>();
            let mut display = mutex_display.lock().unwrap();
            display.blit_items(world.read_storage::<Item>(), &self.items_for_pickup);
        }
    }

    fn update(&mut self, world: &mut World, _screen_push_wrapper: &mut ScreenPushWrapper) {
        if world.read_resource::<GameState>().alive() == false {
            self.state = ScreenState::Stopped;
            return;
        }

        let mut inventory: Inventory = self.get_storage_item(world);

        let arc_mutex_input = world.read_resource::<Arc<Mutex<Input>>>().clone();
        let mut input = arc_mutex_input.lock().unwrap();
        let ch = input.instance_mut().consume_char();
        if ch == 13 as char {
            self.state = ScreenState::Stopped;
            return;
        };

        self.selected_index = if (ch >= 'a') && (ch <= 'z') {
            let index = (ch as usize) - ('a' as usize);
            if index < self.items_for_pickup.len() {
                Some(index)
            } else {
                None
            }
        } else if (ch >= 'A') && (ch <= 'Z') {
            let index = ((ch as usize) - ('A' as usize)) + 26;
            if index < self.items_for_pickup.len() {
                Some(index)
            } else {
                None
            }
        } else {
            None
        };

        if let Some(selected_index) = self.selected_index {
            let item = self.items_for_pickup.remove(selected_index);
            if world
                .write_storage::<Item>()
                .get_mut(item)
                .unwrap()
                .owned_mut(true)
                == false
            {
                inventory.push(item);
            }
        }
    }

    fn state(&self) -> ScreenState {
        self.state
    }
}
