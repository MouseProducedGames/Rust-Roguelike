/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use specs::{Component, VecStorage};

// Standard includes.
use std::convert::From;
use std::fmt;

// Internal includes.
use super::{BuildLevel, BuildPoints};
use crate::game::GameValue;

#[derive(Copy, Clone, Default)]
pub struct CurrencyMarker;

pub type CurrencyValue = GameValue<CurrencyMarker>;

impl Component for CurrencyValue {
    type Storage = VecStorage<Self>;
}

impl fmt::Display for CurrencyValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let raw_value = self.raw();
        /*
        // CurrencyValue tracks down to 0.05 units.
        let mut raw_integer_value = raw_value / 20;

        let mut s = String::from("");
        let gold_pieces = raw_integer_value / 400;
        if gold_pieces > 0 {
            raw_integer_value -= gold_pieces * 400;
            s.push_str(&gold_pieces.to_string());
            s.push_str("gp");
        }
        let silver_pieces = raw_integer_value / 20;
        if silver_pieces > 0 {
            raw_integer_value -= silver_pieces * 20;
            if s.is_empty() == false { s.push_str(", "); }

            s.push_str(&silver_pieces.to_string());
            s.push_str("sp");
        }
        let copper_pieces = raw_integer_value;
        if copper_pieces > 0 {
            // Not actually necessary; but there for reference.
            // raw_integer_value -= copper_pieces;
            if s.is_empty() == false { s.push_str(", "); }

            s.push_str(&copper_pieces.to_string());
            s.push_str("cp");
        }

        write!(f, "{}", s)
        */

        write!(f, "${}", raw_value)
    }
}

impl From<BuildLevel> for CurrencyValue {
    fn from(build_level: BuildLevel) -> Self {
        let build_points = BuildPoints::from(build_level);
        CurrencyValue::from(build_points)
    }
}

impl From<&BuildLevel> for CurrencyValue {
    fn from(build_level: &BuildLevel) -> Self {
        Self::from(*build_level)
    }
}

impl From<BuildPoints> for CurrencyValue {
    fn from(build_points: BuildPoints) -> Self {
        let raw_build_points = build_points.raw();
        let bigger_numbers_raw_build_points = raw_build_points * 10;
        let raw_currency_value =
            (bigger_numbers_raw_build_points * bigger_numbers_raw_build_points) / 5;
        CurrencyValue::new(raw_currency_value)
    }
}

impl From<&BuildPoints> for CurrencyValue {
    fn from(build_points: &BuildPoints) -> Self {
        CurrencyValue::from(*build_points)
    }
}
