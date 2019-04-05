/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes
use std::sync::{Arc, Mutex};

// Internal includes
use super::screen::{Screen, ScreenState};
use crate::io::Display;

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

    pub fn draw(&mut self, display: &Arc<Mutex<Display>>) {
        for screen in self.stack.iter_mut().rev() {
            screen.lock().unwrap().draw(display);

            if screen.lock().unwrap().is_blocker() {
                break;
            }
        }
    }

    pub fn push(&mut self, screen: Arc<Mutex<dyn Screen>>) {
        self.new_screens.push(screen);
    }

    pub fn update(&mut self) {
        for screen in self.stack.iter_mut().rev() {
            screen.lock().unwrap().update(&mut self.new_screens);

            if screen.lock().unwrap().is_blocker() {
                break;
            }
        }

        self.stack
            .retain(|screen| match screen.lock().unwrap().state() {
                ScreenState::Inactive => false,
                _ => true,
            });
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
