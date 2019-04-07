/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

 **/

// External includes.

// Standard includes.

// Internal includes.

pub trait Roll {
    fn roll(&mut self) -> &Self;
    fn total(&self) -> i64;
}

impl Roll for i32 {
    fn roll(&mut self) -> &Self {
        self
    }
    fn total(&self) -> i64 {
        i64::from(*self)
    }
}

impl Roll for u32 {
    fn roll(&mut self) -> &Self {
        self
    }
    fn total(&self) -> i64 {
        i64::from(*self)
    }
}

impl Roll for i64 {
    fn roll(&mut self) -> &Self {
        self
    }
    fn total(&self) -> i64 {
        *self
    }
}
