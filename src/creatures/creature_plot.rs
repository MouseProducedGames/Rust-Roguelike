/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use rand::{thread_rng, Rng};

// Standard includes.

// Internal includes.
use crate::rrl_math::Position;

pub struct CreaturePlot(Position, u32, u32);

impl CreaturePlot {
    pub fn new(position: Position, numerator: u32, denominator: u32) -> Self {
        Self(position, numerator, denominator)
    }

    pub fn check(&self) -> Option<Position> {
        if thread_rng().gen_ratio(self.1, self.2) {
            Some(self.0)
        } else {
            None
        }
    }
}
