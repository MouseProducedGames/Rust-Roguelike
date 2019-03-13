// External includes
use ncurses;
use specs::{ Component, DenseVecStorage, ReadExpect, System, WriteExpect, WriteStorage };

// Internal includes


// use super::CreatureView;
// use super::CreatureLogic;
use super::super::game_state::GameState;
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
        WriteExpect< 'a, GameState >,
        WriteStorage< 'a, CreatureLogicPlayer >,
        WriteStorage< 'a, Position >,
    );

    fn run( &mut self, ( map, mut game_state, mut logic, mut pos ): Self::SystemData)
    {
        use specs::Join;

        let map = map;

        for ( _logic, pos ) in ( &mut logic, &mut pos ).join()
        { 
//             let game_state = logic.game_state;
            
            // get_char refreshes the screen. Why??
            // let command = game_state.window().get_char();
            let command =
                match
                std::char::from_u32(ncurses::getch() as u32)
                {
                    None => ' ',
                    Some(v) => v,
                };
            
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

            // ncurses::mvaddch( pos.y, pos.x, 'C' as u64 );
        }
    }
}


    
