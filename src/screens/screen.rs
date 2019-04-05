/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes
use std::sync::{Arc, Mutex};

// Internal includes
use super::screen_manager::ScreenPushWrapper;
use crate::io::Display;

#[derive(Copy, Clone)]
pub enum ScreenState {
    Inactive,
    Started,
    Running,
    Stopped,
}

pub trait Screen {
    fn init(&mut self);

    fn close(&mut self);

    fn is_blocker(&self) -> bool;

    fn draw(&mut self, display: &Arc<Mutex<Display>>);

    fn update(&mut self, manager: &mut ScreenPushWrapper);

    fn state(&self) -> ScreenState;
}
