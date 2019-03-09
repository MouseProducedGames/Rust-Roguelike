extern crate rand;
use rand::{thread_rng, Rng};
mod creature;
mod io;
use crate::creature::Mobile;

fn main() {
    let mut rng = thread_rng();
    let mut map: [[u32; 80]; 25] = [[0; 80]; 25];

    {
        let top: &mut [u32; 80] = &mut map[0];
        for x in 0..80
        {
            top[x] = 1;
        }
        let bottom: &mut [u32; 80] = &mut map[24];
        for x in 0..80
        {
            bottom[x] = 1;
        }
    }

    {
        for y in 1..24
        {
            let row: &mut [u32; 80] = &mut map[y];
            row[0] = 1;
            row[79] = 1;
        }
    }

    for y in 1..24
    {
        let row: &mut [u32; 80] = &mut map[y];
        for x in 1..79
        {
            row[x] = rng.gen_range(1, 3);
        }
    }

    let mut creatures: Vec<creature::Creature> = vec![];

    creatures.push(creature::Creature::new( 8, 5 ));
    let player_index: usize = 0;
    creatures.push(creature::Creature::new( 12, 7 ));

    {
        let (player_pos_x, player_pos_y) = creatures[player_index].get_position();
        map[player_pos_y as usize][player_pos_x as usize] = 2;
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
