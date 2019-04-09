/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes
use specs::World;

// Standard includes.

// Internal includes
use super::ScreenPushWrapper;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ScreenState {
    Inactive,
    Started,
    Running,
    Stopped,
}

pub trait Screen: Send {
    fn init(&mut self);

    fn close(&mut self);

    fn blocks_draw(&self) -> bool;

    fn blocks_update(&self) -> bool;

    fn draw(&mut self, world: &mut World);

    fn pre_update(&mut self, _world: &mut World) {}

    fn update(&mut self, world: &mut World, screen_push_wrapper: &mut ScreenPushWrapper);

    fn post_update(&mut self, _world: &mut World) {}

    fn state(&self) -> ScreenState;
}
