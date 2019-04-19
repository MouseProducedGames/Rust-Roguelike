/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.

// Standard includes.
use std::cmp::Ordering;
use std::convert::From;

// Internal includes.

#[derive(
    Add, AddAssign, Copy, Clone, Default, Display, Eq, Ord, PartialOrd, PartialEq, Sub, SubAssign,
)]
pub struct DefenceValue(i32);

impl DefenceValue {
    fn new(value: i32) -> Self {
        Self(value)
    }
}

impl From<i32> for DefenceValue {
    fn from(value: i32) -> Self {
        Self::new(value)
    }
}

impl From<&i32> for DefenceValue {
    fn from(value: &i32) -> Self {
        Self::from(*value)
    }
}

impl From<DefenceValue> for i32 {
    fn from(value: DefenceValue) -> Self {
        value.0
    }
}

impl PartialEq<i32> for DefenceValue {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialOrd<i32> for DefenceValue {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        Some(self.cmp(&DefenceValue::from(other)))
    }
}
