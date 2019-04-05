/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External dependencies.
use specs::{Component, Entities, NullStorage, ReadExpect, ReadStorage, System, WriteStorage};

// Internal dependencies.
use crate::ai::{Command, CreatureTracker};
use crate::dice::get_random_move;
use crate::rrl_math::Position;
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
    commands: WriteStorage<'a, Command>,
    _pos: WriteStorage<'a, Position>,
}

impl<'a> System<'a> for CreatureLogicWanderSystem {
    type SystemData = SystemDataT<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        use specs::join::Join;

        let _creature_tracker = data._creature_tracker;
        let _map = data._map;

        for (_entity, _, command, _pos) in (
            &data._entities,
            &data.logic,
            &mut data.commands,
            &mut data._pos,
        )
            .join()
        {
            let target_move = get_random_move();

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
