/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use linked_hash_map::LinkedHashMap;
use rand::{thread_rng, Rng};
use specs::{Component, Entities, ReadStorage, System, VecStorage, WriteStorage};

// Standard includes.
use std::sync::{Arc, Mutex, MutexGuard};

// internal includes.
use super::BodySlot;
use crate::items::Item;
use crate::rrl_math::Position;
use crate::stats::{CreatureStats, Stat};

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

    pub fn get_random_by_size(&self) -> BodySlot {
        let values = self.values.lock().unwrap();
        let mut total_size: u32 = 0;
        for body_slot in values.values() {
            total_size += body_slot.size();
        }
        let total_size = total_size;

        let mut roll = i64::from(thread_rng().gen_range(0, total_size));
        for body_slot in values.values() {
            roll -= i64::from(body_slot.size());
            if roll <= 0 {
                return body_slot.clone();
            }
        }

        panic!("BodySlot selection size roll escaped bounds!");
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

pub struct CreatureBodySystem;

#[derive(SystemData)]
pub struct CreatureSystemData<'a> {
    bodies: WriteStorage<'a, Body>,
    items: WriteStorage<'a, Item>,
    stats: ReadStorage<'a, CreatureStats>,
}

impl<'a> System<'a> for CreatureBodySystem {
    type SystemData = CreatureSystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        use specs::join::Join;

        let mut items = data.items;

        for (body, stats) in (&mut data.bodies, &data.stats).join() {
            if stats.health().value() <= 0 {
                for (_, body_slot) in body.get().iter_mut() {
                    if let Some(item_entity) = body_slot.drop_item() {
                        if let Some(item) = items.get_mut(item_entity) {
                            item.owned_mut(false);
                        } else {
                            panic!("All item entities must have an Item component.");
                        }
                    }

                    /* if let Some(body_part_item) = items.get_mut(body_slot.default_item()) {
                        body_part_item.owned_mut(false);
                    } else {
                        panic!("All item entities must have an Item component.");
                    } */
                }

                // body.get().clear();
            }
        }
    }
}
