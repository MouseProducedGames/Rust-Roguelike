/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

 **/
// External dependencies.
use specs::{Entities, ReadStorage, System, WriteExpect, WriteStorage};

// Internal dependencies.
use crate::creature::{CreatureStats, CreatureTracker, PlayerPosition, ViewpointMarker};
use crate::rrl_math::Position;
use crate::stats::Stat;

pub struct CreatureLastUpdateSystem;

#[derive(SystemData)]
pub struct SystemDataT<'a> {
    entities: Entities<'a>,
    creature_tracker: WriteExpect<'a, CreatureTracker>,
    player_pos: WriteExpect<'a, PlayerPosition>,
    viewpoint: ReadStorage<'a, ViewpointMarker>,
    stats: WriteStorage<'a, CreatureStats>,
    pos: WriteStorage<'a, Position>,
}

impl<'a> System<'a> for CreatureLastUpdateSystem {
    type SystemData = SystemDataT<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        use specs::join::Join;

        let mut creature_tracker = data.creature_tracker;

        for (entity, stats) in (&data.entities, &data.stats).join() {
            if stats.health().value() <= 0 {
                creature_tracker.remove_entity(entity);

                match data.entities.delete(entity) {
                    Ok(_) => (),
                    _ => panic!(" Could not delete entity which has to exist!"),
                }
            }
        }

        for (pos, _) in (&data.pos, &data.viewpoint).join() {
            data.player_pos.0 = *pos;
        }
    }
}
