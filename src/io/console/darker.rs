/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

 **/
// External includes.
extern crate crossterm;
extern crate crossterm_style;

// Internal includes.

pub trait Darker {
    fn darker(self) -> Self;
}
