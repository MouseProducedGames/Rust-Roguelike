/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.

// Standard includes.
use std::convert::From;

// Internal includes.
use super::DamageValue;
use crate::game::GameValue;

#[derive(Copy, Clone, Default)]
pub struct InjuryMarker;

pub type InjuryValue = GameValue<InjuryMarker>;

impl From<DamageValue> for InjuryValue {
    fn from(value: DamageValue) -> Self {
        InjuryValue::from(i32::from(value))
    }
}
