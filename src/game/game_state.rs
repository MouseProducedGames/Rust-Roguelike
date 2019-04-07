/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.

// Standard includes.

// Internal includes.

pub struct GameState {
    alive: bool,
}

impl GameState {
    pub fn new() -> Self {
        Self { alive: true }
    }

    pub fn alive(&self) -> bool {
        self.alive
    }

    pub fn kill(&mut self) {
        self.alive = false;
    }
}
