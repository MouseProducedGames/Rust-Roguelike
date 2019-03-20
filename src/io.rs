/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

 **/

// External includes.
extern crate crossterm;
extern crate crossterm_cursor;
extern crate crossterm_input;
extern crate crossterm_screen;
// extern crate ncurses;
// use std::iter::Iterator;

// Internal includes.
use crate::rrl_math::{ Displacement, Position };
use crate::world::{ Tilemap, VisibilityMap, VisibilityType };
use super::multidim::Multidim;

static MAP_GRAPHICS: [ char; 4 ] = [ ' ', '#', '.', '+' ];
static SEEN_MAP_GRAPHICS: [ char; 4 ] = [ ' ', 'x', '-', '=' ];

pub struct Window
{
    term: crossterm::Crossterm,
    buffers: [Multidim<char>; 2],
    back_buffer_index: usize,
}

impl Window
{
    /* pub fn init()
    {
    /* ncurses::initscr();
    ncurses::keypad(ncurses::stdscr(), true);
    // ncurses::raw();
    ncurses::curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    ncurses::nonl();
    ncurses::cbreak();
    ncurses::noecho(); */
}

    pub fn close()
    {
    // ncurses::endwin();
} */

    pub fn new() -> Self
    {
        let term = crossterm::Crossterm::new();
        match term.terminal().clear( crossterm::ClearType::All )
        {
            Ok(_v) => (),
            _ => panic!( "Could not clear screen." )            
        }
        // Not panic-worthy if it doesn't work...
        if let Ok(_v) = term.cursor().hide() {}
        let mut output =
            Self {
                term,
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

    pub fn get_char( &self ) -> char
    {
        self.move_cursor( 0, 0 );
        let ch;
        match self.term.input().read_char()
        {
            Ok(v) => ch = v,
            _ => ch = ' ',
        }
        self.move_cursor( 0, 0 );
        println!(" ");
        ch
            
        /* if let Some(Ok(key)) = self.term.input().read_async().bytes().next()
        {
            key as char
    }
        else
        {
            ' '
    } */
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
                        repeat_count += 1;
                        if repeat_count >= 5
                        {
                            // ncurses::refresh();
                            repeat_count = 0;
                        }
                        continue;
                    }
                    
                    if lastch == front_ch
                    {
                        repeat_count += 1;
                    } else {
                        repeat_count = 1;
                    }
                    
                    if repeat_count >= 5
                    {
                        // ncurses::refresh();
                        repeat_count = 0;
                    }
                    
                    lastch = front_ch;
                    
                    self.put_char(xi, yi, front_ch);
                }
            }
        }
        // ncurses::refresh();

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

    pub fn write_creature( &mut self, creature_pos: Position, view_pos: Position )
    {
        let disp = creature_pos - view_pos;
        if ( disp.x < -17 ) || ( disp.x > 17 ) ||
            ( disp.y < -17 ) || ( disp.y > 17 )
        {
            return;
        }
        let ( display_pos_x, display_pos_y ) = ( 18 + disp.x, 18 + disp.y );
        *self.buffers[ self.back_buffer_index ].value_mut( display_pos_y as usize, display_pos_x as usize ) = 'C';
        // self.put_char( 18 + disp.x, 18 + disp.y, 'C' );
    }

    pub fn write_map( &mut self, view_pos: Position, map: &Tilemap, vis: &VisibilityMap )
    {
        let back_buffer = &mut self.buffers[ self.back_buffer_index ];
        for view_addend_y in -17..18_i32
        {
            let display_pos_y = ( 18 + view_addend_y ) as usize;
            for view_addend_x in -17..18_i32
            {
                let display_pos_x = ( 18 + view_addend_x ) as usize;
                let map_pos = view_pos + Displacement::new( view_addend_x, view_addend_y );
                let ch;
                match vis.value_pos( map_pos )
                {
                    VisibilityType::None => ch = ' ',
                    VisibilityType::Seen => {
                        let tile_type = map.tile_type_pos( map_pos );
                        ch = SEEN_MAP_GRAPHICS[ tile_type as usize ];
                    },
                    VisibilityType::Visible => { 
                        let tile_type = map.tile_type_pos( map_pos );
                        ch = MAP_GRAPHICS[ tile_type as usize ];
                    },
                }
                
                *back_buffer.value_mut( display_pos_y, display_pos_x ) = ch;
            }
        }    
    }

    fn front_buffer_index(&self) -> usize
    {
        1 - self.back_buffer_index
    }

    fn move_cursor( &self, x: i32, y: i32)
    {
        match self.term.cursor().goto( x as u16, y as u16 ) {
            Ok(_v) => (),
            _ => panic!( "Could not move cursor to ( {}, {} )", x, y )
        }
    }

    fn write_char(&self, ch: char)
    {
        println!( "{}", ch );
    }

    fn put_char(&self, x: i32, y: i32, ch: char)
    {
        self.move_cursor( x, y );
        self.write_char( ch );
        // ncurses::mvaddch(y, x, ch as u64);
    }
}

impl Drop for Window
{
    fn drop( &mut self )
    {
        match self.term.cursor().show() {
            Ok(_v) => (),
            // We shouldn't panic; but we should inform the user.
            _ => println!( "Could not restore cursor visibility!" ),
        }
        // If it doesn't work here, we should not panic.
        // Fear is the shutdown-killer that brings total confusion.
        if let Ok(_v) = self.term.terminal().clear( crossterm::ClearType::All ) {}
    }
}
