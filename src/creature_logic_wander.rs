// External dependencies.
use rand::Rng;

// Internal dependencies.
use super::creature_view::CreatureView;
use super::creature_logic::CreatureLogic;
use super::game_state::GameState;
use super::linear::Displacement;
use super::tilemap::Tilemap;

pub struct CreatureLogicWander
{
}

impl CreatureLogicWander
{
    pub fn new() -> Self { Self {} }
}

impl CreatureLogic for CreatureLogicWander
{
    fn update(&self, target: &mut CreatureView, map: &Tilemap, game_state: &mut GameState)
    {
        let ( map_width, map_height ) = map.bounds();
        let command = game_state.rng().gen_range(1, 10);
        let target_move;
        match command
        {
            1 =>   target_move = Displacement::new(-1,  1),
            2 =>   target_move = Displacement::new( 0,  1),
            3 =>   target_move = Displacement::new( 1,  1),
            4 =>   target_move = Displacement::new(-1,  0),
            5 =>   target_move = Displacement::new( 0,  0),
            6 =>   target_move = Displacement::new( 1,  0),
            7 =>   target_move = Displacement::new(-1, -1),
            8 =>   target_move = Displacement::new( 0, -1),
            9 =>   target_move = Displacement::new( 1, -1),
            _ =>   target_move = Displacement::new( 0,  0),
        }

        let target_pos = target.get_position();
        let target_new_pos = target_pos + target_move;
        let passable;
        if (target_new_pos.x < 0) || (target_new_pos.x as usize >= map_width) ||
            (target_new_pos.y < 0) || (target_new_pos.y as usize >= map_height)
        {
            passable = false;
        }
        else
        {
            passable = map.passable( target_new_pos.x as usize, target_new_pos.y as usize );
        }

        if passable
        {
            target.move_self( target_move.x, target_move.y );
        }
    }
}

