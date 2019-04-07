/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use specs::{Entity, ReadStorage};

// Standard includes.
use std::collections::HashMap;

// Internal includes.
use crate::factions::Faction;
use crate::rrl_math::Position;
use crate::world::{VisibilityMap, VisibilityType};

pub struct _CreatureData {
    pos: Position,
}

pub struct CreatureTracker {
    lookup: HashMap<Entity, Position>,
}

impl CreatureTracker {
    pub fn new() -> Self {
        Self {
            lookup: HashMap::new(),
        }
    }

    pub fn check_collision(&self, entity: Entity, check_pos: Position) -> Option<Entity> {
        let mut output = None;
        for (other, other_pos) in self.lookup.iter() {
            if entity != *other && check_pos == *other_pos {
                output = Some(*other);
            }
        }

        output
    }

    pub fn get_nearest_enemy<'a>(
        &self,
        faction: Faction,
        factions: &ReadStorage<'a, Faction>,
        visibility_map: &VisibilityMap,
    ) -> Option<(Entity, Position)> {
        for (other, other_pos) in self.lookup.iter() {
            if let Some(other_faction) = factions.get(*other) {
                // println!("Aah! 1");
                if faction != *other_faction {
                    // println!("Aah! 2");
                    if visibility_map.value_pos(*other_pos) == VisibilityType::Visible {
                        // println!("Aah! 3");
                        return Some((*other, *other_pos));
                    }
                }
            }
        }

        None
    }

    pub fn get_nearest_friend<'a>(
        &self,
        entity: Entity,
        faction: Faction,
        factions: &ReadStorage<'a, Faction>,
        visibility_map: &VisibilityMap,
    ) -> Option<(Entity, Position)> {
        for (other, other_pos) in self.lookup.iter() {
            if *other == entity {
                continue;
            }

            if let Some(other_faction) = factions.get(*other) {
                // println!("Aah! 1");
                if faction == *other_faction {
                    // println!("Aah! 2");
                    if visibility_map.value_pos(*other_pos) == VisibilityType::Visible {
                        // println!("Aah! 3");
                        return Some((*other, *other_pos));
                    }
                }
            }
        }

        None
    }

    pub fn set_position(&mut self, entity: Entity, pos: Position) {
        self.lookup.insert(entity, pos);
    }

    pub fn remove_entity(&mut self, entity: Entity) {
        self.lookup.remove(&entity);
    }
}
