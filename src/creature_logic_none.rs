use super::creature::Creature;
use super::creature_logic::CreatureLogic;
use super::game_state::GameState;
use super::tilemap::Tilemap;

pub struct CreatureLogicNone
{
}

impl CreatureLogicNone
{
    pub fn new() -> Self { Self {} }
}

impl CreatureLogic for CreatureLogicNone
{
    fn update(&self, _target: &mut Creature, _map: &Tilemap, _game_state: &mut GameState) {}
}
