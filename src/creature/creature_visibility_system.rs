/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/

// External includes
pub use specs::{ ReadExpect, ReadStorage, System, WriteStorage };

// Internal includescarc
use crate::creature::{ SightRange, Visibility };
use crate::rrl_math::{ calculate_hash, Position };
use crate::world::{ calculate_visibility, Mapping, Tilemap, VisibilityMap };

pub struct CreatureVisibilitySystem;

impl<'a> System<'a> for CreatureVisibilitySystem
{
    type SystemData = (
        ReadExpect< 'a, Tilemap >,
        ReadStorage< 'a, Position >,
        ReadStorage< 'a, SightRange >,
        WriteStorage< 'a, Visibility >
    );

    fn run( &mut self, ( map, pos, sight_range, mut visibility_comp ): Self::SystemData )
    {
        use specs::join::Join;

        let map = map;
        let map_hash = calculate_hash( &*map );

        for ( pos, sight_range, visibility_comp ) in ( &pos, &sight_range, &mut visibility_comp ).join()
        {
            let visibility_lookup = visibility_comp.visibility_lookup_mut();
            
            if visibility_lookup.contains_key( &map_hash ) == false
            {
                let ( map_width, map_height ) = map.bounds();
                visibility_lookup.insert( map_hash, VisibilityMap::new( map_width, map_height )) ;
            }

            let visibility;
            match visibility_lookup.get_mut( &map_hash )
            {
                Some( vis_map ) => visibility = vis_map,
                _ => panic!( "We no longer have the visibility map we just added!" ),
            }

            let sight_range = sight_range.sight_range();

            calculate_visibility( visibility, *pos, sight_range, &map );

        }
    }
}
