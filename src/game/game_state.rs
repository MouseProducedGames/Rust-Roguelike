/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// use super::faction::FactionLookup;
// use super::io::Window;

pub struct GameState {
    alive: bool,
    // window: Window,
    // factions: FactionLookup,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            alive: true,
            // window: Window::new(),
            // factions: FactionLookup::new(),
        }
    }

    pub fn alive(&self) -> bool {
        self.alive
    }

    /* pub fn factions( &self ) -> &FactionLookup
    {
        &self.factions
    }

    pub fn factions_mut( &mut self ) -> &mut FactionLookup
    {
        &mut self.factions
    } */

    pub fn kill(&mut self) {
        self.alive = false;
    }

    /* pub fn window(&self) -> &Window
    {
        &self.window
    }

    pub fn window_mut(&mut self) -> &mut Window
    {
        return &mut self.window
    } */
}
