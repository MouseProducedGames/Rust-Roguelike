/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use specs::{
    Component, DenseVecStorage, ReadExpect, ReadStorage, System, WriteExpect, WriteStorage,
};

// Standard includes.
use std::sync::{Arc, Mutex};

// Internal includes.
use super::{Command, PlayerMarker};
use crate::game::GameState;
use crate::io::Display;
use crate::rrl_math::Displacement;

pub struct LogicPlayer {}

impl Component for LogicPlayer {
    type Storage = DenseVecStorage<Self>;
}

pub struct LogicPlayerSystem;

#[derive(SystemData)]
pub struct SystemDataT<'a> {
    display: ReadExpect<'a, Arc<Mutex<Display>>>,
    game_state: WriteExpect<'a, GameState>,
    player_marker: ReadStorage<'a, PlayerMarker>,
    commands: WriteStorage<'a, Command>,
    logic: WriteStorage<'a, LogicPlayer>,
}

impl<'a> System<'a> for LogicPlayerSystem {
    type SystemData = SystemDataT<'a>;

    fn run(&mut self, mut data: SystemDataT) {
        use specs::Join;

        let mut game_state = data.game_state;
        let display = data.display.lock().unwrap();

        for (_, command, _) in (&mut data.logic, &mut data.commands, &data.player_marker).join() {
            let key_command = display.get_char();

            let target_move;
            match key_command {
                '1' => target_move = Displacement::new(-1, 1),
                '2' => target_move = Displacement::new(0, 1),
                '3' => target_move = Displacement::new(1, 1),
                '4' => target_move = Displacement::new(-1, 0),
                '6' => target_move = Displacement::new(1, 0),
                '7' => target_move = Displacement::new(-1, -1),
                '8' => target_move = Displacement::new(0, -1),
                '9' => target_move = Displacement::new(1, -1),
                'q' => {
                    target_move = Displacement::new(0, 0);
                    game_state.kill();
                }
                _ => target_move = Displacement::new(0, 0),
            }

            *command = Command::Move(target_move);
        }
    }
}
