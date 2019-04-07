/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

 **/
// External includes.

// Standard includes.

// Internal includes.
use crate::Roll;

pub struct RollSet<TCount: Roll, TDie: Roll, TModifier: Roll> {
    count: TCount,
    die: TDie,
    modifier: TModifier,
}

impl<TCount, TDie, TModifier> RollSet<TCount, TDie, TModifier>
where
    TCount: Roll,
    TDie: Roll,
    TModifier: Roll,
{
    #[allow(dead_code)]
    pub fn new(count: TCount, die: TDie, modifier: TModifier) -> Self {
        Self {
            count,
            die,
            modifier,
        }
    }
}

impl<TCount, TDie, TModifier> Roll for RollSet<TCount, TDie, TModifier>
where
    TCount: Roll,
    TDie: Roll,
    TModifier: Roll,
{
    fn roll(&mut self) -> &Self {
        self.count.roll();
        self.die.roll();
        self.modifier.roll();

        self
    }

    fn total(&self) -> i64 {
        let mut total: i64 = 0;
        for _ in 0..self.count.total() {
            total += self.die.total();
        }
        total += self.modifier.total();
        total
    }
}
