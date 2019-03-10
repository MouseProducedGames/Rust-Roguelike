extern crate rand;
use rand::{thread_rng, Rng};
mod creature;
mod io;
mod tilemap;
use crate::creature::Mobile;

fn main() {
    let mut rng = thread_rng();
    let mut map: tilemap::Tilemap = tilemap::Tilemap::new(80, 25);
    let ( map_width, map_height) = map.get_bounds();

    {
        for x in 0..map_width
        {
            map.set_tile(x, 0, 1);
        }
        for x in 0..map_width
        {
            map.set_tile(x, map_height - 1, 1);
        }
    }

    {
        for y in 1..map_height
        {
            map.set_tile(0, y, 1);
            map.set_tile(map_width - 1, y, 1);
        }
    }

    for y in 1..map_height - 1
    {
        for x in 1..map_width - 1
        {
            map.set_tile(x, y, rng.gen_range(1, 3));
        }
    }

    let mut creatures: Vec<creature::Creature> = vec![];

    creatures.push(creature::Creature::new( 8, 5 ));
    let player_index: usize = 0;
    creatures.push(creature::Creature::new( 12, 7 ));

    {
        let (player_pos_x, player_pos_y) = creatures[player_index].get_position();
        map.set_tile(player_pos_x as usize, player_pos_y as usize, 2);
    }

    io::init();

    let mut game_running = true;
    while game_running
    {
        let (player_pos_x, player_pos_y) = creatures[player_index].get_position();
        io::write_map(player_pos_x, player_pos_y, &map);
        io::write_creatures(&creatures, player_index);
        io::refresh();

        // get_char refreshes the screen. Why??
        let command = io::get_char();
        match command
        {
            '1' => creatures[player_index].move_self(-1,  1),
            '2' => creatures[player_index].move_self( 0,  1),
            '3' => creatures[player_index].move_self( 1,  1),
            '4' => creatures[player_index].move_self(-1,  0),
            '6' => creatures[player_index].move_self( 1,  0),
            '7' => creatures[player_index].move_self(-1, -1),
            '8' => creatures[player_index].move_self( 0, -1),
            '9' => creatures[player_index].move_self( 1, -1),
            'q' => game_running = false,
            _ => (),
        }
    }

    io::close();
}
