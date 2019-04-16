/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use specs::World;

// Standard includes.
use std::marker::Send;
use std::sync::{Arc, Mutex};

// Internal includes.
use crate::world::Tilemap;

pub struct MapProcessor {
    gen_func: Arc<Mutex<FnMut(&Tilemap, &mut World) -> Tilemap + Send>>,
}

impl MapProcessor {
    pub fn new(gen_func: Arc<Mutex<FnMut(&Tilemap, &mut World) -> Tilemap + Send>>) -> Self {
        Self { gen_func }
    }

    pub fn gen_once(&mut self, meta_tile_map: &Tilemap, world: &mut World) -> Tilemap {
        let gen_func = self.gen_func.clone();
        let mut gen_func = gen_func.lock().unwrap();
        (&mut *gen_func)(meta_tile_map, world)
    }
}
