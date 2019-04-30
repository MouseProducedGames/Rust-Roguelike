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
use crate::rrl_math::Position;

pub struct CreatureFactoryWrapper {
    gen_func: Arc<Mutex<FnMut(Position, &mut World) + Send>>,
}

impl CreatureFactoryWrapper {
    pub fn new(gen_func: Arc<Mutex<FnMut(Position, &mut World) + Send>>) -> Self {
        Self { gen_func }
    }

    pub fn gen_once(&mut self, position: Position, world: &mut World) {
        let gen_func = self.gen_func.clone();
        let mut gen_func = gen_func.lock().unwrap();
        (&mut *gen_func)(position, world);
    }
}
