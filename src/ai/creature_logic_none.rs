d/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.

// Standard includes.

// Internal includes.
use super::{CreatureView, CreatureLogic};
use crate::game::GameState;
use crate::world::Tilemap;

pub struct CreatureLogicNone
{
}

impl CreatureLogicNone
{
    pub fn new() -> Self { Self {} }
}

impl CreatureLogic for CreatureLogicNone
{
    fn update(&self, _target: &mut CreatureView, _map: &Tilemap, _game_state: &mut GameState) {}
}
