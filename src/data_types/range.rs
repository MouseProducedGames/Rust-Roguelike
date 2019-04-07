/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
extern crate derive_more;

// Standard includes.
use std::convert::From;

// Internal includes.

#[derive(
    Add,
    AddAssign,
    Copy,
    Clone,
    Default,
    Div,
    DivAssign,
    Eq,
    Mul,
    MulAssign,
    PartialEq,
    Sub,
    SubAssign,
)]
pub struct Range(i32);

impl Range {
    pub fn new(value: i32) -> Self {
        Self(value)
    }
}

impl From<Range> for i32 {
    fn from(range: Range) -> i32 {
        range.0
    }
}
