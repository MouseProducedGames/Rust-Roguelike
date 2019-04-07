/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

 **/
// External includes.
use rand::{thread_rng, Rng};

// Standard includes.

// Internal includes.
use crate::Roll;

#[allow(dead_code)]
pub struct Die<TSize: Roll> {
    size: TSize,
    result: i64,
}

impl<TSize> Die<TSize>
where
    TSize: Roll,
{
    #[allow(dead_code)]
    pub fn new(size: TSize) -> Self {
        Self {
            size,
            result: 0,
        }
    }
}

impl<TSize> Roll for Die<TSize>
where
    TSize: Roll,
{
    fn roll(&mut self) -> &Self {
        self.size.roll();
        self.result = thread_rng().gen_range(1, self.size.total() + 1) as i64;

        self
    }

    fn total(&self) -> i64 {
        self.result
    }
}
