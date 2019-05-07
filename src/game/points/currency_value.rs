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
        let raw_value = i32::from(self);
        // CurrencyValue tracks down to 1 units.
        let rrl_bucks = raw_value / 1;
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

        write!(f, "${}", rrl_bucks)
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
        let raw_build_points = i32::from(build_points);
        let raw_currency_value = raw_build_points.pow(2) / 5;
        CurrencyValue::from(raw_currency_value as i32)
    }
}

impl From<&BuildPoints> for CurrencyValue {
    fn from(build_points: &BuildPoints) -> Self {
        CurrencyValue::from(*build_points)
    }
}