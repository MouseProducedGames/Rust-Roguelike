/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

 **/
// External includes.
use crossterm::{terminal, Terminal};
use crossterm_style::{Color, Colored};

// Standard includes.
use std::convert::Into;
use std::fmt;
use std::iter::Iterator;

// Internal includes.
use super::ConsoleChar;
use crate::data_types::Immut;
use crate::game::GameValueFixed;
use crate::io::{Display, LongDescription, ShortDescription};
use crate::multidim::Multidim;
use crate::rrl_math::Bounds;
use crate::stats::{Attribute, Stat, StatModifier};

pub struct ConsoleDisplay {
    term: crossterm::Crossterm,
    term_window: Terminal<'static>,
    buffers: [Multidim<ConsoleChar>; 2],
    back_buffer_index: usize,
    // map_graphics: [ConsoleChar; 8],
    item_graphics: Vec<ConsoleChar>,
    map_graphics: Vec<ConsoleChar>,
    // Ensure we get an unchanged value.
    window_resized: Immut<bool>,
    start_size: Immut<Bounds>,
}

impl ConsoleDisplay {
    pub fn new() -> Self {
        let term = crossterm::Crossterm::new();

        let term_window = terminal();
        let (width, height) = term_window.terminal_size();
        // The values cannot change before we store them.
        let start_size = Immut::new(Bounds::new(width, height));
        let window_resized: bool;
        {
            // The values do not permanently change.
            let (mut width, mut height) = (width, height);
            let window_too_small = if height < 40 {
                height = 40;
                true
            } else if width < 80 {
                width = 80;
                true
            } else {
                false
            };

            window_resized = if window_too_small {
                match term_window.set_size(width as i16, height as i16) {
                    Ok(_) => true,
                    _ => panic!("Could not set the size of the window."),
                }
            } else {
                false
            }
        }

        // Not panic-worthy if it doesn't work...
        if let Ok(_v) = term.cursor().hide() {}
        let mut output = Self {
            term,
            term_window,
            buffers: [Multidim::new(40, 80), Multidim::new(40, 80)],
            back_buffer_index: 0,
            item_graphics: vec![
                // Torch.
                ConsoleChar::new(
                    '/',
                    Color::Rgb {
                        r: 193,
                        g: 69,
                        b: 47,
                    },
                    Color::Rgb { r: 0, g: 0, b: 0 },
                ),
                // Weapon.
                ConsoleChar::new(
                    '/',
                    Color::Rgb {
                        r: 192,
                        g: 195,
                        b: 198,
                    },
                    Color::Rgb { r: 0, g: 0, b: 0 },
                ),
                // Armour.
                ConsoleChar::new(
                    '&',
                    Color::Rgb {
                        r: 192,
                        g: 195,
                        b: 198,
                    },
                    Color::Rgb { r: 0, g: 0, b: 0 },
                ),
            ],
            map_graphics: vec![
                // Void.
                ConsoleChar::new(
                    ' ',
                    Color::Rgb { r: 0, g: 0, b: 0 },
                    Color::Rgb { r: 0, g: 0, b: 0 },
                ),
                // Wall.
                ConsoleChar::new(
                    '#',
                    Color::Rgb {
                        r: 255,
                        g: 255,
                        b: 255,
                    },
                    Color::Rgb { r: 0, g: 0, b: 0 },
                ),
                // Floor.
                ConsoleChar::new(
                    '.',
                    Color::Rgb {
                        r: 255,
                        g: 255,
                        b: 255,
                    },
                    Color::Rgb { r: 0, g: 0, b: 0 },
                ),
                // Door.
                ConsoleChar::new(
                    '+',
                    Color::Rgb {
                        r: 255,
                        g: 255,
                        b: 255,
                    },
                    Color::Rgb {
                        r: 129,
                        g: 69,
                        b: 47,
                    },
                ),
                // Open door.
                ConsoleChar::new(
                    '/',
                    Color::Rgb {
                        r: 255,
                        g: 255,
                        b: 255,
                    },
                    Color::Rgb {
                        r: 129,
                        g: 69,
                        b: 47,
                    },
                ),
                // Secret door.
                ConsoleChar::new(
                    '#',
                    Color::Rgb {
                        r: 255,
                        g: 255,
                        b: 255,
                    },
                    Color::Rgb { r: 0, g: 0, b: 0 },
                ),
                // Discovered secret door.
                ConsoleChar::new(
                    '+',
                    Color::Rgb {
                        r: 255,
                        g: 255,
                        b: 255,
                    },
                    Color::Rgb {
                        r: 128,
                        g: 128,
                        b: 128,
                    },
                ),
                // Open secret door.
                ConsoleChar::new(
                    '/',
                    Color::Rgb {
                        r: 255,
                        g: 255,
                        b: 255,
                    },
                    Color::Rgb {
                        r: 128,
                        g: 128,
                        b: 128,
                    },
                ),
            ],
            window_resized: Immut::new(window_resized),
            start_size,
        };

        output.clear();

        output
    }

    pub(crate) fn clear(&mut self) {
        self.clear_back_buffer();
        self.clear_front_buffer();

        self.clear_terminal();
    }

    fn clear_buffer(&mut self, buffer_index: usize) {
        let (buffer_height, buffer_width) = self.buffers[self.back_buffer_index].bounds();
        let buffer = &mut self.buffers[buffer_index];
        for y in 0..buffer_height {
            for x in 0..buffer_width {
                *buffer.value_mut(y, x) = self.map_graphics[0];
            }
        }
    }

    pub(crate) fn clear_back_buffer(&mut self) {
        self.clear_buffer(self.back_buffer_index);
    }

    pub(crate) fn clear_front_buffer(&mut self) {
        self.clear_buffer(self.front_buffer_index());
    }

    pub(crate) fn clear_terminal(&mut self) {
        match self.term.terminal().clear(crossterm::ClearType::All) {
            Ok(_v) => (),
            _ => panic!("Could not clear screen."),
        }
    }

    pub(crate) fn choose<TOption: 'static>(
        &mut self,
        header: &'static str,
        options: &'static [TOption],
    ) -> TOption
    where
        TOption: fmt::Display + Copy + Into<LongDescription> + Into<ShortDescription>,
        LongDescription: std::convert::From<TOption>,
        ShortDescription: std::convert::From<&'static TOption>,
    {
        let mut index: usize = 0;

        self.clear();
        loop {
            self.clear_back_buffer();
            self.put_string(1, 1, header, Color::Grey, Color::Black);

            for (i, species_type) in options.iter().enumerate() {
                let formatted = format!(
                    "   {:<10} {}",
                    species_type.to_string(),
                    *ShortDescription::from(species_type)
                );
                self.put_string(1, 3_i32 + i as i32, &formatted, Color::Grey, Color::Black);
            }

            self.put_string(1, 3_i32 + index as i32, "->", Color::Yellow, Color::Black);

            self.put_string(
                4,
                20,
                *LongDescription::from(options[index]),
                Color::Grey,
                Color::Black,
            );

            self.present();

            match self.get_char() {
                // Return.
                '\r' => {
                    self.clear();
                    return options[index];
                }
                '2' => {
                    index = (index + 1) % options.len();
                }
                '8' => {
                    index = index.wrapping_sub(1);
                    if index > options.len() {
                        index = options.len() - 1;
                    }
                }
                _ => (),
            }
        }
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

    pub(crate) fn get_draw_info(&mut self) -> (&Vec<ConsoleChar>, &mut Multidim<ConsoleChar>) {
        (
            &self.map_graphics,
            &mut self.buffers[self.back_buffer_index],
        )
    }

    pub(crate) fn get_item_graphics(&self, index: u32) -> ConsoleChar {
        self.item_graphics[index as usize]
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
                for x in 0..buffer_width {
                    xi += 1;
                    let back_ch = *back_buffer.value(y, x);
                    let front_ch = *front_buffer.value(y, x);
                    if ConsoleChar::any_equality(&front_ch, &back_ch) {
                        continue;
                    }

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

    pub(crate) fn put_console_char(&mut self, x: i32, y: i32, ch: ConsoleChar) {
        *self.buffers[self.back_buffer_index].value_mut(y as usize, x as usize) = ch;
    }

    pub(crate) fn put_health(
        &mut self,
        x: i32,
        y: i32,
        name: &str,
        max: GameValueFixed,
        stat: Attribute,
    ) {
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

    fn front_buffer_index(&self) -> usize {
        1 - self.back_buffer_index
    }

    fn present_console_char(&self, x: i32, y: i32, ch: ConsoleChar) {
        self.move_cursor(x, y);
        self.write_console_char(ch);
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
}

impl Drop for ConsoleDisplay {
    fn drop(&mut self) {
        println!("{}{}", Colored::Fg(Color::Grey), Colored::Bg(Color::Black));

        match self.term.cursor().show() {
            Ok(_v) => (),
            // We shouldn't panic; but we should inform the user.
            _ => println!("Could not restore cursor visibility!"),
        }
        // If it doesn't work here, we should not panic.
        // Fear is the shutdown-killer that brings total confusion.
        if let Ok(_v) = self.term.terminal().clear(crossterm::ClearType::All) {}

        if *self.window_resized.read() {
            let start_size = *self.start_size.read();
            let (start_width, start_height) = (start_size.width as i16, start_size.height as i16);
            match self.term_window.set_size(start_width, start_height) {
                Ok(_) => (),
                _ => println!("Could not resize window."),
            }
        }
    }
}
