/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/

// External includes
use specs::{ ReadExpect, ReadStorage, System, WriteExpect };
use std::sync::{ Arc, Mutex };

// Internal includes
pub use crate::creature::PlayerPosition;
pub use crate::rrl_math::Position;
pub use crate::io::Display;

pub struct CreatureDisplaySystem;

impl<'a> System<'a> for CreatureDisplaySystem
{
    type SystemData = (
        ReadExpect< 'a, PlayerPosition >,
        ReadStorage< 'a, Position >,
        WriteExpect< 'a, Arc< Mutex< Display > > >
    );

    fn run( &mut self, ( view_pos, pos, window ): Self::SystemData )
    {
        use specs::join::Join;

        let view_pos = view_pos.0;
        let mut window = ( window ).lock().unwrap();

        for pos in pos.join()
        {
            window.write_creature( *pos, view_pos );
        }
    }
}
