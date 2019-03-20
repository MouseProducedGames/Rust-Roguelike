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
use rand::Rng;

// Internal dependencies.
use super::super::GameState;
use crate::creature::{
    Command,
    CreatureStats,
    CreatureTracker,
    PlayerPosition,
    ViewpointMarker,
};
use crate::rrl_math::Position;
use crate::stats::{ Stat, StatModifier };
use crate::world::Tilemap;

pub struct CreatureCommandSystem;

#[derive(SystemData)]
pub struct SystemDataT< 'a >
{
    creature_tracker: ReadExpect< 'a, CreatureTracker >,
    entities: Entities< 'a >,
    map: ReadExpect< 'a, Tilemap >,
    player_pos: WriteExpect< 'a, PlayerPosition >,
    game_state: WriteExpect< 'a, GameState >,
    command: ReadStorage< 'a, Command >,
    viewpoint: ReadStorage< 'a, ViewpointMarker >,
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
                                
                                let attack_mod = attacker_stats.coordination().modifier();
                                let defence_mod = defender_stats.agility().modifier();

                                let attack_roll =
                                    game_state.rng().gen_range( 1, 7 ) +
                                    game_state.rng().gen_range( 1, 7 ) +
                                    game_state.rng().gen_range( 1, 7 ) +
                                    attack_mod;
                                let defence_total = 10 + defence_mod;

                                if attack_roll > defence_total
                                {
                                    let damage_mod = 5 + defender_stats.strength().modifier();
                                    let new_defender_health = defender_stats.health() - damage_mod;
                                    *defender_stats.health_mut() = new_defender_health;
                                    /* match data.health.get_mut( other_entity ) {
                                        Some( tmut_health ) => *tmut_health = new_defender_health,
                                        _ => panic!( " Could not access component which has to exist!" ),
                                    } */
                                    if defender_stats.health().value() <= 0
                                    {
                                        match data.entities.delete( other_entity ) {
                                            Ok( _ ) => (),
                                            _ => panic!( " Could not delete entity which has to exist!" ),
                                        }
                                    }
                                }
                            }
                            None => *pos = new_pos,
                        }
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
