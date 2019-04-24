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
    Add,
    AddAssign,
    Copy,
    Clone,
    Default,
    Display,
    Eq,
    Hash,
    Ord,
    PartialOrd,
    PartialEq,
    Sub,
    SubAssign,
)]
pub struct ProtectionValue(i32);

impl ProtectionValue {
    fn new(value: i32) -> Self {
        Self(value)
    }
}

impl From<i32> for ProtectionValue {
    fn from(value: i32) -> Self {
        Self::new(value)
    }
}

impl From<&i32> for ProtectionValue {
    fn from(value: &i32) -> Self {
        Self::from(*value)
    }
}

impl From<ProtectionValue> for i32 {
    fn from(value: ProtectionValue) -> Self {
        value.0
    }
}

impl PartialEq<i32> for ProtectionValue {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialOrd<i32> for ProtectionValue {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        Some(self.cmp(&ProtectionValue::from(other)))
    }
}
