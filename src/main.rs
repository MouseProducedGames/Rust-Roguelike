// External dependencies
use rand::Rng;

// Internal dependencies.
mod creature;
mod creature_logic;
// mod creature_logic_none;
mod creature_logic_player;
mod creature_logic_wander;
mod creature_view;
mod game_state;
mod io;
mod linear;
mod multidim;
mod tilemap;
mod tiletype;
use creature::Creature;
use creature_logic::CreatureLogic;
// use creature_logic_none::CreatureLogicNone;
use creature_logic_player::CreatureLogicPlayer;
use creature_logic_wander::CreatureLogicWander;
use creature_view::CreatureView;
use game_state::GameState;
use io::Window;

fn main() {
    Window::init();
    let mut window = Window::new();

    let mut game_state: GameState = GameState::new( &mut window );

    let mut map: tilemap::Tilemap = tilemap::Tilemap::new(80, 25);
    let ( map_width, map_height) = map.bounds();

    {
        for x in 0..map_width
        {
            *map.tile_mut(x, 0) = 1_u32;
        }
        for x in 0..map_width
        {
            *map.tile_mut(x, map_height - 1) = 1;
        }
    }

    {
        for y in 1..map_height
        {
            *map.tile_mut(0, y) = 1;
            *map.tile_mut(map_width - 1, y) = 1;
        }
    }

    for y in 1..map_height - 1
    {
        for x in 1..map_width - 1
        {
            *map.tile_mut(x, y) = game_state.rng().gen_range(1, 3);
        }
    }

    // let creature_logic_none = CreatureLogicNone::new();
    let creature_logic_player = CreatureLogicPlayer::new();
    let creature_logic_wander = CreatureLogicWander::new();
    let mut creatures: Vec<Creature> = vec![];
    // let mut creatures: Vec<&CreatureView> = vec![];

    let player_index: usize;
    {
        {
            creatures.push(creature::Creature::new( &creature_logic_player as &dyn CreatureLogic, 8, 5 ));
            /* match creature_datas.last()
            {
                Some(creature) => creatures.push(creature),
            _ => (),
            } */
        }
        player_index = 0;
        {
            creatures.push(creature::Creature::new( &creature_logic_wander as &dyn CreatureLogic, 12, 7 ));
            /* match creature_datas.last()
            {
                Some(creature) => creatures.push(creature),
                _ => (),
            } */
        }
    }

    {
        let player_pos = creatures[player_index].get_position();
        *map.tile_mut(player_pos.x as usize, player_pos.y as usize) = 2;
    }

    while game_state.alive()
    {
        {
            for creature in &mut creatures
            {
                creature.calculate_visibility(&map);
            }
        }

        {
            let player_pos = creatures[ player_index ].get_position();
            game_state.window_mut().write_map( &creatures[ player_index ], &map );
            let test = Box::new( creatures.iter().map( |e| e as &CreatureView ) );
            game_state.window_mut().write_creatures( player_pos, test, player_index );
            game_state.window_mut().present();
        }

        {
            for creature in &mut creatures
            {
                creature.update( &map, &mut game_state );
            }
        }
    }

    Window::close();
}
