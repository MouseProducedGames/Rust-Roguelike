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
use crate::bodies::Body;
use crate::events::EventManager;
use crate::factions::Faction;
use crate::game::combat::{AttackData, AttackValue, DefenceValue};
use crate::game::{EntityPositionTracker, Time};
use crate::items::weapons::{Weapon, WeaponGroup};
use crate::rrl_math::Position;
use crate::world::{execute_tile_func, Tilemap, VisibilityMapLookup};

pub struct CommandSystem;

#[derive(SystemData)]
pub struct SystemDataT<'a> {
    current_time: ReadExpect<'a, Time>,
    entity_position_tracker: ReadExpect<'a, EntityPositionTracker>,
    entities: Entities<'a>,
    event_manager: ReadExpect<'a, Arc<Mutex<EventManager>>>,
    map: WriteExpect<'a, Tilemap>,
    bodies: ReadStorage<'a, Body>,
    command: ReadStorage<'a, Command>,
    factions: ReadStorage<'a, Faction>,
    weapons: ReadStorage<'a, Weapon>,
    visibility_map_lookup: WriteStorage<'a, VisibilityMapLookup>,
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

        for (entity, body, command, pos, visibility_map_lookup) in (
            &data.entities,
            &data.bodies,
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
                            body.clone(),
                            current_time,
                            entity,
                            event_manager.clone(),
                            new_pos,
                            pos,
                            entity_position_tracker,
                            &factions,
                            &data.weapons,
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
    body: Body,
    current_time: Time,
    entity: Entity,
    event_manager: Arc<Mutex<EventManager>>,
    new_pos: Position,
    pos: &mut Position,
    entity_position_tracker: &EntityPositionTracker,
    factions: &ReadStorage<'a, Faction>,
    weapons: &ReadStorage<'a, Weapon>,
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

            let mut multi_action_penalty: AttackValue = AttackValue::from(0);
            for body_slot in body.get().values() {
                if body_slot.attack_slot() {
                    let item_entity = body_slot.item();
                    let mut attack_modifier = multi_action_penalty;
                    let mut weapon_group = WeaponGroup::Unarmed;
                    if let Some(weapon) = weapons.get(item_entity) {
                        attack_modifier += weapon.attack_value();
                        weapon_group = weapon.weapon_group();
                    }
                    let attack_modifier = attack_modifier;
                    let weapon_group = weapon_group;
                    event_manager.lock().unwrap().push_attack_event(
                        current_time,
                        AttackData::new(
                            entity,
                            other_entity,
                            attack_modifier,
                            DefenceValue::from(0),
                            weapon_group,
                        ),
                    );

                    multi_action_penalty += AttackValue::from(-4);
                }
            }
        }
        None => *pos = new_pos,
    }
}
