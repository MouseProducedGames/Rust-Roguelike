/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use specs::{Component, Entities, Entity, ReadStorage, System, VecStorage, WriteStorage};

// Standard includes.
use std::sync::{Arc, Mutex, MutexGuard};

// internal includes.
use super::Item;
use crate::rrl_math::Position;
use crate::stats::{CreatureStats, Stat};

#[derive(Clone)]
pub struct Inventory {
    values: Arc<Mutex<Vec<Entity>>>,
}

impl Inventory {
    pub fn new() -> Self {
        Self {
            values: Arc::new(Mutex::new(vec![])),
        }
    }

    pub fn get(&self) -> MutexGuard<Vec<Entity>> {
        self.values.lock().unwrap()
    }

    pub fn push(&mut self, item: Entity) {
        self.values.lock().unwrap().push(item)
    }

    pub fn remove(&mut self, index: usize) -> Entity {
        self.values.lock().unwrap().remove(index)
    }
}

impl Component for Inventory {
    type Storage = VecStorage<Self>;
}

pub struct InventorySystem;

#[derive(SystemData)]
pub struct SystemDataT<'a> {
    entities: Entities<'a>,
    positions: WriteStorage<'a, Position>,
    inventories: WriteStorage<'a, Inventory>,
}

impl<'a> System<'a> for InventorySystem {
    type SystemData = SystemDataT<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        use specs::join::Join;

        let mut positions = data.positions;

        for (entity, inventory) in (&data.entities, &mut data.inventories).join() {
            if let Some(inventory_position) = positions.get(entity) {
                let inventory_position = *inventory_position;
                for item_entity in inventory.get().iter() {
                    let item_entity = *item_entity;
                    if positions.get(item_entity).is_none()
                        && positions.insert(item_entity, inventory_position).is_err()
                    {
                        panic!("Could not add position to an item entity.");
                    }

                    if let Some(item_position) = positions.get_mut(item_entity) {
                        *item_position = inventory_position;
                    } else {
                        panic!("Item position that was just added, does not exist!");
                    };
                }
            }
        }
    }
}

pub struct CreatureInventorySystem;

#[derive(SystemData)]
pub struct CreatureSystemData<'a> {
    inventories: ReadStorage<'a, Inventory>,
    items: WriteStorage<'a, Item>,
    stats: ReadStorage<'a, CreatureStats>,
}

impl<'a> System<'a> for CreatureInventorySystem {
    type SystemData = CreatureSystemData<'a>;

    fn run(&mut self, data: Self::SystemData) {
        use specs::join::Join;

        let mut items = data.items;

        for (inventory, stats) in (&data.inventories, &data.stats).join() {
            if stats.health().value() <= 0 {
                for item_entity in inventory.get().iter() {
                    if let Some(item) = items.get_mut(*item_entity) {
                        item.owned_mut(false);
                    } else {
                        panic!("All item entities must have an Item component.");
                    }
                }

                inventory.get().clear();
            }
        }
    }
}
