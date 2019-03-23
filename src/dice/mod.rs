/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

 **/
// External includes.
use rust_dice::{Die, Roll, RollSet};

// Internal includes.

type SuccessRoll = RollSet<u32, Die<u32>, i32>;

pub fn roll_success( skill_bonus: i64 ) -> bool
{
    SuccessRoll::new( 3, Die::new(6), 0).roll().total() + skill_bonus > 10
}
