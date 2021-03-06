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
use super::{
    MapPosition, Mapping, Multimap, TileFunc, TileTypeData, TILE_FUNC_DATA, TILE_FUNC_INDEX_VOID,
    TILE_TYPE_DATA, TILE_TYPE_INDEX_VOID,
};
use crate::rrl_math::Position;

type TileType = u32;
type Width = u16;
type Height = u16;

pub struct Tilemap {
    id: TypeId,
    tiles: Multimap<TileType>,
    tile_funcs: Multimap<TileType>,
}

impl Tilemap {
    pub fn new(width: Width, height: Height) -> Self {
        Self {
            id: TypeId::of::<Tilemap>(),
            tiles: Multimap::new(width, height),
            tile_funcs: Multimap::new(width, height),
        }
    }

    pub fn height(&self) -> Height {
        self.tiles.height()
    }

    pub fn width(&self) -> Width {
        self.tiles.width()
    }

    pub fn _passable(&self, pos: MapPosition) -> bool {
        self._tile(pos).passable()
    }

    pub fn passable_pos(&self, pos: Position) -> bool {
        self.tile_pos(pos).passable()
    }

    pub fn _tile_func(&self, pos: MapPosition) -> TileFunc {
        TILE_FUNC_DATA[self.tile_func_type(pos) as usize]
    }

    pub fn tile_func_pos(&self, pos: Position) -> TileFunc {
        TILE_FUNC_DATA[self.tile_func_type_pos(pos) as usize]
    }

    pub fn tile_func_type(&self, pos: MapPosition) -> TileType {
        *self.tile_funcs.value(pos)
    }

    pub fn tile_func_type_mut(&mut self, pos: MapPosition) -> &mut TileType {
        self.tile_funcs.value_mut(pos)
    }

    pub fn tile_func_type_pos(&self, pos: Position) -> TileType {
        if self.is_pos_in_bounds(pos) {
            self.tile_func_type(self.get_position(pos.x as u16, pos.y as u16).unwrap())
        } else {
            TILE_FUNC_INDEX_VOID
        }
    }

    pub fn _tile(&self, pos: MapPosition) -> TileTypeData {
        TILE_TYPE_DATA[self.tile_type(pos) as usize]
    }

    pub fn tile_pos(&self, pos: Position) -> TileTypeData {
        TILE_TYPE_DATA[self.tile_type_pos(pos) as usize]
    }

    pub fn tile_type(&self, pos: MapPosition) -> TileType {
        *self.tiles.value(pos)
    }

    pub fn tile_type_mut(&mut self, pos: MapPosition) -> &mut TileType {
        self.tiles.value_mut(pos)
    }

    pub fn tile_type_pos(&self, pos: Position) -> TileType {
        if self.is_pos_in_bounds(pos) {
            self.tile_type(self.get_position(pos.x as u16, pos.y as u16).unwrap())
        } else {
            TILE_TYPE_INDEX_VOID
        }
    }

    pub fn transparent(&self, pos: MapPosition) -> bool {
        self._tile(pos).transparent()
    }

    pub fn _transparent_pos(&self, pos: Position) -> bool {
        self.tile_pos(pos).transparent()
    }
}

impl Hash for Tilemap {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl<'a> Mapping<'a> for Tilemap {
    fn height(&self) -> Height {
        Tilemap::height(self)
    }

    fn width(&self) -> Width {
        Tilemap::width(self)
    }
}

impl PartialEq for Tilemap {
    fn eq(&self, other: &Tilemap) -> bool {
        self.id == other.id
    }
}

impl Eq for Tilemap {}
