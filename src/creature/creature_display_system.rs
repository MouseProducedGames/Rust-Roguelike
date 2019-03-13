// External includes
use specs::{ ReadStorage, System };

// Internal includes
pub use crate::rrl_math::Position;
pub use super::super::io::Window;

pub struct CreatureDisplaySystem<'a>
{
    pub window: &'a mut Window,
}

impl<'a> System<'a> for CreatureDisplaySystem<'a>
{
    type SystemData = ReadStorage<'a, Position>;

    fn run( &mut self, pos: Self::SystemData )
    {
        use specs::join::Join;
        
        for pos in pos.join()
        {
            self.window.write_character( pos );
        }
    }
}
