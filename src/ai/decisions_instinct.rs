/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

 **/
// External includes
use rand::{thread_rng, Rng};

// Internal includes
use crate::ai::_LoveFearHate;

pub fn _make_decision_attack(emotion: _LoveFearHate, rel_str: f32) -> bool {
    if emotion._calc_love(rel_str) < 0.0 {
        let hate_sub_fear = emotion._calc_hate(rel_str) - emotion._calc_fear(rel_str);
        let zero_to_one = (hate_sub_fear * 0.5) + 0.5;
        _calc_response_zero_one(zero_to_one)
    } else {
        false
    }
}

fn _calc_response_zero_one(value: f32) -> bool {
    let chance = value.powf(2.0);
    chance >= thread_rng().gen_range(0.0, 1.0)
}
