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
use crate::game::combat::AttackActionData;
use crate::game::{EntityPositionTracker, Time};
use crate::maps::{execute_tile_func, Mapping, Tilemap, VisibilityMapLookup};
use crate::rrl_math::Position;

pub struct CommandSystem;

#[derive(SystemData)]
pub struct SystemDataT<'a> {
    current_time: ReadExpect<'a, Time>,
    entity_position_tracker: WriteExpect<'a, EntityPositionTracker>,
    entities: Entities<'a>,
    event_manager: ReadExpect<'a, Arc<Mutex<EventManager>>>,
    map: WriteExpect<'a, Tilemap>,
    command: ReadStorage<'a, Command>,
    factions: ReadStorage<'a, Faction>,
    visibility_map_lookup: WriteStorage<'a, VisibilityMapLookup>,
    positions: WriteStorage<'a, Position>,
}

impl<'a> System<'a> for CommandSystem {
    type SystemData = SystemDataT<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        use specs::join::Join;

        let current_time = *data.current_time;
        let entity_position_tracker = &mut *data.entity_position_tracker;
        let event_manager = data.event_manager.clone();
        let factions = data.factions;
        let map = &mut *data.map;

        for (entity, command, position, visibility_map_lookup) in (
            &data.entities,
            &data.command,
            &mut data.positions,
            &mut data.visibility_map_lookup,
        )
            .join()
        {
            let command = command;

            let visibility_map = visibility_map_lookup.get_or_add(map);

            #[allow(clippy::single_match)]
            match *command {
                Command::Move(displacement) => {
                    let new_position = *position + displacement;

                    if map.is_pos_in_bounds(new_position) && map.passable_pos(new_position) {
                        impassable_movement(
                            current_time,
                            entity,
                            event_manager.clone(),
                            new_position,
                            position,
                            entity_position_tracker,
                            &factions,
                        );

                        let visibility_type = visibility_map.value_pos(new_position);

                        execute_tile_func(false, 100, map, visibility_type, new_position);
                    }
                }
                _ => (),
            }

            entity_position_tracker.set_position(entity, *position);
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn impassable_movement<'a>(
    current_time: Time,
    entity: Entity,
    event_manager: Arc<Mutex<EventManager>>,
    new_position: Position,
    position: &mut Position,
    entity_position_tracker: &EntityPositionTracker,
    factions: &ReadStorage<'a, Faction>,
) {
    match entity_position_tracker.check_collision(entity, new_position) {
        Some(other_entity) => {
            if let Some(faction_a) = factions.get(entity) {
                if let Some(faction_b) = factions.get(other_entity) {
                    if faction_a == faction_b {
                        return;
                    }
                }
            }

            event_manager.lock().unwrap().push_attack_action_event(
                current_time,
                AttackActionData::new(entity, other_entity),
            );
        }
        None => *position = new_position,
    }
}
