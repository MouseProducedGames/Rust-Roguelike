// External includes
use specs::{ ReadExpect, ReadStorage, System, WriteExpect };

// Internal includes
pub use crate::creature::PlayerMarker;
pub use crate::rrl_math::Position;
pub use crate::world::Tilemap;
pub use super::super::io::Window;

pub struct PlayerDisplaySystem;

impl<'a> System<'a> for PlayerDisplaySystem
{
    type SystemData = (
        ReadExpect< 'a, Tilemap >,
        ReadStorage< 'a, PlayerMarker >,
        ReadStorage< 'a, Position >,
        WriteExpect< 'a, Window >
    );

    fn run( &mut self, ( map, _player_marker, player_pos, window ): Self::SystemData )
    {
        use specs::join::Join;

        let map = map;
        let player_pos = player_pos;
        let mut window = window;

        for player_pos in player_pos.join()
        {
            window.write_map( *player_pos, &map );
        }
    }
}
