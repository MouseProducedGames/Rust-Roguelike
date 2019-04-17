/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use specs::World;

// Standard includes.
use std::sync::{Arc, Mutex};

// Internal includes.
use super::{Screen, ScreenPushWrapper, ScreenState};
#[allow(unused_imports)]
use crate::world::{
    Mapping, PatternFlags, PatternLookup, Tilemap, TILE_TYPE_INDEX_DOOR, TILE_TYPE_INDEX_FLOOR,
    TILE_TYPE_INDEX_VOID, TILE_TYPE_INDEX_WALL,
};

pub struct PatternsInitScreen {
    state: ScreenState,
}

impl PatternsInitScreen {
    pub fn new() -> Self {
        Self {
            state: ScreenState::Started,
        }
    }
}

impl Screen for PatternsInitScreen {
    fn init(&mut self) {
        self.state = match self.state {
            ScreenState::Inactive => ScreenState::Inactive,
            ScreenState::Started => ScreenState::Running,
            ScreenState::Running => ScreenState::Running,
            ScreenState::Stopped => ScreenState::Inactive,
        }
    }

    fn close(&mut self) {
        self.state = match self.state {
            ScreenState::Inactive => ScreenState::Inactive,
            ScreenState::Started => ScreenState::Inactive,
            ScreenState::Running => ScreenState::Inactive,
            ScreenState::Stopped => ScreenState::Inactive,
        }
    }

    fn blocks_draw(&self) -> bool {
        true
    }

    fn blocks_update(&self) -> bool {
        true
    }

    fn draw(&mut self, _world: &mut World) {}

    fn update(&mut self, world: &mut World, _screen_push_wrapper: &mut ScreenPushWrapper) {
        let pattern_lookup = world.write_resource::<Arc<Mutex<PatternLookup>>>();
        let mut pattern_lookup = pattern_lookup.lock().unwrap();

        let mut crypt_niche = Tilemap::new(3, 3);

        *crypt_niche.tile_type_mut(crypt_niche.get_position(0, 0).unwrap()) = TILE_TYPE_INDEX_WALL;
        *crypt_niche.tile_type_mut(crypt_niche.get_position(1, 0).unwrap()) = TILE_TYPE_INDEX_WALL;
        *crypt_niche.tile_type_mut(crypt_niche.get_position(2, 0).unwrap()) = TILE_TYPE_INDEX_WALL;

        *crypt_niche.tile_type_mut(crypt_niche.get_position(0, 1).unwrap()) = TILE_TYPE_INDEX_WALL;
        *crypt_niche.tile_type_mut(crypt_niche.get_position(1, 1).unwrap()) = TILE_TYPE_INDEX_FLOOR;
        *crypt_niche.tile_type_mut(crypt_niche.get_position(2, 1).unwrap()) = TILE_TYPE_INDEX_WALL;

        *crypt_niche.tile_type_mut(crypt_niche.get_position(0, 2).unwrap()) = TILE_TYPE_INDEX_WALL;
        *crypt_niche.tile_type_mut(crypt_niche.get_position(1, 2).unwrap()) = TILE_TYPE_INDEX_DOOR;
        *crypt_niche.tile_type_mut(crypt_niche.get_position(2, 2).unwrap()) = TILE_TYPE_INDEX_WALL;

        pattern_lookup.insert("Crypt Niche", crypt_niche);

        self.state = ScreenState::Stopped;
    }

    fn state(&self) -> ScreenState {
        self.state
    }
}
