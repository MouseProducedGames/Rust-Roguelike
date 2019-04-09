/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use specs::World;

// Standard includes.
use std::sync::{Arc, Mutex};

// Internal includes.
use super::{Screen, ScreenState};

pub struct ScreenManager {
    stack: Vec<Arc<Mutex<dyn Screen>>>,
    new_screens: Vec<Arc<Mutex<dyn Screen>>>,
}

impl ScreenManager {
    pub fn new() -> Self {
        Self {
            stack: vec![],
            new_screens: vec![],
        }
    }

    pub fn draw(&mut self, world: &mut World) {
        for screen in self.stack.iter_mut().rev() {
            let mut screen = screen.lock().unwrap();
            screen.draw(world);

            if screen.blocks_draw() {
                break;
            }
        }
    }

    pub fn push(&mut self, screen: Arc<Mutex<dyn Screen>>) {
        self.new_screens.push(screen);
    }

    pub fn screen_count(&self) -> usize {
        self.stack.len() + self.new_screens.len()
    }

    pub fn update(&mut self, world: &mut World) {
        for screen in self.stack.iter_mut().rev() {
            screen.lock().unwrap().pre_update(world);
        }

        for screen in self.stack.iter_mut().rev() {
            let mut screen = screen.lock().unwrap();
            screen.update(world, &mut self.new_screens);

            if screen.state() == ScreenState::Stopped {
                screen.close();
            } else if screen.blocks_update() {
                break;
            }
        }

        self.stack
            .retain(|screen| match screen.lock().unwrap().state() {
                ScreenState::Inactive => false,
                _ => true,
            });

        for screen in self.stack.iter_mut().rev() {
            screen.lock().unwrap().post_update(world);
        }
    }

    pub fn update_start(&mut self) {
        for screen in self.new_screens.iter_mut() {
            screen.lock().unwrap().init();
        }
        self.stack.append(&mut self.new_screens);
    }
}

pub trait ScreenPushWrapper {
    fn push(&mut self, screen: Arc<Mutex<dyn Screen>>);
}

impl ScreenPushWrapper for Vec<Arc<Mutex<dyn Screen>>> {
    fn push(&mut self, screen: Arc<Mutex<dyn Screen>>) {
        Vec::push(self, screen);
    }
}
