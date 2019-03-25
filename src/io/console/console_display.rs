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
use crossterm_style::{Color, Colored};
use std::iter::Iterator;

// Internal includes.
use super::super::super::multidim::Multidim;
use super::{ConsoleChar, Darker};
use crate::rrl_math::{Displacement, Position};
use crate::stats::{Attribute, Stat, StatModifier};
use crate::world::{Tilemap, VisibilityMap, VisibilityType};

pub struct ConsoleDisplay {
    term: crossterm::Crossterm,
    buffers: [Multidim<ConsoleChar>; 2],
    back_buffer_index: usize,
    map_graphics: [ConsoleChar; 8],
}

impl ConsoleDisplay {
    pub fn new() -> Self {
        let term = crossterm::Crossterm::new();

        // Not panic-worthy if it doesn't work...
        if let Ok(_v) = term.cursor().hide() {}
        let mut output = Self {
            term,
            buffers: [Multidim::new(40, 80), Multidim::new(40, 80)],
            back_buffer_index: 0,
            map_graphics: [
                // Void.
                ConsoleChar::new(' ', Color::Black, Color::Black),
                // Wall.
                ConsoleChar::new('#', Color::Grey, Color::Black),
                // Floor.
                ConsoleChar::new('.', Color::Grey, Color::Black),
                // Door.
                ConsoleChar::new('+', Color::Grey, Color::Red),
                // Open door.
                ConsoleChar::new('/', Color::Grey, Color::Red),
                // Secret door.
                ConsoleChar::new('#', Color::Grey, Color::Black),
                // Discovered secret door.
                ConsoleChar::new('+', Color::Grey, Color::White),
                // Open secret door.
                ConsoleChar::new('/', Color::Grey, Color::White),
            ],
        };

        output.clear();

        /* let front_buffer_index = output.front_buffer_index();
        let buffers = &mut output.buffers;
        let (buffer_height, buffer_width) = buffers[output.back_buffer_index].bounds();
        for y in 0..buffer_height {
            for x in 0..buffer_width {
                *buffers[front_buffer_index].value_mut(y, x) = output.map_graphics[0];
                *buffers[output.back_buffer_index].value_mut(y, x) = output.map_graphics[0];
            }
        } */

        output
    }

    pub(crate) fn clear(&mut self) {
        let front_buffer_index = self.front_buffer_index();
        let buffers = &mut self.buffers;
        let (buffer_height, buffer_width) = buffers[self.back_buffer_index].bounds();
        for y in 0..buffer_height {
            for x in 0..buffer_width {
                *self.buffers[front_buffer_index].value_mut(y, x) = self.map_graphics[0];
                *self.buffers[self.back_buffer_index].value_mut(y, x) = self.map_graphics[0];
            }
        }

        match self.term.terminal().clear(crossterm::ClearType::All) {
            Ok(_v) => (),
            _ => panic!("Could not clear screen."),
        }
    }

    fn front_buffer_index(&self) -> usize {
        1 - self.back_buffer_index
    }

    pub(crate) fn get_char_impl(&self) -> char {
        self.move_cursor(0, 0);
        let ch;
        match self.term.input().read_char() {
            Ok(v) => ch = v,
            _ => ch = ' ',
        }
        self.move_cursor(0, 0);
        println!(" ");
        self.move_cursor(0, 0);
        ch
    }

    pub(crate) fn move_cursor(&self, x: i32, y: i32) {
        match self.term.cursor().goto(x as u16, y as u16) {
            Ok(_v) => (),
            _ => panic!("Could not move cursor to ( {}, {} )", x, y),
        }
    }

    pub(crate) fn present_impl(&mut self) {
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
                let mut lastch = self.map_graphics[0];
                for x in 0..buffer_width {
                    xi += 1;
                    let back_ch = *back_buffer.value(y, x);
                    let front_ch = *front_buffer.value(y, x);
                    if ConsoleChar::any_equality(&front_ch, &back_ch) {
                        // lastch = front_ch;
                        repeat_count += 1;
                        if repeat_count >= 5 {
                            // ncurses::refresh();
                            repeat_count = 0;
                        }
                        continue;
                    }

                    if ConsoleChar::any_equality(&lastch, &front_ch) {
                        repeat_count += 1;
                    } else {
                        repeat_count = 1;
                    }

                    if repeat_count >= 5 {
                        // ncurses::refresh();
                        repeat_count = 0;
                    }

                    lastch = front_ch;

                    self.present_console_char(xi, yi, front_ch);
                    // self.put_console_char(xi, yi, front_ch);
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

    fn write_console_char(&self, ch: ConsoleChar) {
        println!(
            "{}{}{}",
            Colored::Fg(ch.foreground()),
            Colored::Bg(ch.background()),
            ch.graphic()
        );
    }

    fn _write_string(&self, s: String) {
        println!("{}", s);
    }

    fn present_console_char(&self, x: i32, y: i32, ch: ConsoleChar) {
        self.move_cursor(x, y);
        self.write_console_char(ch);
    }

    pub(crate) fn put_console_char(&mut self, x: i32, y: i32, ch: ConsoleChar) {
        *self.buffers[self.back_buffer_index].value_mut(y as usize, x as usize) = ch;
    }

    pub(crate) fn put_health(&mut self, x: i32, y: i32, name: &str, max: i32, stat: Attribute) {
        let formatted = format!("{:.>13} {:>2}/{:+>2}", name, stat.value(), max);
        self.put_string(x, y, &formatted, Color::Grey, Color::Black);
    }

    pub(crate) fn put_stat(&mut self, x: i32, y: i32, name: &str, stat: Attribute) {
        let formatted = format!(
            "{:.>13} {:>2} : {:+>2}",
            name,
            stat.value(),
            stat.modifier()
        );
        self.put_string(x, y, &formatted, Color::Grey, Color::Black);
    }

    pub(crate) fn put_string(&mut self, x: i32, y: i32, s: &str, fg: Color, bg: Color) {
        for (i, ch) in s.chars().enumerate() {
            self.put_console_char(x + i as i32, y, ConsoleChar::new(ch, fg, bg));
        }
    }

    pub(crate) fn write_map_impl(
        &mut self,
        view_pos: Position,
        map: &Tilemap,
        vis: &VisibilityMap,
    ) {
        let back_buffer = &mut self.buffers[self.back_buffer_index];
        for view_addend_y in -17..18_i32 {
            let display_pos_y = (18 + view_addend_y) as usize;
            for view_addend_x in -17..18_i32 {
                let display_pos_x = (18 + view_addend_x) as usize;
                let map_pos = view_pos + Displacement::new(view_addend_x, view_addend_y);
                let ch;
                match vis.value_pos(map_pos) {
                    VisibilityType::None => ch = self.map_graphics[0],
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
