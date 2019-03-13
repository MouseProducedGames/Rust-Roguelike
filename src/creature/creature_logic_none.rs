use super::CreatureView;
use super::CreatureLogic;
use super::super::game_state::GameState;
use super::super::tilemap::Tilemap;

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
