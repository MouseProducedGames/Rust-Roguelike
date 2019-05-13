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
use super::{BuildPoints, CurrencyValue};
use crate::game::GameValue;

#[derive(Copy, Clone, Default)]
pub struct BuildLevelMarker;

pub type BuildLevel = GameValue<BuildLevelMarker>;

impl Component for BuildLevel {
    type Storage = VecStorage<Self>;
}

impl fmt::Display for BuildLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.raw())
    }
}

impl From<BuildPoints> for BuildLevel {
    fn from(build_points: BuildPoints) -> Self {
        let raw_build_points = build_points.raw();
        let float_build_points = f64::from(raw_build_points) / 10.0;
        let float_build_level = float_build_points.log(1.259_921_049_894_873_2);
        let raw_build_level = (float_build_level * 10.0).ceil() as i32;
        BuildLevel::new(raw_build_level)
    }
}

impl From<&BuildPoints> for BuildLevel {
    fn from(build_level: &BuildPoints) -> Self {
        Self::from(*build_level)
    }
}

impl From<CurrencyValue> for BuildLevel {
    fn from(currency_value: CurrencyValue) -> Self {
        let build_points = BuildPoints::from(currency_value);
        Self::from(build_points)
    }
}

impl From<&CurrencyValue> for BuildLevel {
    fn from(currency_value: &CurrencyValue) -> Self {
        Self::from(*currency_value)
    }
}
