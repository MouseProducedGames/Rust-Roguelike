/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use linked_hash_map::LinkedHashMap;
use specs::{Component, Entities, System, VecStorage, WriteStorage};

// Standard includes.
use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard};

// internal includes.
use super::BodySlot;
use crate::rrl_math::Position;

#[derive(Clone)]
pub struct Body {
    values: Arc<Mutex<LinkedHashMap<String, BodySlot>>>,
}

impl Body {
    pub fn new(body_slots: &[BodySlot]) -> Self {
        let output = Self {
            values: Arc::new(Mutex::new(LinkedHashMap::new())),
        };

        {
            let mut values = output.values.lock().unwrap();
            for body_slot in body_slots {
                values.insert(body_slot.name().clone(), body_slot.clone());
            }
        }

        output
    }

    pub fn get(&self) -> MutexGuard<LinkedHashMap<String, BodySlot>> {
        self.values.lock().unwrap()
    }
}

impl Component for Body {
    type Storage = VecStorage<Self>;
}

pub struct BodySystem;

#[derive(SystemData)]
pub struct SystemDataT<'a> {
    entities: Entities<'a>,
    positions: WriteStorage<'a, Position>,
    bodies: WriteStorage<'a, Body>,
}

impl<'a> System<'a> for BodySystem {
    type SystemData = SystemDataT<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        use specs::join::Join;

        let mut positions = data.positions;

        for (entity, body) in (&data.entities, &mut data.bodies).join() {
            if let Some(body_position) = positions.get(entity) {
                let body_position = *body_position;
                for body_slot in body.get().values() {
                    let item_entity = body_slot.default_item();
                    if positions.get(item_entity).is_none()
                        && positions.insert(item_entity, body_position).is_err()
                    {
                        panic!("Could not add position to an item entity.");
                    }

                    if let Some(item_position) = positions.get_mut(item_entity) {
                        *item_position = body_position;
                    } else {
                        panic!("Item position that was just added, does not exist!");
                    };

                    if let Some(item_entity) = body_slot.held_item() {
                        if positions.get(item_entity).is_none()
                            && positions.insert(item_entity, body_position).is_err()
                        {
                            panic!("Could not add position to an item entity.");
                        }

                        if let Some(item_position) = positions.get_mut(item_entity) {
                            *item_position = body_position;
                        } else {
                            panic!("Item position that was just added, does not exist!");
                        };
                    }
                }
            }
        }
    }
}