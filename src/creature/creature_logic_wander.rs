/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/

// External dependencies.
use specs::{
    Component,
    Entities,
    NullStorage,
    ReadExpect,
    ReadStorage,
    System,
    WriteExpect,
    WriteStorage
};
use rand::Rng;

// Internal dependencies.
use super::super::game_state::GameState;
use crate::creature::CreatureTracker;
use crate::rrl_math::{ Displacement, Position };
use crate::world::Tilemap;

#[derive(Default)]
pub struct CreatureLogicWander;

impl Component for CreatureLogicWander
{
    type Storage = NullStorage<Self>;
}

pub struct CreatureLogicWanderSystem;

#[derive(SystemData)]
pub struct SystemDataT< 'a >
{
    creature_tracker: ReadExpect< 'a, CreatureTracker >,
    entities: Entities< 'a >,
    map: ReadExpect< 'a, Tilemap >,
    logic: ReadStorage< 'a, CreatureLogicWander >,
    game_state: WriteExpect< 'a, GameState >,
    pos: WriteStorage< 'a, Position >,    
}

impl<'a> System<'a> for CreatureLogicWanderSystem
{
    type SystemData = SystemDataT< 'a >;
    
    fn run( &mut self, mut data: Self::SystemData )
    {
        use specs::join::Join;

        let creature_tracker = data.creature_tracker;
        let mut game_state = data.game_state;
        let map = data.map;

        for ( entity, _, pos ) in ( &data.entities, &data.logic, &mut data.pos ).join()
        {
            let command = game_state.rng().gen_range(1, 10);
            let target_move;
            match command
            {
                1 =>   target_move = Displacement::new(-1,  1),
                2 =>   target_move = Displacement::new( 0,  1),
                3 =>   target_move = Displacement::new( 1,  1),
                4 =>   target_move = Displacement::new(-1,  0),
                5 =>   target_move = Displacement::new( 0,  0),
                6 =>   target_move = Displacement::new( 1,  0),
                7 =>   target_move = Displacement::new(-1, -1),
                8 =>   target_move = Displacement::new( 0, -1),
                9 =>   target_move = Displacement::new( 1, -1),
                _ =>   target_move = Displacement::new( 0,  0),
            }

            let new_pos = *pos + target_move;

            if map.passable_pos( new_pos ) &&
                creature_tracker.check_collision( entity, new_pos ) == None
            {
                *pos = new_pos;
            }
        }
    }
}
