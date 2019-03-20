/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/

// External includes
pub use specs::{ ReadExpect, ReadStorage, System, WriteExpect };

// Internal includes
pub use crate::creature::{ PlayerMarker, Visibility };
pub use crate::io::Display;
pub use crate::rrl_math::{ calculate_hash, Position };
use std::sync::{ Arc, Mutex };
pub use crate::world::Tilemap;

pub struct PlayerDisplaySystem;

#[derive(SystemData)]
pub struct SystemDataT< 'a >
{
    map: ReadExpect< 'a, Tilemap >,
    player_markers: ReadStorage< 'a, PlayerMarker >,
    positions: ReadStorage< 'a, Position >,
    visibilities: ReadStorage< 'a, Visibility >,
    display: WriteExpect< 'a, Arc< Mutex< Display > > >,
}

impl< 'a > System< 'a > for PlayerDisplaySystem
{
    type SystemData = SystemDataT< 'a >;

    fn run( &mut self, data: Self::SystemData )
    {
        use specs::join::Join;

        let map = data.map;
        let map_hash = calculate_hash( &*map );
        let mut display = data.display.lock().unwrap();

        for ( _, player_pos, visibility ) in ( &data.player_markers, &data.positions, &data.visibilities ).join()
        {
            let visibility_lookup = visibility.visibility_lookup();
            
            let visibility;
            match visibility_lookup.get( &map_hash )
            {
                Some( vis_map ) => visibility = vis_map,
                _ => continue,
            }
            
            display.write_map( *player_pos, &map, &visibility );
        }
    }
}
