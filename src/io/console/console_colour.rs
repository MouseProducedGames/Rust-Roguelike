/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

 **/
// External includes.
extern crate crossterm;
extern crate crossterm_style;
use crossterm_style::Color;

// Internal includes.
use super::Darker;

// pub enum ConsoleColour { Color }

impl Darker for Color
{
    fn darker( self ) -> Self
    {
        match self {
            Color::Black => Color::Black,

            Color::Red => Color::DarkRed,
            Color::DarkRed => Color::Black,

            Color::Green => Color::Green,
            Color::DarkGreen => Color::Black,

            Color::Yellow => Color::DarkYellow,
            Color::DarkYellow => Color::Black,

            Color::Blue => Color::DarkBlue,
            Color::DarkBlue => Color::Black,

            Color::Magenta => Color::DarkMagenta,
            Color::DarkMagenta => Color::Black,

            Color::Cyan => Color::DarkCyan,
            Color::DarkCyan => Color::Black,

            // These values are backwards.
            Color::White => Color::Black,
            Color::Grey => Color::White,

            Color::Rgb { r, g, b } => Color::Rgb { r: r / 2, g: g / 2, b: b / 2 },

            Color::AnsiValue( v ) => Color::AnsiValue( v ),
        }
    }
}

/* impl Eq for ConsoleColour {}

impl PartialEq for ConsoleColour
{
    fn eq( &self, other: &ConsoleColour ) -> bool
    {
        self == other
    }
} */
