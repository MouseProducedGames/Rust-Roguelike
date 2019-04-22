/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes

// Standard includes.

// internal includes

pub(super) union InputData {
    pub(super) ch: char,
}

pub struct InputInstance {
    pub(super) input_data: InputData,
}

impl InputInstance {
    pub(super) fn new() -> Self {
        Self {
            input_data: InputData { ch: 0 as char },
        }
    }
    
    pub fn consume_char(&mut self) -> char {
        let output = unsafe { self.input_data.ch };
        self.input_data.ch = '\0';
        output
    }

    pub fn get_char(&self) -> char {
        unsafe { self.input_data.ch }
    }
}
