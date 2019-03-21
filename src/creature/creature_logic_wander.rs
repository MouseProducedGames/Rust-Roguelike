use rand::Rng;
/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External dependencies.
use specs::{
    Component, Entities, NullStorage, ReadExpect, ReadStorage, System, WriteExpect, WriteStorage,
};

// Internal dependencies.
use crate::creature::{Command, CreatureTracker};
use crate::game::GameState;
use crate::rrl_math::{Displacement, Position};
use crate::world::Tilemap;

#[derive(Default)]
pub struct CreatureLogicWander;

impl Component for CreatureLogicWander {
    type Storage = NullStorage<Self>;
}

pub struct CreatureLogicWanderSystem;

#[derive(SystemData)]
pub struct SystemDataT<'a> {
    _creature_tracker: ReadExpect<'a, CreatureTracker>,
    _entities: Entities<'a>,
    _map: ReadExpect<'a, Tilemap>,
    logic: ReadStorage<'a, CreatureLogicWander>,
    game_state: WriteExpect<'a, GameState>,
    commands: WriteStorage<'a, Command>,
    _pos: WriteStorage<'a, Position>,
}

impl<'a> System<'a> for CreatureLogicWanderSystem {
    type SystemData = SystemDataT<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        use specs::join::Join;

        let _creature_tracker = data._creature_tracker;
        let mut game_state = data.game_state;
        let _map = data._map;

        for (_entity, _, command, _pos) in (
            &data._entities,
            &data.logic,
            &mut data.commands,
            &mut data._pos,
        )
            .join()
        {
            let key_command = game_state.rng().gen_range(1, 10);
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

            *command = Command::Move(target_move);

            /* let new_pos = *pos + target_move;

            if map.passable_pos( new_pos ) &&
                creature_tracker.check_collision( entity, new_pos ) == None
            {
                *pos = new_pos;
            } */
        }
    }
}
