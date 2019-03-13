use super::CreatureView;
use super::super::game_state::GameState;
use crate::world::Tilemap;

pub trait CreatureLogic
{
    fn update(&self, target: &mut CreatureView, map: &Tilemap, game_state: &mut GameState);
}
