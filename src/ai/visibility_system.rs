/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
pub use specs::{Entities, ReadExpect, ReadStorage, System, WriteExpect, WriteStorage};

// Standard includes.

// Internal includes.
use super::{EntityPositionTracker, Visibility};
use crate::rrl_math::{calculate_hash, Position};
use crate::stats::{CreatureStats, SightRange};
use crate::world::{calculate_visibility, Lightmap, Mapping, Tilemap, VisibilityMap};

pub struct VisibilitySystem;

#[derive(SystemData)]
pub struct SystemDataT<'a> {
    entities: Entities<'a>,
    lightmap: WriteExpect<'a, Lightmap>,
    map: WriteExpect<'a, Tilemap>,
    entity_position_tracker: WriteExpect<'a, EntityPositionTracker>,
    stats: WriteStorage<'a, CreatureStats>,
    positions: WriteStorage<'a, Position>,
    sight_ranges: ReadStorage<'a, SightRange>,
    visibilities: WriteStorage<'a, Visibility>,
}

impl<'a> System<'a> for VisibilitySystem {
    type SystemData = SystemDataT<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        use specs::join::Join;

        let entity_position_tracker = &mut data.entity_position_tracker;
        let lightmap = &mut *data.lightmap;
        let map = &mut *data.map;
        let map_hash = calculate_hash(&*map);

        for (entity, stats, pos, sight_range, visibility_comp) in (
            &data.entities,
            &mut data.stats,
            &mut data.positions,
            &data.sight_ranges,
            &mut data.visibilities,
        )
            .join()
        {
            entity_position_tracker.set_position(entity, *pos);

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

            sight_range
                .sight_range()
                .apply(stats, lightmap, pos, map, visibility);
        }

        for (entity, stats, pos, visibility_comp) in (
            &data.entities,
            &mut data.stats,
            &mut data.positions,
            &mut data.visibilities,
        )
            .join()
        {
            entity_position_tracker.set_position(entity, *pos);

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

            calculate_visibility(lightmap, *pos, stats, &map, visibility);
        }
    }
}
