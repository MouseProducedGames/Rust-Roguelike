/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes
use specs::{ReadExpect, ReadStorage, System, WriteExpect, WriteStorage};

// Standard includes.
use std::sync::{Arc, Mutex};

// Internal includes.
use crate::ai::PlayerMarker;
use crate::io::Display;
use crate::rrl_math::Position;
use crate::stats::CreatureStats;
use crate::world::{Tilemap, VisibilityMapLookup};

pub struct PlayerDisplaySystem;

#[derive(SystemData)]
pub struct SystemDataT<'a> {
    map: ReadExpect<'a, Tilemap>,
    player_markers: ReadStorage<'a, PlayerMarker>,
    positions: ReadStorage<'a, Position>,
    visibility_map_lookup: WriteStorage<'a, VisibilityMapLookup>,
    stats: ReadStorage<'a, CreatureStats>,
    display: WriteExpect<'a, Arc<Mutex<Display>>>,
}

impl<'a> System<'a> for PlayerDisplaySystem {
    type SystemData = SystemDataT<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        use specs::join::Join;

        let map = &*data.map;
        let mut display = data.display.lock().unwrap();

        for (_, player_pos, visibility_map_lookup, stats) in (
            &data.player_markers,
            &data.positions,
            &mut data.visibility_map_lookup,
            &data.stats,
        )
            .join()
        {
            let visibility_map = visibility_map_lookup.get_or_add(map);

            display.write_map(*player_pos, &map, &visibility_map);
            display.display_stats(*stats);
        }
    }
}
