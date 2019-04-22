/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.

// Standard includes.
use std::convert::From;
use std::fmt;

// internal includes.
use super::{LongDescription, ShortDescription};

#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum DisplayOption {
    Console,
    R8G8B8Console,
}

impl fmt::Display for DisplayOption {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                DisplayOption::Console => "Console",
                DisplayOption::R8G8B8Console => "r8g8b8 Console",
            }
        )
    }
}

impl From<&DisplayOption> for LongDescription {
    fn from(item: &DisplayOption) -> LongDescription {
        LongDescription::from(*item)
    }
}

impl From<DisplayOption> for LongDescription {
    fn from(item: DisplayOption) -> LongDescription {
        match item {
            DisplayOption::Console => {
                LongDescription(concat!("Should run on any tty-compatable device", "."))
            }
            DisplayOption::R8G8B8Console => LongDescription(concat!(
                "A console supporting 8-bit red, green, and blue",
                ".",
                " Should run on most modern operating systems",
                ".",
            )),
        }
    }
}

impl From<&DisplayOption> for ShortDescription {
    fn from(item: &DisplayOption) -> ShortDescription {
        ShortDescription::from(*item)
    }
}

impl From<DisplayOption> for ShortDescription {
    fn from(item: DisplayOption) -> ShortDescription {
        match item {
            DisplayOption::Console => ShortDescription("A 15-colour console display."),
            DisplayOption::R8G8B8Console => ShortDescription("A standard 24-bit colour console."),
        }
    }
}
