/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.

// Standard includes.
use std::cmp::Ordering;
use std::fmt;

// Internal includes.

#[derive(Add, AddAssign, Copy, Clone, Default, Eq, PartialEq, Sub, SubAssign)]
pub struct Time(u64);

impl Time {
    pub fn new(value: u64) -> Self {
        Self(value)
    }
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} turns", self.0)
    }
}

impl Ord for Time {
    fn cmp(&self, other: &Time) -> Ordering {
        other.0.cmp(&self.0)
    }
}

impl PartialOrd for Time {
    fn partial_cmp(&self, other: &Time) -> Option<Ordering> {
        other.0.partial_cmp(&self.0)
    }
}
