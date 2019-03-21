/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

 **/
// External includes.
extern crate crossterm;
extern crate crossterm_style;
use crossterm_style::Color;
use std::default::Default;

// Internal includes.
use super::Darker;

#[derive(Copy, Clone)]
pub struct ConsoleChar
{
    graphic: char,
    foreground: Color,
    background: Color,
}

impl ConsoleChar
{
    pub fn new( graphic: char, foreground: Color, background: Color ) -> Self
    {
        Self { graphic, foreground, background }
    }

    pub fn graphic( &self ) -> char
    {
        self.graphic
    }

    pub fn foreground( &self ) -> Color
    {
        self.foreground
    }

    pub fn background( &self ) -> Color
    {
        self.background
    }

    pub fn any_equality( &self, other: &ConsoleChar ) -> bool
    {
        self.graphic == other.graphic &&
            Self::color_equality( &self.foreground, &other.foreground ) &&
            Self::color_equality( &self.background, &other.background )
    }

    fn color_equality( a: &Color, b: &Color ) -> bool
    {
        let a_val = Self::any_equality_helper( a );
        let b_val = Self::any_equality_helper( b );
        if a_val == 65535 && b_val == 65535
        {
            let ( r0, g0, b0 );
            let ( r1, g1, b1 );
            match a {
                Color::Rgb { r, g, b } => { r0 = r; g0 = g; b0 = b; },
                _ => panic!( "This must be an Rgb but it's not an Rgb but it must be an Rgb but..." ),
            }

            match b {
                Color::Rgb { r, g, b } => { r1 = r; g1 = g; b1 = b; },
                _ => panic!( "This must be an Rgb but it's not an Rgb but it must be an Rgb but..." ),
            }

            ( r0 == r1 ) && ( g0 == g1 ) && ( b0 == b1 )
        }
        else
        {
            a_val == b_val
        }
    }

    fn any_equality_helper( a: &Color ) -> u16
    {
        match a {
            Color::Black => 0,

            Color::Red => 1,
            Color::DarkRed => 2,

            Color::Green => 3,
            Color::DarkGreen => 4,

            Color::Yellow => 5,
            Color::DarkYellow => 6,

            Color::Blue => 7,
            Color::DarkBlue => 8,

            Color::Magenta => 9,
            Color::DarkMagenta => 10,

            Color::Cyan => 11,
            Color::DarkCyan => 12,

            Color::White => 13,
            Color::Grey => 14,

            // Since other values are u8, this isn't used.
            Color::Rgb { r: _, g: _, b: _ } => 65535,
            
            Color::AnsiValue( v ) => *v as u16,
        }
    }
}

impl Darker for ConsoleChar
{
    fn darker( self ) -> Self
    {
        Self::new( self.graphic, self.foreground.darker(), self.background.darker() )
    }
}

impl Default for ConsoleChar
{
    fn default() -> Self
    {
        Self::new( ' ', Color::White, Color::Black )
    }
}

/* impl Eq for ConsoleChar {}

impl PartialEq for ConsoleChar
{
    fn eq( &self, other: &ConsoleChar ) -> bool
    {
        let argh = self.foreground;
        let bleh = other.foreground;
        let argh2 = self.background;
        let bleh2 = other.background;
        let arglblah: ConsoleColour;
        arglblah
        return            
            ( self.graphic == other.graphic ) &&
            ( argh as ConsoleColour::Color == bleh as ConsoleColour ) &&
            matches!( argh2, bleh2 )
    }
} */

/*  */
