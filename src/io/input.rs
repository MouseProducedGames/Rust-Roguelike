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
        Self {
            input_instance: InputInstance::new(),
        }
    }

    pub(super) fn update(&mut self, input_data: InputData) {
        self.input_instance.input_data = input_data;
    }

    pub fn instance(&self) -> &InputInstance {
        &self.input_instance
    }

    pub fn instance_mut(&mut self) -> Arg {
        Arg::new(&mut self.input_instance)
    }
}

pub struct Arg<'a> {
    value: &'a mut InputInstance,
}

impl<'a> Arg<'a> {
    pub fn new(value: &'a mut InputInstance) -> Self {
        Self { value }
    }

    pub fn consume_char(&mut self) -> char {
        self.value.consume_char()
    }
}
