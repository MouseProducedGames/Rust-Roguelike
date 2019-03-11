use super::creature::Creature;
use super::game_state::GameState;
use super::tilemap::Tilemap;

pub trait CreatureLogic
{
    fn update(&self, target: &mut Creature, map: &Tilemap, game_state: &mut GameState);
}
