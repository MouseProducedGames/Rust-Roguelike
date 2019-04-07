/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes
use specs::{ReadExpect, ReadStorage, System, WriteExpect};

// Standard includes.
use std::sync::{Arc, Mutex};

// Internal includes.
use super::{PlayerMarker, Visibility};
use crate::io::Display;
use crate::rrl_math::{calculate_hash, Position};
use crate::stats::CreatureStats;
use crate::world::Tilemap;

pub struct PlayerDisplaySystem;

#[derive(SystemData)]
pub struct SystemDataT<'a> {
    map: ReadExpect<'a, Tilemap>,
    player_markers: ReadStorage<'a, PlayerMarker>,
    positions: ReadStorage<'a, Position>,
    visibilities: ReadStorage<'a, Visibility>,
    stats: ReadStorage<'a, CreatureStats>,
    display: WriteExpect<'a, Arc<Mutex<Display>>>,
}

impl<'a> System<'a> for PlayerDisplaySystem {
    type SystemData = SystemDataT<'a>;

    fn run(&mut self, data: Self::SystemData) {
        use specs::join::Join;

        let map = data.map;
        let map_hash = calculate_hash(&*map);
        let mut display = data.display.lock().unwrap();

        for (_, player_pos, visibility, stats) in (
            &data.player_markers,
            &data.positions,
            &data.visibilities,
            &data.stats,
        )
            .join()
        {
            let visibility_lookup = visibility.visibility_lookup();

            let visibility;
            match visibility_lookup.get(&map_hash) {
                Some(vis_map) => visibility = vis_map,
                _ => continue,
            }

            display.write_map(*player_pos, &map, &visibility);
            display.display_stats(*stats);
        }
    }
}
