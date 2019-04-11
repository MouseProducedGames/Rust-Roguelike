/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use specs::{Component, NullStorage, ReadStorage, System, WriteStorage};

// Standard includes.

// Internal includes.
use crate::ai::Command;
use crate::dice::get_random_move;

#[derive(Default)]
pub struct LogicWander;

impl Component for LogicWander {
    type Storage = NullStorage<Self>;
}

pub struct LogicWanderSystem;

#[derive(SystemData)]
pub struct SystemDataT<'a> {
    logic: ReadStorage<'a, LogicWander>,
    commands: WriteStorage<'a, Command>,
}

impl<'a> System<'a> for LogicWanderSystem {
    type SystemData = SystemDataT<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        use specs::join::Join;

        for (_, command) in (&data.logic, &mut data.commands).join() {
            let target_move = get_random_move();

            *command = Command::Move(target_move);
        }
    }
}
