/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

 **/

// External dependencies
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
mod tiled_shapes_2d;
mod world;
use creature::{
    CreatureDisplaySystem,
    CreatureLogicPlayer,
    CreatureLogicPlayerSystem,
    CreatureLogicWander,
    CreatureLogicWanderSystem,
    CreatureVisibilitySystem,
    PlayerDisplaySystem,
    PlayerMarker,
    PlayerPosition,
    SightRange,
    Visibility
};
use dungen::{ DungenCommon, SplitDungeon, /* RandomlyTileDungeon, */ SplitType };
use faction::Faction;
use game_state::GameState;
use io::Window;
use rrl_math::{ Bounds,  Position };
use tiled_shapes_2d::TiledRect;
use world::{ TiledArea, Tilemap };

fn main() {

    Window::init();
    let mut game_state = GameState::new();
    
    let temp_map: Tilemap = Tilemap::new( 40, 30 );
    let boxed_map: Box<dyn TiledArea> = Box::new( temp_map );
    let map =
        boxed_map
    // .randomly_tile_dungeon( 1, 3, &mut game_state.rng() )
        .split_dungeon(
            SplitType::LongestDimension,
            Bounds { width: 6, height: 6 },
            3, 2, 1,
            &mut game_state.rng() )
        .finish();

    let mut world = World::new();
    world.add_resource( game_state );
    world.add_resource( map );
    world.add_resource( Window::new() );
    world.add_resource( PlayerPosition( Position::new( 8, 5 ) ) );
    world.register::< CreatureLogicPlayer >();
    world.register::< CreatureLogicWander >();
    world.register::< Faction >();
    world.register::< PlayerMarker >();
    world.register::< Position >();
    world.register::< SightRange >();
    world.register::< Visibility >();

    world
        .create_entity()
        .with( CreatureLogicPlayer {} )
        .with( Faction::new( 0 ) )
        .with( Position::new( 8, 5 ) )
        .with( PlayerMarker )
        .with( SightRange::new( 5 ) )
        .with( Visibility::new() )
        .build();
    {
        *(&mut *world.write_resource::< Tilemap >()).tile_type_mut( 8, 5 ) = 2;
    }
    
    world.
        create_entity()
        .with( CreatureLogicWander )
        .with( Faction::new( 0 ) )
        .with( Position::new( 12, 8 ) )
        .with( SightRange::new( 5 ) )
        .with( Visibility::new() )
        .build();

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
    }

    Window::close();
}
