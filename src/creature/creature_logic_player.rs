/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/

// External includes
use specs::{
    Component,
    DenseVecStorage,
    Entities,
    ReadExpect,
    ReadStorage,
    System,
    WriteExpect,
    WriteStorage
};
use specs::prelude::*;

// Internal includes
use super::super::game_state::GameState;
use super::super::io::Window;
use crate::creature::{ CreatureTracker, PlayerMarker, PlayerPosition };
use crate::rrl_math::{ Displacement, Position };
use crate::world::Tilemap;

pub struct CreatureLogicPlayer {}

impl Component for CreatureLogicPlayer
{
    type Storage = DenseVecStorage<Self>;
}

pub struct CreatureLogicPlayerSystem;

#[derive(SystemData)]
pub struct SystemDataT< 'a >
{
    entities: Entities< 'a >,
    creature_tracker: ReadExpect< 'a, CreatureTracker >,
    map: ReadExpect< 'a, Tilemap >,
    window: ReadExpect< 'a, Window >,
    game_state: WriteExpect< 'a, GameState >,
    player_pos: WriteExpect< 'a, PlayerPosition >,
    player_marker: ReadStorage< 'a, PlayerMarker >,
    logic: WriteStorage< 'a, CreatureLogicPlayer >,
    pos: WriteStorage< 'a, Position >,
}

impl<'a> System<'a> for CreatureLogicPlayerSystem
{
    type SystemData = SystemDataT< 'a >;

    fn run( &mut self, mut data: SystemDataT )
    {
        use specs::Join;

        let creature_tracker = data.creature_tracker;
        let mut game_state = data.game_state;
        let map = data.map;
        let mut player_pos = data.player_pos;
        let window = data.window;

        for ( entity, _logic, pos, _ ) in
            ( &data.entities, &mut data.logic, &mut data.pos, &data.player_marker ).join()
        { 
            let command = window.get_char();
                
            let target_move;
            match command
            {
                '1' =>   target_move = Displacement::new(-1,  1),
                '2' =>   target_move = Displacement::new( 0,  1),
                '3' =>   target_move = Displacement::new( 1,  1),
                '4' =>   target_move = Displacement::new(-1,  0),
                '6' =>   target_move = Displacement::new( 1,  0),
                '7' =>   target_move = Displacement::new(-1, -1),
                '8' =>   target_move = Displacement::new( 0, -1),
                '9' =>   target_move = Displacement::new( 1, -1),
                'q' => { target_move = Displacement::new( 0,  0); game_state.kill(); },
                _ =>     target_move = Displacement::new( 0,  0),
            }

            let target_pos = *pos;
            let target_new_pos = target_pos + target_move;

            if map.passable_pos( target_new_pos ) &&
                creature_tracker.check_collision( entity, target_new_pos ) == None
            {
                *pos = target_new_pos;
            }

            player_pos.0 = *pos;
        }
    }
}
