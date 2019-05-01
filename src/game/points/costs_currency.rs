/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use specs::World;

// Standard includes.
use std::convert::From;

// Internal includes.
use super::{CostsBuildPoints, CurrencyValue};

pub trait CostsCurrency {
    fn currency_total(&self, world: &World) -> CurrencyValue;
}

impl CostsCurrency for CostsBuildPoints {
    fn currency_total(&self, world: &World) -> CurrencyValue {
        CurrencyValue::from(self.build_points_total(world))
    }
}
