/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use specs::{Component, VecStorage};

// Standard includes.
use std::fmt;

// Internal includes.
use super::BuildPointsValue;
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

impl From<BuildPointsValue> for CurrencyValue {
    fn from(build_points_value: BuildPointsValue) -> Self {
        let raw_build_level = i32::from(build_points_value);
        let raw_built_points_total =
            1.259_921_049_894_873_2_f64.powf(f64::from(raw_build_level) / 30.0) * 6.0;
        let raw_currency_value = raw_built_points_total.powi(2);
        CurrencyValue::from(raw_currency_value as i32)
    }
}

impl From<&BuildPointsValue> for CurrencyValue {
    fn from(build_points_value: &BuildPointsValue) -> Self {
        CurrencyValue::from(*build_points_value)
    }
}
