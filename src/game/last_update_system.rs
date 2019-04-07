/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

 **/
// External includes.
use specs::{Entities, ReadStorage, System, WriteExpect, WriteStorage};

// Standard includes.

// Internal includes.
use super::EntityPositionTracker;
use crate::ai::{PlayerPosition, ViewpointMarker};
use crate::rrl_math::Position;
use crate::stats::{CreatureStats, Stat};

pub struct LastUpdateSystem;

#[derive(SystemData)]
pub struct SystemDataT<'a> {
    entities: Entities<'a>,
    entity_position_tracker: WriteExpect<'a, EntityPositionTracker>,
    player_pos: WriteExpect<'a, PlayerPosition>,
    viewpoint: ReadStorage<'a, ViewpointMarker>,
    stats: WriteStorage<'a, CreatureStats>,
    pos: WriteStorage<'a, Position>,
}

impl<'a> System<'a> for LastUpdateSystem {
    type SystemData = SystemDataT<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        use specs::join::Join;

        let mut entity_position_tracker = data.entity_position_tracker;

        for (entity, stats) in (&data.entities, &data.stats).join() {
            if stats.health().value() <= 0 {
                entity_position_tracker.remove_entity(entity);

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
