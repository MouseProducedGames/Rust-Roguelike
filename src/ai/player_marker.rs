/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use specs::{Component, NullStorage};

// Standard includes.

// Internal includes.

#[derive(Default)]
pub struct PlayerMarker;

impl Component for PlayerMarker {
    type Storage = NullStorage<Self>;
}
