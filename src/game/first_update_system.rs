/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

 **/
// External includes.
use specs::{System, WriteStorage};

// Standard includes.

// Internal includes.
use super::combat::MultiAttackPenalty;

pub struct FirstUpdateSystem;

#[derive(SystemData)]
pub struct SystemDataT<'a> {
    multi_attack_penalties: WriteStorage<'a, MultiAttackPenalty>,
}

impl<'a> System<'a> for FirstUpdateSystem {
    type SystemData = SystemDataT<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        use specs::join::Join;

        for multi_attack_penalty in (&mut data.multi_attack_penalties).join() {
            *multi_attack_penalty = MultiAttackPenalty::from(0);
        }
    }
}
