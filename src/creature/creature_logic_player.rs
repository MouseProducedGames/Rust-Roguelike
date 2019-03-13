// External includes
use specs::{ Component, DenseVecStorage, ReadExpect, ReadStorage, System, WriteExpect, WriteStorage };

// Internal includes
use super::super::game_state::GameState;
use super::super::io::Window;
use crate::creature::{ PlayerMarker, PlayerPosition };
use crate::rrl_math::{ Displacement, Position };
use crate::world::Tilemap;

pub struct CreatureLogicPlayer {}

impl Component for CreatureLogicPlayer
{
    type Storage = DenseVecStorage<Self>;
}

pub struct CreatureLogicPlayerSystem;

impl<'a> System<'a> for CreatureLogicPlayerSystem
{
    type SystemData = (
        ReadExpect< 'a, Tilemap >,
        ReadExpect< 'a, Window >,
        WriteExpect< 'a, GameState >,
        WriteExpect< 'a, PlayerPosition >,
        ReadStorage< 'a, PlayerMarker >,
        WriteStorage< 'a, CreatureLogicPlayer >,
        WriteStorage< 'a, Position >,
    );

    fn run( &mut self, ( map, window, game_state, player_pos, _player_marker, mut logic, mut pos ): Self::SystemData)
    {
        use specs::Join;

        let mut game_state = game_state;
        let map = map;
        let mut player_pos = player_pos;
        let window = window;

        for ( _logic, pos ) in ( &mut logic, &mut pos ).join()
        { 
//             let game_state = logic.game_state;
            
            // get_char refreshes the screen. Why??
            // let command = game_state.window().get_char();
            let command = window.get_char();
                
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

            let target_pos = *pos;
            let target_new_pos = target_pos + target_move;

            if map.passable_pos( target_new_pos )
            {
                *pos = *pos + target_move;
            }

            player_pos.0 = *pos;
        }
    }
}
