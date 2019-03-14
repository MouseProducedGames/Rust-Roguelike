/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

 **/

// External includes
extern crate rand;
use rand::Rng;
use rand::rngs::ThreadRng;

// Internal includes
use crate::creature::LoveFearHate;

pub fn make_decision_attack( emotion: LoveFearHate, rel_str: f32, rnd: &mut ThreadRng ) -> bool
{
    match emotion.calc_love( rel_str ) < 0.0 {
        true => {
            let hate_sub_fear = emotion.calc_hate( rel_str ) - emotion.calc_fear( rel_str );
            let zero_to_one = ( hate_sub_fear * 0.5 ) + 0.5;
            calc_response_zero_one( zero_to_one, rnd )
        },
        false => false,
    }
}

fn calc_response_zero_one( value: f32, rnd: &mut ThreadRng ) -> bool
{    
    let chance = value.powf( 2.0 );
    chance >= rnd.gen_range( 0.0, 1.0 )
}
