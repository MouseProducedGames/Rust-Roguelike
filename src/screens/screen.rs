/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes
use specs::World;
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

    fn blocks_draw(&self) -> bool;
    
    fn blocks_update(&self) -> bool;

    fn draw(&mut self, world: &mut World);

    fn update(&mut self, world: &mut World, manager: &mut ScreenPushWrapper);

    fn state(&self) -> ScreenState;
}