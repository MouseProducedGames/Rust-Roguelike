/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

 **/
// External includes.
use rand::{thread_rng, Rng};
use rust_dice::{Die, Roll, RollSet};

// Standard includes.
use std::ops::Deref;

// Internal includes.
use crate::game::combat::{AttackValue, DefenceValue};
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

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SuccessResult(bool, i32);

impl SuccessResult {
    pub fn new(roll: i32) -> Self {
        SuccessResult(roll > 10, roll)
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

pub fn roll_attack(attack_value: AttackValue, defence_value: DefenceValue) -> SuccessResult {
    roll_success(i32::from(attack_value) - i32::from(defence_value))
}

pub fn roll_success(skill_bonus: i32) -> SuccessResult {
    let result = SuccessRoll::new(3, Die::new(6), 0).roll().total() + i64::from(skill_bonus);
    SuccessResult::new(result as i32)
}
