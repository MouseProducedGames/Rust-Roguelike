/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

 **/
// External includes.
extern crate crossterm;
extern crate crossterm_cursor;
extern crate crossterm_input;
extern crate crossterm_screen;
extern crate crossterm_style;
use crossterm_style::{ Color, Colored };

// Internal includes.
use super::super::super::multidim::Multidim;
use crate::creature::CreatureStats;
use crate::creature::background::SpeciesType;
use crate::io::Display;
use crate::faction::Faction;
use crate::rrl_math::{Displacement, Position};
use crate::stats::{ Stat, StatModifier };
use crate::world::{Tilemap, VisibilityMap, VisibilityType};
use super::{ ConsoleChar, Darker };

pub struct ConsoleDisplay {
    term: crossterm::Crossterm,
    buffers: [Multidim<ConsoleChar>; 2],
    back_buffer_index: usize,
    map_graphics: [ConsoleChar; 5],
}

impl ConsoleDisplay {
    pub fn new() -> Self {
        let term = crossterm::Crossterm::new();
        match term.terminal().clear(crossterm::ClearType::All) {
            Ok(_v) => (),
            _ => panic!("Could not clear screen."),
        }
        // Not panic-worthy if it doesn't work...
        if let Ok(_v) = term.cursor().hide() {}
        let mut output = Self {
            term,
            buffers: [Multidim::new(40, 80), Multidim::new(40, 80)],
            back_buffer_index: 0,
            map_graphics: [
                ConsoleChar::new( ' ', Color::Black, Color::Black ),
                ConsoleChar::new( '#', Color::Grey, Color::Black ),
                ConsoleChar::new( '.', Color::Grey, Color::Black ),
                ConsoleChar::new( '+', Color::Grey, Color::Red ),
                ConsoleChar::new( '/', Color::Grey, Color::Red ),
            ],
        };
        let front_buffer_index = output.front_buffer_index();
        let buffers = &mut output.buffers;
        let (buffer_height, buffer_width) = buffers[output.back_buffer_index].bounds();
        for y in 0..buffer_height {
            for x in 0..buffer_width {
                *buffers[front_buffer_index].value_mut(y, x) = output.map_graphics[ 0 ];
                *buffers[output.back_buffer_index].value_mut(y, x) = output.map_graphics[ 0 ];
            }
        }

        output
    }

    fn front_buffer_index(&self) -> usize {
        1 - self.back_buffer_index
    }

    fn move_cursor(&self, x: i32, y: i32) {
        match self.term.cursor().goto(x as u16, y as u16) {
            Ok(_v) => (),
            _ => panic!("Could not move cursor to ( {}, {} )", x, y),
        }
    }

    fn write_console_char( &self, ch: ConsoleChar ) {
        println!(
            "{}{}{}",
            Colored::Fg( ch.foreground() ),
            Colored::Bg( ch.background() ),
            ch.graphic()
        );
    }

    fn _write_string( &self, s: String )
    {
        println!( "{}", s );
    }

    fn put_console_char( &self, x: i32, y: i32, ch: ConsoleChar ) {
        self.move_cursor(x, y);
        self.write_console_char(ch);
    }
}

impl Display for ConsoleDisplay
{
    fn choose_species( &self, options: &Vec<SpeciesType> ) -> SpeciesType
    {
        println!("{}{}", Colored::Fg( Color::Grey ), Colored::Bg( Color::Black ));
        
        let mut keep_going = true;
        let mut option = SpeciesType::Human;

        while keep_going
        {
            match self.term.terminal().clear(crossterm::ClearType::All) {
                Ok(_v) => (),
                _ => panic!("Could not clear screen."),
            }

            for ( i, species_type ) in options.iter().enumerate()
            {
                println!( "{}) {}", ( 1 + i ), species_type.to_string() );
            }

            println!();
            println!( " Select your character's species: " );

            match self.get_char() {
                '1' => { option = options[0]; keep_going = false; },
                '2' => { option = options[1]; keep_going = false; },
                '3' => { option = options[2]; keep_going = false; },
                '4' => { option = options[3]; keep_going = false; },
                _ => (),
            }
        }

        match self.term.terminal().clear(crossterm::ClearType::All) {
            Ok(_v) => (),
            _ => panic!("Could not clear screen."),
        }
        
        option
    }

    fn display_stats( &mut self, stats: CreatureStats )
    {
        println!("{}{}", Colored::Fg( Color::Grey ), Colored::Bg( Color::Black ));
        
        self.move_cursor( 42, 2 );
        println!("Strength.....: {:>2} : {:+>2}", stats.strength().value(), stats.strength().modifier() );
        self.move_cursor( 42, 3 );
        println!("Agility......: {:>2} : {:+>2}", stats.agility().value(), stats.agility().modifier() );
        self.move_cursor( 42, 4 );
        println!("Coordination.: {:>2} : {:+>2}", stats.coordination().value(), stats.strength().modifier() );
        self.move_cursor( 42, 5 );
        println!("Endurance....: {:>2} : {:+>2}", stats.endurance().value(), stats.endurance().modifier() );
        self.move_cursor( 42, 7 );
        println!("Health.......: {:>2}/{:>2}", stats.health().value(), stats.endurance().value() );
    }

    fn get_char(&self) -> char {
        self.move_cursor(0, 0);
        let ch;
        match self.term.input().read_char() {
            Ok(v) => ch = v,
            _ => ch = ' ',
        }
        self.move_cursor(0, 0);
        println!(" ");
        ch
    }

    fn present(&mut self)
    {
        {
            self.back_buffer_index = 1 - self.back_buffer_index;
            let buffers = &self.buffers;
            let back_buffer = &buffers[self.back_buffer_index];
            let front_buffer = &buffers[self.front_buffer_index()];
            let (buffer_height, buffer_width) = front_buffer.bounds();
            let mut yi = -1_i32;
            for y in 0..buffer_height {
                yi += 1;
                let mut xi = -1_i32;
                // Without the repeat_count check, repeated characters (5 or more)
                // are placed in the same spot.
                let mut repeat_count = 1;
                let mut lastch = self.map_graphics[ 0 ];
                for x in 0..buffer_width {
                    xi += 1;
                    let back_ch = *back_buffer.value(y, x);
                    let front_ch = *front_buffer.value(y, x);
                    if ConsoleChar::any_equality( &front_ch, &back_ch ) {
                        // lastch = front_ch;
                        repeat_count += 1;
                        if repeat_count >= 5 {
                            // ncurses::refresh();
                            repeat_count = 0;
                        }
                        continue;
                    }

                    if ConsoleChar::any_equality( &lastch, &front_ch ) {
                        repeat_count += 1;
                    } else {
                        repeat_count = 1;
                    }

                    if repeat_count >= 5 {
                        // ncurses::refresh();
                        repeat_count = 0;
                    }

                    lastch = front_ch;

                    self.put_console_char(xi, yi, front_ch);
                }
            }
        }
        // ncurses::refresh();

        {
            let front_buffer_index = self.front_buffer_index();
            let buffers = &mut self.buffers;
            let (buffer_height, buffer_width) = buffers[self.back_buffer_index].bounds();
            for y in 0..buffer_height {
                for x in 0..buffer_width {
                    let val = *buffers[front_buffer_index].value(y, x);
                    *buffers[self.back_buffer_index].value_mut(y, x) = val;
                }
            }
        }
    }

    fn write_creature(&mut self, faction: Faction, creature_pos: Position, view_pos: Position)
    {
        let disp = creature_pos - view_pos;
        if (disp.x < -17) || (disp.x > 17) || (disp.y < -17) || (disp.y > 17) {
            return;
        }
        let (display_pos_x, display_pos_y) = (18 + disp.x, 18 + disp.y);
        let ch;
        if faction == Faction::new(0)
        {
            ch = ConsoleChar::new( '@', Color::Grey, Color::Black );
        }
        else
        {
            ch = ConsoleChar::new( 'C', Color::Grey, Color::Black );
        }
        *self.buffers[self.back_buffer_index]
            .value_mut(display_pos_y as usize, display_pos_x as usize) = ch;
        // self.put_char( 18 + disp.x, 18 + disp.y, 'C' );
    }

    fn write_map(&mut self, view_pos: Position, map: &Tilemap, vis: &VisibilityMap)
    {
        let back_buffer = &mut self.buffers[self.back_buffer_index];
        for view_addend_y in -17..18_i32 {
            let display_pos_y = (18 + view_addend_y) as usize;
            for view_addend_x in -17..18_i32 {
                let display_pos_x = (18 + view_addend_x) as usize;
                let map_pos = view_pos + Displacement::new(view_addend_x, view_addend_y);
                let ch;
                match vis.value_pos(map_pos) {
                    VisibilityType::None => ch = self.map_graphics[ 0 ],
                    VisibilityType::Seen => {
                        let tile_type = map.tile_type_pos(map_pos);
                        ch = self.map_graphics[tile_type as usize].darker();
                    }
                    VisibilityType::Visible => {
                        let tile_type = map.tile_type_pos(map_pos);
                        ch = self.map_graphics[tile_type as usize];
                    }
                }

                *back_buffer.value_mut(display_pos_y, display_pos_x) = ch;
            }
        }
    }
}

impl Drop for ConsoleDisplay {
    fn drop(&mut self) {
        match self.term.cursor().show() {
            Ok(_v) => (),
            // We shouldn't panic; but we should inform the user.
            _ => println!("Could not restore cursor visibility!"),
        }
        // If it doesn't work here, we should not panic.
        // Fear is the shutdown-killer that brings total confusion.
        if let Ok(_v) = self.term.terminal().clear(crossterm::ClearType::All) {}
    }
}
