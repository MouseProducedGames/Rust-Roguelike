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
use super::{BuildLevel, CurrencyValue};
use crate::game::GameValue;

#[derive(Copy, Clone, Default)]
pub struct BuildPointsMarker;

pub type BuildPoints = GameValue<BuildPointsMarker>;

impl Component for BuildPoints {
    type Storage = VecStorage<Self>;
}

impl fmt::Display for BuildPoints {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.raw())
    }
}

impl From<BuildLevel> for BuildPoints {
    fn from(build_level: BuildLevel) -> Self {
        let raw_build_level: i32 = build_level.raw();
        let float_build_level: f64 = f64::from(raw_build_level) / 10.0;
        let float_build_points: f64 = 1.259_921_049_894_873_2_f64.powf(float_build_level);
        let raw_build_points: i32 = (float_build_points * 10.0).ceil() as i32;

        BuildPoints::new(raw_build_points)
    }
}

impl From<&BuildLevel> for BuildPoints {
    fn from(build_level: &BuildLevel) -> Self {
        Self::from(*build_level)
    }
}

impl From<CurrencyValue> for BuildPoints {
    fn from(currency_value: CurrencyValue) -> Self {
        let raw_currency_value = currency_value.raw();
        let float_currency_value = f64::from(raw_currency_value * 5);
        let float_build_points = float_currency_value.sqrt();
        let raw_build_points = float_build_points as i32;
        BuildPoints::new(raw_build_points)
    }
}

impl From<&CurrencyValue> for BuildPoints {
    fn from(currency_value: &CurrencyValue) -> Self {
        BuildPoints::from(*currency_value)
    }
}
