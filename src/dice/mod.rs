/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

 **/
// External includes.
use rand::{thread_rng, Rng};
use rust_dice::{Die, Roll, RollSet};

// Standard includes.

// Internal includes.
use crate::rrl_math::Displacement;

type SuccessRoll = RollSet<u32, Die<u32>, i32>;

pub fn get_random_move() -> Displacement {
    let key_command = random_wander_command();
    let target_move;
    match key_command {
        1 => target_move = Displacement::new(-1, 1),
        2 => target_move = Displacement::new(0, 1),
        3 => target_move = Displacement::new(1, 1),
        4 => target_move = Displacement::new(-1, 0),
        5 => target_move = Displacement::new(0, 0),
        6 => target_move = Displacement::new(1, 0),
        7 => target_move = Displacement::new(-1, -1),
        8 => target_move = Displacement::new(0, -1),
        9 => target_move = Displacement::new(1, -1),
        _ => target_move = Displacement::new(0, 0),
    }

    target_move
}

fn random_wander_command() -> u32 {
    thread_rng().gen_range(1, 10)
}

pub fn roll_success(skill_bonus: i64) -> bool {
    SuccessRoll::new(3, Die::new(6), 0).roll().total() + skill_bonus > 10
}
