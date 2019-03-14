/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

 **/

// External includes
extern crate rand;
use rand::Rngx;
use rand::rngs::ThreadRng;

// Internal includes
use crate::creature::LoveFearHate;

pub fn make_decision_attack( emotion: LoveFearHate, rel_str: f32, rnd: &mut ThreadRng ) -> bool
{
    match emotion.calc_love( rel_str ) < 0.0 {
        true => {
            let hate_sub_fear = emotion.calc_hate( rel_str ) - emotion.calc_fear( rel_str );
            let zero_to_one = ( hate_sub_fear * 0.5 ) + 0.5;
            let attack_chance = zero_to_one.powf( 2.0 );
            attack_chance >= rnd.gen_range( 0.0, 1.0 )
        },
        false => false,
    }
}
