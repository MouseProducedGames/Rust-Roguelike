use super::creature_view::CreatureView;
use super::creature_logic::CreatureLogic;
use super::game_state::GameState;
use super::linear::Displacement;
use super::tilemap::Tilemap;

pub struct CreatureLogicPlayer
{
}

impl CreatureLogicPlayer
{
    pub fn new() -> Self { Self {} }
}

impl CreatureLogic for CreatureLogicPlayer
{
    fn update(&self, target: &mut CreatureView, map: &Tilemap, game_state: &mut GameState)
    {
        let ( map_width, map_height ) = map.bounds();
        // get_char refreshes the screen. Why??
        let command = game_state.window().get_char();
        let target_move;
        match command
        {
            '1' =>   target_move = Displacement::new(-1,  1),
            '2' =>   target_move = Displacement::new( 0,  1),
            '3' =>   target_move = Displacement::new( 1,  1),
            '4' =>   target_move = Displacement::new(-1,  0),
            '6' =>   target_move = Displacement::new( 1,  0),
            '7' =>   target_move = Displacement::new(-1, -1),
            '8' =>   target_move = Displacement::new( 0, -1),
            '9' =>   target_move = Displacement::new( 1, -1),
            'q' => { target_move = Displacement::new( 0,  0); game_state.kill(); },
            _ =>     target_move = Displacement::new( 0,  0),
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

    
