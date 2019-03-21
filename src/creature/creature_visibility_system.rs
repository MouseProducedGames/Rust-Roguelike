/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes
pub use specs::{Entities, ReadExpect, ReadStorage, System, WriteExpect, WriteStorage};

// Internal includescarc
use crate::creature::{CreatureTracker, SightRange, Visibility};
use crate::rrl_math::{calculate_hash, Position};
use crate::world::{calculate_visibility, Mapping, Tilemap, VisibilityMap};

pub struct CreatureVisibilitySystem;

#[derive(SystemData)]
pub struct SystemDataT<'a> {
    entities: Entities<'a>,
    map: ReadExpect<'a, Tilemap>,
    creature_tracker: WriteExpect<'a, CreatureTracker>,
    positions: ReadStorage<'a, Position>,
    sight_ranges: ReadStorage<'a, SightRange>,
    visibilities: WriteStorage<'a, Visibility>,
}

impl<'a> System<'a> for CreatureVisibilitySystem {
    type SystemData = SystemDataT<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        use specs::join::Join;

        let creature_tracker = &mut data.creature_tracker;
        let map = data.map;
        let map_hash = calculate_hash(&*map);

        for (entity, pos, sight_range, visibility_comp) in (
            &data.entities,
            &data.positions,
            &data.sight_ranges,
            &mut data.visibilities,
        )
            .join()
        {
            creature_tracker.set_position(entity, *pos);

            let visibility_lookup = visibility_comp.visibility_lookup_mut();

            if visibility_lookup.contains_key(&map_hash) == false {
                let (map_width, map_height) = map.bounds();
                visibility_lookup.insert(map_hash, VisibilityMap::new(map_width, map_height));
            }

            let visibility;
            match visibility_lookup.get_mut(&map_hash) {
                Some(vis_map) => visibility = vis_map,
                _ => panic!("We no longer have the visibility map we just added!"),
            }

            let sight_range = sight_range.sight_range();

            calculate_visibility(visibility, *pos, sight_range, &map);
        }
    }
}
