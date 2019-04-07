/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use specs::{
    Component, DenseVecStorage, Entities, ReadExpect, ReadStorage, System, WriteExpect,
    WriteStorage,
};

// Standard includes.
use std::sync::{Arc, Mutex};

// Internal includes.
use super::{Command, CreatureTracker, PlayerMarker};
use crate::game::GameState;
use crate::io::Display;
use crate::rrl_math::{Displacement, Position};
use crate::world::Tilemap;

pub struct CreatureLogicPlayer {}

impl Component for CreatureLogicPlayer {
    type Storage = DenseVecStorage<Self>;
}

pub struct CreatureLogicPlayerSystem;

#[derive(SystemData)]
pub struct SystemDataT<'a> {
    _entities: Entities<'a>,
    _creature_tracker: ReadExpect<'a, CreatureTracker>,
    _map: ReadExpect<'a, Tilemap>,
    display: ReadExpect<'a, Arc<Mutex<Display>>>,
    game_state: WriteExpect<'a, GameState>,
    player_marker: ReadStorage<'a, PlayerMarker>,
    commands: WriteStorage<'a, Command>,
    logic: WriteStorage<'a, CreatureLogicPlayer>,
    pos: WriteStorage<'a, Position>,
}

impl<'a> System<'a> for CreatureLogicPlayerSystem {
    type SystemData = SystemDataT<'a>;

    fn run(&mut self, mut data: SystemDataT) {
        use specs::Join;

        let _creature_tracker = data._creature_tracker;
        let mut game_state = data.game_state;
        let _map = data._map;
        let display = data.display.lock().unwrap();

        for (_entity, _logic, command, _pos, _) in (
            &data._entities,
            &mut data.logic,
            &mut data.commands,
            &mut data.pos,
            &data.player_marker,
        )
            .join()
        {
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
