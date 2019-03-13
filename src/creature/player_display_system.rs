// External includes
pub use specs::{ ReadExpect, ReadStorage, System, WriteExpect };

// Internal includes
pub use crate::creature::{ PlayerMarker, Visibility };
pub use crate::rrl_math::{ calculate_hash, Position };
pub use crate::world::Tilemap;
pub use super::super::io::Window;

pub struct PlayerDisplaySystem;

impl<'a> System<'a> for PlayerDisplaySystem
{
    type SystemData = (
        ReadExpect< 'a, Tilemap >,
        ReadStorage< 'a, PlayerMarker >,
        ReadStorage< 'a, Position >,
        ReadStorage< 'a, Visibility >,
        WriteExpect< 'a, Window >,
    );

    fn run( &mut self, ( map, _player_marker, player_pos, visibility, window ): Self::SystemData )
    {
        use specs::join::Join;

        let map = map;
        let map_hash = calculate_hash( &*map );
        let mut window = window;

        for ( player_pos, visibility ) in ( &player_pos, &visibility ).join()
        {
            let visibility_lookup = visibility.visibility_lookup();
            
            let visibility;
            match visibility_lookup.get( &map_hash )
            {
                Some( vis_map ) => visibility = vis_map,
                _ => continue,
            }
            
            window.write_map( *player_pos, &map, &visibility );
        }
    }
}
