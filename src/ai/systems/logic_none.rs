d/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.

// Standard includes.

// Internal includes.
use crate::ai::{CreatureView, CreatureLogic};
use crate::game::GameState;
use crate::world::Tilemap;

pub struct LogicNone
{
}

impl Logic for LogicNone
{
    fn update(&self, _target: &mut CreatureView, _map: &Tilemap, _game_state: &mut GameState) {}
}
