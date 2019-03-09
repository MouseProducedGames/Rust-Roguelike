extern crate rand;
use rand::{thread_rng, Rng};
mod io;

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

    let player_pos_x: usize = 8;
    let player_pos_y: usize = 5;

    map[player_pos_y][player_pos_x] = 2;

    io::init();

    io::write_map(player_pos_x, player_pos_y, map);
    io::write_map(player_pos_x, player_pos_y, map);

    io::refresh();

    // get_char refreshes the screen. Why??
    io::get_char();

    io::close();
}
