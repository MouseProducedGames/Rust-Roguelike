// External includes
use specs::{ ReadStorage, System, WriteExpect };

// Internal includes
pub use crate::rrl_math::Position;
pub use super::super::io::Window;

pub struct CreatureDisplaySystem;

impl<'a> System<'a> for CreatureDisplaySystem
{
    type SystemData = (
        WriteExpect<'a, Window>,
        ReadStorage<'a, Position>
    );

    fn run( &mut self, ( mut window, pos ): Self::SystemData )
    {
        use specs::join::Join;

        for pos in pos.join()
        {
            window.write_character( pos );
        }
    }
}
