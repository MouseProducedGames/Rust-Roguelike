/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.

// Standard includes.
use std::ops::{Sub, SubAssign};

// Internal includes.
use super::DefenceValue;
use crate::game::GameValue;

#[derive(Copy, Clone, Default)]
pub struct AttackMarker;

pub type AttackValue = GameValue<AttackMarker>;

impl Sub<DefenceValue> for AttackValue {
    type Output = AttackValue;

    fn sub(self, other: DefenceValue) -> Self {
        self - AttackValue::from(i32::from(other))
    }
}

impl SubAssign<DefenceValue> for AttackValue {
    fn sub_assign(&mut self, other: DefenceValue) {
        *self -= AttackValue::from(i32::from(other));
    }
}
