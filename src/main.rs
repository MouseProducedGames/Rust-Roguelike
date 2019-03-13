// External dependencies
use rand::Rng;

// Internal dependencies.
mod creature;
// mod creature_logic_none;
mod faction;
mod game_state;
mod io;
mod rrl_math;
mod multidim;
mod world;
use creature::{
    CreatureDisplaySystem,
    CreatureLogicPlayer,
    CreatureLogicPlayerSystem,
    PlayerDisplaySystem,
    PlayerMarker,
    PlayerPosition
};
// use creature_logic_none::CreatureLogicNone;
use game_state::GameState;
use rrl_math::{ Position };
use specs::{ Builder, /* System, */ World, RunNow };
use io::Window;
use world::{ Mapping, Tilemap };

fn main() {
    Window::init();
    let mut game_state = GameState::new();
    
    let mut map: Tilemap = Tilemap::new( 80, 25 );
    let ( map_width, map_height ) = map.bounds();

    {
        for x in 0..map_width
        {
            *map.tile_type_mut(x, 0) = 1_u32;
        }
        for x in 0..map_width
        {
            *map.tile_type_mut(x, map_height - 1) = 1;
        }
    }

    {
        for y in 1..map_height
        {
            *map.tile_type_mut( 0, y ) = 1;
            *map.tile_type_mut( map_width - 1, y ) = 1;
        }
    }

    {
        let mut rng = game_state.rng();
        for y in 1..map_height - 1
        {
            for x in 1..map_width - 1
            {
                *map.tile_type_mut( x, y ) = rng.gen_range( 1, 3 );
            }
        }
    }

    let mut world = World::new();
    world.add_resource( game_state );
    world.add_resource( map );
    world.add_resource( Window::new() );
    world.add_resource( PlayerPosition( Position::new( 8, 5 ) ) );
    world.register::< CreatureLogicPlayer >();
    world.register::< PlayerMarker >();
    world.register::< Position >();

    world
        .create_entity()
        .with( CreatureLogicPlayer {}) 
        .with( Position::new( 8, 5 ) )
        .with( PlayerMarker )
        .build();

    let mut creature_display_system = CreatureDisplaySystem;
    let mut creature_player_logic = CreatureLogicPlayerSystem;
    let mut player_display_system = PlayerDisplaySystem;

    while world.read_resource::< GameState >().alive()
    {
        creature_player_logic.run_now( &world.res );

        world.maintain();

        player_display_system.run_now( &world.res );
    
        creature_display_system.run_now( &world.res );

        world.write_resource::< Window >().present();
    }

    Window::close();
}
