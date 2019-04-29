/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.

// Standard includes.
use std::any::TypeId;
use std::cmp::{Eq, PartialEq};
use std::hash::{Hash, Hasher};

// Internal includes.
use super::{MapPosition, Mapping, Multimap};
use crate::rrl_math::Position;

type LightLevel = f64;
type Width = u16;
type Height = u16;

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
        for pos in self.get_position(0, 0) {
            *self.values.value_mut(pos) = 0.0;
        }
    }

    pub fn height(&self) -> Height {
        self.values.height()
    }

    pub fn width(&self) -> Width {
        self.values.width()
    }

    pub fn value(&self, pos: MapPosition) -> LightLevel {
        if self.is_in_bounds(pos.x(), pos.y()) {
            *self.values.value(pos)
        } else {
            0.0
        }
    }

    pub fn value_mut(&mut self, pos: MapPosition) -> &mut LightLevel {
        self.values.value_mut(pos)
    }

    pub fn _value_pos(&self, pos: Position) -> LightLevel {
        if self.is_pos_in_bounds(pos) {
            self.value(self.get_position(pos.x as u16, pos.y as u16).unwrap())
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

impl<'a> Mapping<'a> for Lightmap {
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
