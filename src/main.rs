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
use ncurses;
use creature::{ CreatureLogicPlayer, CreatureLogicPlayerSystem /* Faction, */ /* SightRange, */ /* System, */ /* Visibility, */ };
// use creature_logic_none::CreatureLogicNone;
use game_state::GameState;
use rrl_math::{ Position };
use specs::{ Builder, /* System, */ World, RunNow };
// use io::Window;
use world::{ Mapping, Tilemap };

fn main() {
    ncurses::initscr();
    ncurses::keypad(ncurses::stdscr(), true);
    // ncurses::raw();
    ncurses::curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    ncurses::nonl();
    ncurses::cbreak();
    ncurses::noecho();

    // Window::init();
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
    world.register::<CreatureLogicPlayer>();
    world.register::<Position>();
    world.register::<Tilemap>();

    // let player_faction = game_state.factions_mut().add();
    // let _monster_faction = game_state.factions_mut().add();
    
    // let creature_logic_none = CreatureLogicNone::new();
    // let creature_logic_player = CreatureLogicPlayer::new();
    // let creature_logic_wander = CreatureLogicWander::new();
    // let mut creatures: Vec< Creature > = vec![];
    // let mut creatures: Vec< &CreatureView > = vec![];

    world
        .create_entity()
        .with(CreatureLogicPlayer {})
        .with(Position::new( 8, 5 ))
        .with(map)
        .build();

    /* let player_index: usize;
    {
        {
            creatures.push(
                creature::Creature::new(
                    &creature_logic_player as &dyn CreatureLogic,
                    8, 5,
                    player_faction
                )
            );
            /* match creature_datas.last()
            {
                Some( creature ) => creatures.push( creature ),
            _ => (),
            } */
        }
        player_index = 0;
        {
            creatures.push(
                creature::Creature::new(
                    &creature_logic_wander as &dyn CreatureLogic,
                    12, 7,
                    player_faction
                )
            );
            /* match creature_datas.last()
            {
                Some( creature ) => creatures.push( creature ),
                _ => (),
            } */
        }
    } */

/*     {
        let player_pos = creatures[ player_index ].get_position();
        *map.tile_type_mut( player_pos.x as usize, player_pos.y as usize ) = 2;
    } */

    let mut test = CreatureLogicPlayerSystem;

    while game_state.alive()
    {
        test.run_now(&world.res);
        world.maintain();
        
        /*         for y in 0..map.width()
        {
            for x in 0..map.height()
            {
                match map.tile_type( x, y ) {
                    0 => 
                }
            }
        } */
/*         {
            for creature in &mut creatures
            {
                creature.calculate_visibility( &map );
            }
        }

        {
            let window = game_state.window_mut();
            let player_pos = creatures[ player_index ].get_position();
            window.write_map( &creatures[ player_index ], &map );
            let test = Box::new( creatures.iter().map( |e| e as &CreatureView ) );
            window.write_creatures( player_pos, test, player_index );
            window.present();
        }

        {
            for creature in &mut creatures
            {
                creature.update( &map, &mut game_state );
            }
        } */
    }

    ncurses::endwin();

    // Window::close();
}
