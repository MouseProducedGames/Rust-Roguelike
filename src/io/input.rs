/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes

// Standard includes.

// internal includes
use super::input_instance::{InputData, InputInstance};

pub struct Input {
    input_instance: InputInstance,
}

impl Input {
    pub fn new() -> Self {
        Self { input_instance: InputInstance::new() }
    }
    
    pub(super) fn update(&mut self, input_data: InputData) {
        self.instance_mut().input_data = input_data;
    }
    
    pub fn instance(&self) -> &InputInstance {
        &self.input_instance
    }
    
    fn instance_mut(&mut self) -> &mut InputInstance {
        &mut self.input_instance
    }
}
