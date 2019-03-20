/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/

// External dependencies.
use specs::{
    Entities,
    Entity,
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

        let creature_tracker = &*data.creature_tracker;
        let game_state = &mut data.game_state;
        let map = data.map;
        let stats = &mut data.stats;

        for ( entity, command, pos ) in
            ( &data.entities, &data.command, &mut data.pos ).join()
        {
            let command = command;

            match *command {
                Command::Move( disp ) => {
                    let new_pos = *pos + disp;

                    if map.passable_pos( new_pos )
                    {
                        impassable_movement( entity, new_pos, pos, creature_tracker, game_state, stats );
                    }                    
                }
                _ => (),
            }
        }
    }
}

fn impassable_movement<'a>(
    entity: Entity,
    new_pos: Position,
    pos: &mut Position,
    creature_tracker: &CreatureTracker,
    game_state: &mut WriteExpect< 'a, GameState >,
    stats: &mut WriteStorage< 'a, CreatureStats >
)
{
    match creature_tracker.check_collision( entity, new_pos ) {
        Some( other_entity ) => {
            let attacker_stats; 
            let defender_stats;
            match stats.get( entity ) {
                Some( stats ) => attacker_stats = *stats,
                _ => return,
            }
            match stats.get_mut( other_entity ) {
                Some( stats ) => defender_stats = stats,
                _ => return,
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
