// External includes
use ncurses;
use specs::{ Component, ReadStorage, System, VecStorage, WriteStorage };

// Internal includes


// use super::CreatureView;
// use super::CreatureLogic;
// use super::super::game_state::GameState;
use crate::rrl_math::{ Displacement, Position };
use crate::world::Tilemap;

pub struct CreatureLogicPlayer {}

impl Component for CreatureLogicPlayer
{
    type Storage = VecStorage<Self>;
}

pub struct CreatureLogicPlayerSystem
{
    pub end_game_signal: bool,
}

impl<'a> System<'a> for CreatureLogicPlayerSystem
{
    type SystemData = (
        WriteStorage< 'a, CreatureLogicPlayer >,
        WriteStorage< 'a, Position >,
        ReadStorage< 'a, Tilemap >
    );

    fn run( &mut self, ( mut logic, mut pos, map ): Self::SystemData)
    {
        use specs::Join;

        for ( _logic, pos, map) in ( &mut logic, &mut pos, &map ).join()
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
                'q' => { target_move = Displacement::new( 0,  0); self.end_game_signal = true; },
                _ =>     target_move = Displacement::new( 0,  0),
            }

            let target_pos = *pos;
            let target_new_pos = target_pos + target_move;

            if map.passable_pos( target_new_pos )
            {
                *pos = *pos + target_move;
            }

            ncurses::mvaddch( pos.y, pos.x, 'C' as u64 );
        }
    }
}


    
