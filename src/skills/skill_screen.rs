/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use specs::{Component, Entity, World};

// Standard includes.
use std::sync::{Arc, Mutex};

// Internal includes.
use super::SkillLookup;
use crate::game::GameState;
use crate::io::{Display, Input};
use crate::screens::{Screen, ScreenPushWrapper, ScreenState};

pub struct SkillScreen {
    creature: Entity,
    state: ScreenState,
}

impl SkillScreen {
    pub fn new(creature: Entity) -> Self {
        Self {
            creature,
            state: ScreenState::Started,
        }
    }

    fn _get_storage_item<T: Clone + Component>(&self, world: &mut World) -> T {
        let mut items = world.write_storage::<T>();
        let item_option = items.get_mut(self.creature);;

        item_option.unwrap().clone()
    }
}

impl Screen for SkillScreen {
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
        let skill_storage = world.read_storage::<SkillLookup>();
        let skill_lookup = skill_storage.get(self.creature).unwrap();

        let mutex_display = world.write_resource::<Arc<Mutex<Display>>>();
        let mut display = mutex_display.lock().unwrap();
        display.blit_skills(world, skill_lookup);
    }

    fn update(&mut self, world: &mut World, _screen_push_wrapper: &mut ScreenPushWrapper) {
        if world.read_resource::<GameState>().alive() == false {
            self.state = ScreenState::Stopped;
            return;
        }

        let arc_mutex_input = world.read_resource::<Arc<Mutex<Input>>>().clone();
        let mut input = arc_mutex_input.lock().unwrap();
        let ch = input.instance_mut().consume_char();
        if ch == 13 as char {
            self.state = ScreenState::Stopped;
            return;
        };
    }

    fn state(&self) -> ScreenState {
        self.state
    }
}
