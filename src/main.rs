/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

 **/

// External dependencies
extern crate shred;
#[macro_use]
extern crate shred_derive;
use specs::{ Builder, /* System, */ World, RunNow };

// Internal dependencies.
mod creature;
mod dungen;
mod faction;
mod game_state;
mod io;
mod rrl_math;
mod multidim;
mod multimap;
mod stats;
mod tiled_shapes_2d;
mod world;
use creature::{
    Command,
    CreatureCommandSystem,
    CreatureDisplaySystem,
    CreatureLogicPlayer,
    CreatureLogicPlayerSystem,
    CreatureLogicWander,
    CreatureLogicWanderSystem,
    CreatureStats,
    CreatureTracker,
    CreatureVisibilitySystem,
    PlayerDisplaySystem,
    PlayerMarker,
    PlayerPosition,
    SightRange,
    ViewpointMarker,
    Visibility
};
use dungen::{ DungeonGenerator, SplitDungeon, /* RandomlyTileDungeon, */ SplitType };
use faction::Faction;
use game_state::GameState;
use io::Window;
use rrl_math::{ Bounds,  Position };
use world::Tilemap;

fn main() {

    // Window::init();
    let mut game_state = GameState::new();

    let map;
    {
        let mut temp_map: Tilemap = Tilemap::new( 40, 30 );
        //     let mut boxed_map: Box<dyn TiledArea> = Box::new( temp_map );
        SplitDungeon::new(
            SplitType::LongestDimension,
            Bounds { width: 6, height: 6 },
            3, 2, 1,
            &mut game_state.rng()
        ).apply( &mut temp_map );
        
        map = temp_map;
    }

    let mut world = World::new();
    world.add_resource( CreatureTracker::new() );
    world.add_resource( game_state );
    world.add_resource( map );
    world.add_resource( Window::new() );
    world.add_resource( PlayerPosition( Position::new( 8, 5 ) ) );
    world.register::< Command >();
    world.register::< CreatureLogicPlayer >();
    world.register::< CreatureLogicWander >();
    world.register::< CreatureStats >();
    world.register::< Faction >();
    world.register::< PlayerMarker >();
    world.register::< Position >();
    world.register::< SightRange >();
    world.register::< ViewpointMarker >();
    world.register::< Visibility >();

    world
        .create_entity()
        .with( Command::None )
        .with( CreatureLogicPlayer {} )
        .with( Faction::new( 0 ) )
        .with( CreatureStats::default() )
        .with( Position::new( 8, 5 ) )
        .with( PlayerMarker )
        .with( SightRange::new( 5 ) )
        .with( ViewpointMarker )
        .with( Visibility::new() )
        .build();
    {
        *(&mut *world.write_resource::< Tilemap >()).tile_type_mut( 8, 5 ) = 2;
    }
    
    /* world.
        create_entity()
        .with( Command::None )
        .with( CreatureLogicWander )
        .with( Faction::new( 0 ) )
        .with( Position::new( 12, 8 ) )
        .with( SightRange::new( 5 ) )
        .with( Visibility::new() )
    .build(); */

    world.
        create_entity()
        .with( Command::None )
        .with( CreatureLogicWander )
        .with( Faction::new( 0 ) )
        .with( CreatureStats::default() )
        .with( Position::new( 8, 12 ) )
        .with( SightRange::new( 5 ) )
        .with( Visibility::new() )
        .build();

    let mut creature_command_system = CreatureCommandSystem;
    let mut creature_display_system = CreatureDisplaySystem;
    let mut creature_player_logic = CreatureLogicPlayerSystem;
    let mut creature_wander_logic = CreatureLogicWanderSystem;
    let mut creature_visibility_system = CreatureVisibilitySystem;
    let mut player_display_system = PlayerDisplaySystem;

    while world.read_resource::< GameState >().alive()
    {
        creature_visibility_system.run_now( &world.res );

        player_display_system.run_now( &world.res );
        
        creature_display_system.run_now( &world.res );

        world.maintain();

        world.write_resource::< Window >().present();
        
        creature_player_logic.run_now( &world.res );

        world.maintain();

        creature_wander_logic.run_now( &world.res );

        world.maintain();

        creature_command_system.run_now( &world.res );
        
        world.maintain();
    }

    // Window::close();
}
