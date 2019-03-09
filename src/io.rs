extern crate ncurses;
use super::creature;

static MAP_GRAPHICS: [char; 3] = [ ' ', '#', '.' ];

pub fn init()
{
    ncurses::initscr();
    ncurses::keypad(ncurses::stdscr(), true);
    // ncurses::raw();
    ncurses::curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    ncurses::nonl();
    ncurses::cbreak();
    ncurses::noecho();
}

pub fn close()
{
    ncurses::endwin();
}

pub fn get_char() -> char
{
    return match
        std::char::from_u32(ncurses::getch() as u32)
        {
            None => ' ',
            Some(v) => v,
        };
}

pub fn refresh()
{
    ncurses::refresh();
}

/* pub fn write_line(s: &str)
{
    ncurses::printw(s);
} */

pub fn write_creatures(creatures: Vec<creature::Creature>, player_index: usize)
{
    let player_creature = &creatures[player_index];
    let ( pc_pos_x, pc_pos_y ) = player_creature.get_position();
    for creature_index in 0..creatures.len()
    {
        let current_creature = &creatures[creature_index];
        let ch = if creature_index == player_index { '@' } else { 'C' };
        let ( cc_pos_x, cc_pos_y ) = current_creature.get_position();
        let ( dist_x, dist_y ) = ( cc_pos_x - pc_pos_x, cc_pos_y - pc_pos_y );
        if (dist_x < -17) || (dist_x > 17) ||
           (dist_y < -17) || (dist_y > 17)
        {
            continue;
        }

        let ( display_pos_x, display_pos_y ) = ( 18 + dist_x, 18 + dist_y );
        put_char( display_pos_x, display_pos_y, ch );
    }
}

pub fn write_map(view_x: i32, view_y: i32, map: [[u32; 80]; 25])
{
    for view_addend_y in -17..18_i32
    {
        let map_pos_y = view_y + view_addend_y;
        if map_pos_y < 0
        {
            continue;
        }
        else if map_pos_y >= 25
        {
            break;
        }
        let display_pos_y = 18 + view_addend_y;
        let map_index_y = map_pos_y as usize;
        let row = map[map_index_y];
        let mut repeat_count = 1;
        let mut lastch = ' ';
        for view_addend_x in -17..18_i32
        {
            let map_pos_x = view_x + view_addend_x;
            if map_pos_x < 0
            {
                continue;
            }
            else if map_pos_x >= 80
            {
                break;
            }
            let display_pos_x = 18 + view_addend_x;
            let map_index_x = map_pos_x as usize;
            let tile_type = row[map_index_x];
            let ch = MAP_GRAPHICS[tile_type as usize];
            // let ch = match *is_wall { true => '#', _ => '.', };
            if lastch == ch
            {
                repeat_count = repeat_count + 1;
            }
            else
            {
                repeat_count = 1;
            }
            if repeat_count == 5
            {
                ncurses::refresh();
                repeat_count = 0;
            }
            lastch = ch;
            put_char(display_pos_x, display_pos_y, ch);
        }
    }    
}

/* fn move_cursor(x: i32, y: i32)
{
    ncurses::mv(y, x);
} */

/* fn write_char(ch: char)
{
    ncurses::addch(ch as u64);
} */

fn put_char(x: i32, y: i32, ch: char)
{
    ncurses::mvaddch(y, x, ch as u64);
}
