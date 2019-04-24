/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.

// Standard includes.
use std::cmp::Ordering;
use std::convert::From;
use std::ops::{Sub, SubAssign};

// Internal includes.
use super::ProtectionValue;

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
pub struct DamageValue(i32);

impl DamageValue {
    fn new(value: i32) -> Self {
        Self(value)
    }
}

impl From<i32> for DamageValue {
    fn from(value: i32) -> Self {
        Self::new(value)
    }
}

impl From<&i32> for DamageValue {
    fn from(value: &i32) -> Self {
        Self::from(*value)
    }
}

impl From<DamageValue> for i32 {
    fn from(value: DamageValue) -> Self {
        value.0
    }
}

impl PartialEq<i32> for DamageValue {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialOrd<i32> for DamageValue {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        Some(self.cmp(&DamageValue::from(other)))
    }
}

impl Sub<ProtectionValue> for DamageValue {
    type Output = DamageValue;

    fn sub(self, other: ProtectionValue) -> DamageValue {
        DamageValue::from(self.0 - i32::from(other))
    }
}

impl SubAssign<ProtectionValue> for DamageValue {
    fn sub_assign(&mut self, other: ProtectionValue) {
        self.0 -= i32::from(other);
    }
}
