extern crate rand;
use rand::{thread_rng, Rng};
mod creature;
mod io;
mod linear;
mod multidim;
mod tilemap;
mod tiletype;
use creature::Mobile;
use io::Window;
use linear::Displacement;
use tiletype::{ TILE_TYPE_DATA, TILE_TYPE_INDEX_VOID };

fn main() {
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

    let mut creatures: Vec<creature::Creature> = vec![];

    creatures.push(creature::Creature::new( 8, 5 ));
    let player_index: usize = 0;
    creatures.push(creature::Creature::new( 12, 7 ));

    {
        let player_pos = creatures[player_index].get_position();
        *map.tile_mut(player_pos.x as usize, player_pos.y as usize) = 2;
    }

    Window::init();
    let mut window = Window::new();

    let mut game_running = true;
    while game_running
    {
        let player_pos = creatures[player_index].get_position();
        window.write_map(player_pos.x, player_pos.y, &map);
        window.write_creatures(&creatures, player_index);
        window.present();

        // get_char refreshes the screen. Why??
        let command = window.get_char();
        let player_move;
        match command
        {
            '1' => player_move = Displacement::new(-1,  1),
            '2' => player_move = Displacement::new( 0,  1),
            '3' => player_move = Displacement::new( 1,  1),
            '4' => player_move = Displacement::new(-1,  0),
            '6' => player_move = Displacement::new( 1,  0),
            '7' => player_move = Displacement::new(-1, -1),
            '8' => player_move = Displacement::new( 0, -1),
            '9' => player_move = Displacement::new( 1, -1),
            'q' => { player_move = Displacement::new( 0,  0); game_running = false; },
            _ =>   player_move = Displacement::new( 0,  0),
        }

        let player_new_pos = player_pos + player_move;
        let tile_type;
        if (player_new_pos.x < 0) || (player_new_pos.x as usize >= map_width) ||
            (player_new_pos.y < 0) || (player_new_pos.y as usize >= map_height)
        {
            tile_type = TILE_TYPE_INDEX_VOID;
        }
        else
        {
            tile_type = map.tile( player_new_pos.x as usize, player_new_pos.y as usize );
        }
        let tile_type_data = &TILE_TYPE_DATA[ tile_type as usize ];
        if tile_type_data.passable()
        {
            creatures[player_index].move_self( player_move.x, player_move.y );
        }
    }

    Window::close();
}
