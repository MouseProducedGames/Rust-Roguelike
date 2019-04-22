/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use specs::Entity;

// Standard includes.

// Internal includes.

#[derive(Clone, Eq, PartialEq)]
pub struct BodySlot {
    name: String,
    default_item: Entity,
    held_item: Option<Entity>,
}

impl BodySlot {
    pub fn new(name: &str, default_item: Entity) -> Self {
        Self {
            name: String::from(name),
            default_item,
            held_item: None,
        }
    }

    pub fn with_held_item(name: &str, default_item: Entity, held_item: Entity) -> Self {
        let mut output = Self::new(name, default_item);
        output.hold_item(held_item);

        output
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn default_item(&self) -> Entity {
        self.default_item
    }

    pub fn item(&self) -> Entity {
        if let Some(item) = self.held_item {
            item
        } else {
            self.default_item
        }
    }

    pub fn held_item(&self) -> Option<Entity> {
        self.held_item
    }

    pub fn hold_item(&mut self, item: Entity) -> Option<Entity> {
        let output = self.held_item;
        self.held_item = Some(item);

        output
    }

    pub fn drop_item(&mut self) -> Option<Entity> {
        let output = self.held_item;
        self.held_item = None;

        output
    }
}
