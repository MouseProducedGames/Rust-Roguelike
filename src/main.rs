extern crate rand;
use rand::{thread_rng, Rng};
mod creature;
mod creature_logic;
mod creature_logic_none;
mod creature_logic_player;
mod game_state;
mod io;
mod linear;
mod multidim;
mod tilemap;
mod tiletype;
use creature_logic::CreatureLogic;
use creature_logic_none::CreatureLogicNone;
use creature_logic_player::CreatureLogicPlayer;
use game_state::GameState;
use io::Window;

fn main() {
    Window::init();
    let mut window = Window::new();

    let mut game_state: GameState = GameState::new( &mut window );

    let mut rng = thread_rng();
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
            *map.tile_mut(x, y) = rng.gen_range(1, 3);
        }
    }

    let creature_logic_none = CreatureLogicNone::new();
    let creature_logic_player = CreatureLogicPlayer::new();
    let mut creatures: Vec<creature::Creature> = vec![];

    creatures.push(creature::Creature::new( &creature_logic_player as &dyn CreatureLogic, 8, 5 ));
    let player_index: usize = 0;
    creatures.push(creature::Creature::new( &creature_logic_none as &dyn CreatureLogic, 12, 7 ));

    {
        let player_pos = creatures[player_index].get_position();
        *map.tile_mut(player_pos.x as usize, player_pos.y as usize) = 2;
    }

    while game_state.alive()
    {
        {
            let player_pos = creatures[player_index].get_position();
            game_state.window_mut().write_map(player_pos.x, player_pos.y, &map);
            game_state.window_mut().write_creatures(&creatures, player_index);
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
