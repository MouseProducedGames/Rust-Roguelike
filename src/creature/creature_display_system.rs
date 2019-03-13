// External includes
use specs::{ ReadExpect, ReadStorage, System, WriteExpect };

// Internal includes
pub use crate::creature::PlayerPosition;
pub use crate::rrl_math::Position;
pub use super::super::io::Window;

pub struct CreatureDisplaySystem;

impl<'a> System<'a> for CreatureDisplaySystem
{
    type SystemData = (
        ReadExpect< 'a, PlayerPosition >,
        ReadStorage< 'a, Position >,
        WriteExpect< 'a, Window >
    );

    fn run( &mut self, ( view_pos, pos, window ): Self::SystemData )
    {
        use specs::join::Join;

        let view_pos = view_pos.0;
        let mut window = window;

        for pos in pos.join()
        {
            window.write_creature( *pos, view_pos );
        }
    }
}
