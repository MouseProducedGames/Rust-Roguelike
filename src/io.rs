extern crate ncurses;

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

pub fn write_map(map: [[u32; 80]; 25])
{
    let mut y = -1;
    for row in map.iter()
    {
        y = y + 1;
        let mut x = -1;
        let mut repeat_count = 1;
        let mut lastch = ' ';
        for tile_type in row.iter()
        {
            x = x + 1;
            let ch = MAP_GRAPHICS[*tile_type as usize];
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
            put_char(x, y, ch);
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
