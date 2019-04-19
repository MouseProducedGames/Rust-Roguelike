/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.

// Standard includes.
use std::cmp::Ordering;
use std::convert::From;

// Internal includes.
use super::DamageValue;

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
pub struct InjuryValue(i32);

impl InjuryValue {
    fn new(value: i32) -> Self {
        Self(value)
    }
}

impl From<DamageValue> for InjuryValue {
    fn from(value: DamageValue) -> Self {
        Self::new(i32::from(value))
    }
}

impl From<i32> for InjuryValue {
    fn from(value: i32) -> Self {
        Self::new(value)
    }
}

impl From<&i32> for InjuryValue {
    fn from(value: &i32) -> Self {
        Self::from(*value)
    }
}

impl From<InjuryValue> for i32 {
    fn from(value: InjuryValue) -> Self {
        value.0
    }
}

impl PartialEq<i32> for InjuryValue {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialOrd<i32> for InjuryValue {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        Some(self.cmp(&InjuryValue::from(other)))
    }
}
