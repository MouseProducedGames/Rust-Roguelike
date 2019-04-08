/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes
// #[macro_use]
// extern crate lazy_static;

// Standard includes.
use std::sync::{Arc, Mutex};

// internal includes

lazy_static! {
    static ref INSTANCE: Arc<Mutex<Input>> = Arc::new(Mutex::new(Input::new()));
}

pub(super) union InputData {
    pub(super) ch: char,
}

pub struct Input {
    input_data: InputData,
}

impl Input {
    fn new() -> Self {
        Self { input_data: InputData { ch: 0 as char } }
    }
    
    pub(super) fn update(input_data: InputData) {
        INSTANCE.lock().unwrap().input_data = input_data;
    }
    
    pub fn get_char() -> char {
        unsafe { INSTANCE.lock().unwrap().input_data.ch }
    }
}