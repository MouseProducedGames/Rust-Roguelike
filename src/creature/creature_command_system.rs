/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/

// External dependencies.
use specs::{
    Entities,
    ReadExpect,
    ReadStorage,
    System,
    WriteExpect,
    WriteStorage
};

// Internal dependencies.
use crate::creature::{
    Command,
    CreatureTracker,
    PlayerPosition,
    ViewpointMarker,
};
use crate::rrl_math::Position;
use crate::world::Tilemap;

pub struct CreatureCommandSystem;

#[derive(SystemData)]
pub struct SystemDataT< 'a >
{
    creature_tracker: ReadExpect< 'a, CreatureTracker >,
    entities: Entities< 'a >,
    map: ReadExpect< 'a, Tilemap >,
    player_pos: WriteExpect< 'a, PlayerPosition >,
    command: ReadStorage< 'a, Command >,
    viewpoint: ReadStorage< 'a, ViewpointMarker >,
    pos: WriteStorage< 'a, Position >,
}

impl<'a> System<'a> for CreatureCommandSystem
{
    type SystemData = SystemDataT< 'a >;
    
    fn run( &mut self, mut data: Self::SystemData )
    {
        use specs::join::Join;

        let creature_tracker = data.creature_tracker;
        let map = data.map;

        for ( entity, command, pos ) in ( &data.entities, &data.command, &mut data.pos ).join()
        {
            let command = command;

            match *command {
                Command::Move( disp ) => {
                    let new_pos = *pos + disp;

                    if map.passable_pos( new_pos ) &&
                        creature_tracker.check_collision( entity, new_pos ) == None
                    {
                        *pos = new_pos;
                    }                    
                }
                _ => (),
            }
        }

        for ( pos, _ ) in ( &data.pos, &data.viewpoint ).join()
        {
            data.player_pos.0 = *pos;
        }
    }
}
