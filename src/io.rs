extern crate ncurses;
use super::creature;
use super::linear::{ Position };
use super::tilemap;

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

pub fn write_creatures(creatures: &Vec<creature::Creature>, player_index: usize)
{
    let player_creature = &creatures[player_index];
    for creature_index in 0..creatures.len()
    {
        let pc_pos = player_creature.get_position();
        let current_creature = &creatures[creature_index];
        let ch = if creature_index == player_index { '@' } else { 'C' };
        let cc_pos = current_creature.get_position();
        let dist = cc_pos - pc_pos;
        if (dist.x < -17) || (dist.x > 17) ||
           (dist.y < -17) || (dist.y > 17)
        {
            continue;
        }

        let display_pos = Position::new(18, 18) + dist;
        put_char( display_pos.x, display_pos.y, ch );
    }
}

pub fn write_map(view_x: i32, view_y: i32, map: &tilemap::Tilemap)
{
    let ( map_width, map_height ) = map.get_bounds();
    for view_addend_y in -17..18_i32
    {
        let map_pos_y = view_y + view_addend_y;
        let check_y;
        if map_pos_y < 0
        {
            check_y = false;
            // continue;
        }
        else if (map_pos_y as usize) >= map_height
        {
            check_y = false;
            // break;
        }
        else
        {
            check_y = true;
        }
        let display_pos_y = 18 + view_addend_y;
        let map_index_y = map_pos_y as usize;
        let mut repeat_count = 1;
        let mut lastch = ' ';
        for view_addend_x in -17..18_i32
        {
            let map_pos_x = view_x + view_addend_x;
            let check_x;
            if map_pos_x < 0
            {
                check_x = false;
                // continue;
            }
            else if (map_pos_x as usize) >= map_width
            {
                check_x = false;
                // break;
            }
            else
            {
                check_x = true;
            }
            let check = check_x && check_y;
            let display_pos_x = 18 + view_addend_x;
            let tile_type;
            if check
            {
                let map_index_x = map_pos_x as usize;
                tile_type = map.get_tile(map_index_x, map_index_y);
            }
            else
            {
                tile_type = 0;
            }
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
                // ncurses::refresh();
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
