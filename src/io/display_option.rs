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
}

impl fmt::Display for DisplayOption {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                DisplayOption::Console => "Console",
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
        }
    }
}
