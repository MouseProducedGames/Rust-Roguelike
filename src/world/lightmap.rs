/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes
use std::any::TypeId;
use std::cmp::{Eq, PartialEq};
use std::hash::{Hash, Hasher};

// Internal includes
use super::super::multimap::Multimap;
use super::mapping::Mapping;
use crate::rrl_math::Position;

type LightLevel = f64;
type Width = u32;
type Height = u32;

pub struct Lightmap {
    id: TypeId,
    values: Multimap<LightLevel>,
}

impl Lightmap {
    pub fn new(width: Width, height: Height) -> Self {
        Self {
            id: TypeId::of::<Lightmap>(),
            values: Multimap::new(width, height),
        }
    }
    
    pub fn clear(&mut self) {
        for y in 0..self.values.height() {
            for x in 0..self.values.width() {
                *self.values.value_mut(x, y) = 0.0;
            }
        }
    }

    pub fn height(&self) -> Height {
        self.values.height()
    }

    pub fn width(&self) -> Width {
        self.values.width()
    }

    pub fn value(&self, pos_x: Width, pos_y: Height) -> LightLevel {
        if self.is_in_bounds(pos_x, pos_y) {
            *self.values.value(pos_x, pos_y)
        } else {
            0.0
        }
    }

    pub fn value_mut(&mut self, pos_x: Width, pos_y: Height) -> &mut LightLevel {
        self.values.value_mut(pos_x , pos_y)
    }

    pub fn value_pos(&self, pos: Position) -> LightLevel {
        if self.is_pos_in_bounds(pos) {
            self.value(pos.x as u32, pos.y as u32)
        } else {
            0.0
        }
    }
}

impl Hash for Lightmap {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl Mapping for Lightmap {
    fn height(&self) -> Height {
        Lightmap::height(self)
    }

    fn width(&self) -> Width {
        Lightmap::width(self)
    }
}

impl PartialEq for Lightmap {
    fn eq(&self, other: &Lightmap) -> bool {
        self.id == other.id
    }
}

impl Eq for Lightmap {}
