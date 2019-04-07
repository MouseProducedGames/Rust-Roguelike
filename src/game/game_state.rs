/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.

// Standard includes.
use std::sync::{Arc, Mutex, MutexGuard};

// Internal includes.
use crate::screens::Screen;

type ScreenRef = Arc<Mutex<Screen>>;
type VecRef<T> = Arc<Mutex<Vec<T>>>;

pub struct GameState {
    alive: bool,
    new_screens: VecRef<ScreenRef>,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            alive: true,
            new_screens: Arc::new(Mutex::new(vec![])),
        }
    }

    pub fn alive(&self) -> bool {
        self.alive
    }

    pub fn lock_new_screens(&mut self) -> MutexGuard<Vec<ScreenRef>> {
        self.new_screens.lock().unwrap()
    }

    pub fn kill(&mut self) {
        self.alive = false;
    }
}
