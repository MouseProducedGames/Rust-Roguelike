use super::creature_view::CreatureView;
use super::game_state::GameState;
use super::tilemap::Tilemap;

pub trait CreatureLogic
{
    fn update(&self, target: &mut CreatureView, map: &Tilemap, game_state: &mut GameState);
}
