extern crate ncurses;
use super::creature;
use super::multidim::Multidim;
use super::tilemap;

static MAP_GRAPHICS: [char; 3] = [ ' ', '#', '.' ];

pub struct Window
{
    buffers: [Multidim<char>; 2],
    back_buffer_index: usize,
}

impl Window
{
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

    pub fn new() -> Self
    {
        let mut output =
            Self {
                buffers: [
                    Multidim::new( 40, 80 ),
                    Multidim::new( 40, 80 )
                ],
                back_buffer_index: 0,
            };
        
        let front_buffer_index = output.front_buffer_index();
        let buffers = &mut output.buffers;
        let ( buffer_height, buffer_width ) = buffers[ output.back_buffer_index ].bounds();
        for y in 0..buffer_height
        {
            for x in 0..buffer_width
            {
                *buffers[ front_buffer_index ].value_mut( y, x ) = ' ';
                *buffers[ output.back_buffer_index ].value_mut( y, x) = ' ';
            }
        }
        
        output
    }

    pub fn get_char(&self) -> char
    {
        return match
            std::char::from_u32(ncurses::getch() as u32)
            {
                None => ' ',
                Some(v) => v,
            };
    }

    pub fn present(&mut self)
    {
        {
            self.back_buffer_index = 1 - self.back_buffer_index;
            let buffers = &self.buffers;
            let back_buffer = &buffers[ self.back_buffer_index ];
            let front_buffer = &buffers[ self.front_buffer_index() ];
            let ( buffer_height, buffer_width ) = front_buffer.bounds();
            let mut yi = -1_i32;
            for y in 0..buffer_height
            {
                yi += 1;
                let mut xi = -1_i32;
                // Without the repeat_count check, repeated characters (5 or more)
                // are placed in the same spot.
                let mut repeat_count = 1;
                let mut lastch = ' ';
                for x in 0..buffer_width
                {
                    xi += 1;
                    let back_ch = *back_buffer.value( y, x );
                    let front_ch = *front_buffer.value( y, x );
                    if front_ch == back_ch
                    {
                        // lastch = front_ch;
                        repeat_count = repeat_count + 1;
                        if repeat_count >= 5
                        {
                            ncurses::refresh();
                            repeat_count = 0;
                        }
                        continue;
                    }
        
                    if lastch == front_ch
                    {
                        repeat_count = repeat_count + 1;
                    }
                    else
                    {
                        repeat_count = 1;
                    }
                    if repeat_count >= 5
                    {
                        ncurses::refresh();
                        repeat_count = 0;
                    }
                    lastch = front_ch;
    
                    self.put_char(xi, yi, front_ch);
                }
            }
        }
        ncurses::refresh();

        {
            let front_buffer_index = self.front_buffer_index();
            let buffers = &mut self.buffers;
            let ( buffer_height, buffer_width ) = buffers[ self.back_buffer_index ].bounds();
            for y in 0..buffer_height
            {
                for x in 0..buffer_width
                {
                    let val = *buffers[ front_buffer_index ].value( y, x );
                    *buffers[ self.back_buffer_index ].value_mut( y, x) = val;
                }
            }
        }
    }

    /* pub fn write_line(s: &str)
    {
        ncurses::printw(s);
    } */

    pub fn write_creatures(&mut self, creatures: &Vec<creature::Creature>, player_index: usize)
    {
        let back_buffer = &mut self.buffers[self.back_buffer_index];
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

            let ( display_pos_x, display_pos_y ) = (18 + dist.x, 18 + dist.y);
            *back_buffer.value_mut(display_pos_y as usize, display_pos_x as usize) = ch;
        }
    }

    pub fn write_map(&mut self, view_x: i32, view_y: i32, map: &tilemap::Tilemap)
    {
        let ( map_width, map_height ) = map.bounds();
        let back_buffer = &mut self.buffers[self.back_buffer_index];
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
            let display_pos_y = (18 + view_addend_y) as usize;
            let map_index_y = map_pos_y as usize;
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
                let display_pos_x = (18 + view_addend_x) as usize;
                let tile_type;
                if check
                {
                    let map_index_x = map_pos_x as usize;
                    tile_type = map.tile(map_index_x, map_index_y);
                }
                else
                {
                    tile_type = 0_u32;
                }
                let ch = MAP_GRAPHICS[tile_type as usize];
                // let ch = match *is_wall { true => '#', _ => '.', };
                *back_buffer.value_mut(display_pos_y, display_pos_x) = ch;
            }
        }    
    }

    fn front_buffer_index(&self) -> usize
    {
        1 - self.back_buffer_index
    }

    /* fn move_cursor(x: i32, y: i32)
    {
        ncurses::mv(y, x);
    } */

    /* fn write_char(ch: char)
    {
        ncurses::addch(ch as u64);
    } */

    fn put_char(&self, x: i32, y: i32, ch: char)
    {
        ncurses::mvaddch(y, x, ch as u64);
    }
}
