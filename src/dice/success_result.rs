/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

 **/
// External includes.

// Standard includes.
use std::ops::Deref;

// Internal includes.

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SuccessResult(bool, i32);

impl SuccessResult {
    pub fn new(roll: i32, natural_roll: i32) -> Self {
        SuccessResult((roll > 10) || (natural_roll >= 17), roll)
    }

    pub fn is_failure(self) -> bool {
        self.is_success() == false
    }

    pub fn is_success(self) -> bool {
        self.0
    }

    pub fn margin_of_success(self) -> i32 {
        self.roll() - 10
    }

    pub fn roll(self) -> i32 {
        self.1
    }
}

impl Deref for SuccessResult {
    type Target = bool;

    fn deref(&self) -> &bool {
        &self.0
    }
}
