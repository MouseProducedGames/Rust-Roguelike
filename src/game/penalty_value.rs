/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.

// Standard includes.
use std::convert::From;
use std::marker::PhantomData;

// Internal includes.
use super::{GameValue, GameValueFixed, TemplateGameValue};

pub type PenaltyValue<T, TPenalty> = TemplateGameValue<GameValue<T>, PhantomData<TPenalty>>;

impl<T, TPenalty> From<i32> for PenaltyValue<T, TPenalty>
where
    T: Copy + Clone + Default,
    TPenalty: Copy + Clone + Default,
{
    fn from(other: i32) -> Self {
        Self::new(GameValue::new(GameValueFixed::from_int(other)))
    }
}

impl<T, TPenalty> From<&i32> for PenaltyValue<T, TPenalty>
where
    T: Copy + Clone + Default,
    TPenalty: Copy + Clone + Default,
{
    fn from(other: &i32) -> Self {
        Self::from(*other)
    }
}
