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
use super::DefenceValue;

#[derive(
    Add, AddAssign, Copy, Clone, Default, Display, Ord, PartialOrd, Eq, PartialEq, Sub, SubAssign,
)]
pub struct AttackValue(i32);

impl AttackValue {
    fn new(value: i32) -> Self {
        Self(value)
    }
}

impl From<i32> for AttackValue {
    fn from(value: i32) -> Self {
        Self::new(value)
    }
}

impl From<&i32> for AttackValue {
    fn from(value: &i32) -> Self {
        Self::from(*value)
    }
}

impl From<AttackValue> for i32 {
    fn from(value: AttackValue) -> Self {
        value.0
    }
}

impl PartialEq<i32> for AttackValue {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialOrd<i32> for AttackValue {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        Some(self.cmp(&AttackValue::from(other)))
    }
}

impl Sub<DefenceValue> for AttackValue {
    type Output = AttackValue;

    fn sub(self, other: DefenceValue) -> Self {
        AttackValue::from(self.0 - i32::from(other))
    }
}

impl SubAssign<DefenceValue> for AttackValue {
    fn sub_assign(&mut self, other: DefenceValue) {
        self.0 -= i32::from(other);
    }
}
