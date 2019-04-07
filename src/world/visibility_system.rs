/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
pub use specs::{Entities, ReadExpect, ReadStorage, System, WriteExpect, WriteStorage};

// Standard includes.

// Internal includes.
use super::VisibilityMapLookup;
use crate::game::EntityPositionTracker;
use crate::items::Inventory;
use crate::rrl_math::Position;
use crate::stats::CreatureStats;
use crate::world::{calculate_visibility, Lightmap, Tilemap};

pub struct VisibilitySystem;

#[derive(SystemData)]
pub struct SystemDataT<'a> {
    entities: Entities<'a>,
    lightmap: WriteExpect<'a, Lightmap>,
    map: WriteExpect<'a, Tilemap>,
    entity_position_tracker: WriteExpect<'a, EntityPositionTracker>,
    inventory: WriteStorage<'a, Inventory>,
    stats: WriteStorage<'a, CreatureStats>,
    positions: WriteStorage<'a, Position>,
    visibility_map_lookup: WriteStorage<'a, VisibilityMapLookup>,
}

impl<'a> System<'a> for VisibilitySystem {
    type SystemData = SystemDataT<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        use specs::join::Join;

        let entity_position_tracker = &mut data.entity_position_tracker;
        let lightmap = &mut *data.lightmap;
        let map = &mut *data.map;

        for (entity, stats, pos, visibility_map_lookup, inventory) in (
            &data.entities,
            &mut data.stats,
            &mut data.positions,
            &mut data.visibility_map_lookup,
            &data.inventory,
        )
            .join()
        {
            entity_position_tracker.set_position(entity, *pos);

            let visibility_map = visibility_map_lookup.get_or_add_mut(map);

            for item in inventory.get().iter() {
                item.apply(stats, lightmap, pos, map, visibility_map);
            }
        }

        for (entity, stats, pos, visibility_map_lookup) in (
            &data.entities,
            &mut data.stats,
            &mut data.positions,
            &mut data.visibility_map_lookup,
        )
            .join()
        {
            entity_position_tracker.set_position(entity, *pos);

            let visibility_map = visibility_map_lookup.get_or_add_mut(map);

            calculate_visibility(lightmap, *pos, stats, &map, visibility_map);
        }
    }
}
