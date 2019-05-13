/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

 **/
// External includes.

// Standard includes.
use std::ops::Deref;

// Internal includes.
use crate::game::GameValueFixed;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SuccessResult(bool, GameValueFixed);

impl SuccessResult {
    pub fn new(roll: GameValueFixed, natural_roll: GameValueFixed) -> Self {
        SuccessResult((roll > 10) || (natural_roll >= 17), roll)
    }

    pub fn _is_failure(self) -> bool {
        self.is_success() == false
    }

    pub fn is_success(self) -> bool {
        self.0
    }

    pub fn margin_of_success(self) -> GameValueFixed {
        self.roll() - GameValueFixed::from_int(10)
    }

    pub fn roll(self) -> GameValueFixed {
        self.1
    }
}

impl Deref for SuccessResult {
    type Target = bool;

    fn deref(&self) -> &bool {
        &self.0
    }
}
