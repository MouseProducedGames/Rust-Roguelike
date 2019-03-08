extern crate rand;
use rand::{thread_rng, Rng};
mod io;

fn main() {
    let mut rng = thread_rng();
    let mut map: [[bool; 80]; 25] = [[false; 80]; 25];

    {
        let top: &mut [bool; 80] = &mut map[0];
        for x in 0..80
        {
            top[x] = true;
        }
        let bottom: &mut [bool; 80] = &mut map[24];
        for x in 0..80
        {
            bottom[x] = true;
        }
    }

    {
        for y in 1..24
        {
            map[y][0] = true;
            map[y][79] = true;
        }
    }

    for y in 1..24
    {
        let row: &mut [bool; 80] = &mut map[y];
        for x in 1..79
        {
            row[x] = rng.gen_bool(0.5);
        }
    }

    io::init();

    io::write_map(map);
    io::write_map(map);

    io::refresh();

    // get_char refreshes the screen. Why??
    io::get_char();

    io::close();
}
