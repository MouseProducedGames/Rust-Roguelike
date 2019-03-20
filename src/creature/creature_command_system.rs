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
use super::super::GameState;
use crate::creature::{
    Command,
    CreatureStats,
    CreatureTracker,
};
use crate::game::{ Combat, CombatResult };
use crate::rrl_math::Position;
use crate::stats::Stat;
use crate::world::Tilemap;

pub struct CreatureCommandSystem;

#[derive(SystemData)]
pub struct SystemDataT< 'a >
{
    creature_tracker: ReadExpect< 'a, CreatureTracker >,
    entities: Entities< 'a >,
    map: ReadExpect< 'a, Tilemap >,
    game_state: WriteExpect< 'a, GameState >,
    command: ReadStorage< 'a, Command >,
    stats: WriteStorage< 'a, CreatureStats >,
    pos: WriteStorage< 'a, Position >,
}

impl<'a> System<'a> for CreatureCommandSystem
{
    type SystemData = SystemDataT< 'a >;
    
    fn run( &mut self, mut data: Self::SystemData )
    {
        use specs::join::Join;

        let creature_tracker = data.creature_tracker;
        let mut game_state = data.game_state;
        let map = data.map;

        for ( entity, command, pos ) in
            ( &data.entities, &data.command, &mut data.pos ).join()
        {
            let command = command;

            match *command {
                Command::Move( disp ) => {
                    let new_pos = *pos + disp;

                    if map.passable_pos( new_pos )
                    {
                        match creature_tracker.check_collision( entity, new_pos ) {
                            Some( other_entity ) => {
                                let attacker_stats; 
                                let defender_stats;
                                match data.stats.get( entity ) {
                                    Some( stats ) => attacker_stats = *stats,
                                    _ => continue,
                                }
                                match data.stats.get_mut( other_entity ) {
                                    Some( stats ) => defender_stats = stats,
                                    _ => continue,
                                }

                                match Combat::one_attack(
                                    &mut *game_state,
                                    &attacker_stats,
                                    defender_stats
                                ) {
                                    CombatResult::DefenderDead => {
                                        (*defender_stats.health_mut().value_mut()).min( -100 );
                                    },
                                    _ => (),
                                }
                            }
                            None => *pos = new_pos,
                        }
                    }                    
                }
                _ => (),
            }
        }
    }
}
