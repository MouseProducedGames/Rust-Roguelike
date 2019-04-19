/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

 **/
// External includes.
use specs::{Entities, Entity, ReadExpect, ReadStorage, System, WriteExpect, WriteStorage};

// Standard includes.
use std::sync::{Arc, Mutex};

// Internal includes.
use crate::ai::Command;
use crate::events::EventManager;
use crate::factions::Faction;
use crate::game::{AttackData, Combat, CombatResult, EntityPositionTracker, Time};
use crate::rrl_math::Position;
use crate::stats::{CreatureStats, Stat};
use crate::world::{execute_tile_func, Tilemap, VisibilityMapLookup};

pub struct CommandSystem;

#[derive(SystemData)]
pub struct SystemDataT<'a> {
    current_time: ReadExpect<'a, Time>,
    entity_position_tracker: ReadExpect<'a, EntityPositionTracker>,
    entities: Entities<'a>,
    event_manager: ReadExpect<'a, Arc<Mutex<EventManager>>>,
    map: WriteExpect<'a, Tilemap>,
    command: ReadStorage<'a, Command>,
    factions: ReadStorage<'a, Faction>,
    visibility_map_lookup: WriteStorage<'a, VisibilityMapLookup>,
    stats: WriteStorage<'a, CreatureStats>,
    pos: WriteStorage<'a, Position>,
}

impl<'a> System<'a> for CommandSystem {
    type SystemData = SystemDataT<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        use specs::join::Join;

        let current_time = *data.current_time;
        let entity_position_tracker = &*data.entity_position_tracker;
        let event_manager = data.event_manager.clone();
        let factions = data.factions;
        let map = &mut *data.map;
        let stats = &mut data.stats;

        for (entity, command, pos, visibility_map_lookup) in (
            &data.entities,
            &data.command,
            &mut data.pos,
            &mut data.visibility_map_lookup,
        )
            .join()
        {
            let command = command;

            let visibility_map = visibility_map_lookup.get_or_add(map);

            #[allow(clippy::single_match)]
            match *command {
                Command::Move(disp) => {
                    let new_pos = *pos + disp;

                    if map.passable_pos(new_pos) {
                        impassable_movement(
                            current_time,
                            entity,
                            event_manager.clone(),
                            new_pos,
                            pos,
                            entity_position_tracker,
                            &factions,
                            stats,
                        );
                    }

                    let visibility_type = visibility_map.value_pos(new_pos);

                    execute_tile_func(false, 100, map, visibility_type, new_pos);
                }
                _ => (),
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn impassable_movement<'a>(
    current_time: Time,
    entity: Entity,
    event_manager: Arc<Mutex<EventManager>>,
    new_pos: Position,
    pos: &mut Position,
    entity_position_tracker: &EntityPositionTracker,
    factions: &ReadStorage<'a, Faction>,
    stats: &mut WriteStorage<'a, CreatureStats>,
) {
    match entity_position_tracker.check_collision(entity, new_pos) {
        Some(other_entity) => {
            if let Some(faction_a) = factions.get(entity) {
                if let Some(faction_b) = factions.get(other_entity) {
                    if faction_a == faction_b {
                        return;
                    }
                }
            }

            event_manager
                .lock()
                .unwrap()
                .push_attack_event(current_time, AttackData::new(entity, other_entity, 0, 0));

            /* let attacker_stats;
            let defender_stats;
            match stats.get(entity) {
                Some(stats) => attacker_stats = *stats,
                _ => return,
            }
            match stats.get_mut(other_entity) {
                Some(stats) => defender_stats = stats,
                _ => return,
            }

            if let CombatResult::DefenderDead = Combat::one_attack(&attacker_stats, defender_stats)
            {
                (*defender_stats.health_mut().value_mut()).min(-100);
            } */
        }
        None => *pos = new_pos,
    }
}
