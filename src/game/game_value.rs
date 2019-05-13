/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use fixed::types::I20F12;

// Standard includes.
use std::convert::From;

// Internal includes.
use super::TemplateGameValue;

pub type GameValueFixed = I20F12;

pub type GameValue<TMarker> = TemplateGameValue<GameValueFixed, TMarker>;

impl<TMarker> From<i32> for GameValue<TMarker>
where
    TMarker: Copy + Clone + Default,
{
    fn from(other: i32) -> Self {
        Self::new(GameValueFixed::from_int(other))
    }
}

impl<TMarker> From<&i32> for GameValue<TMarker>
where
    TMarker: Copy + Clone + Default,
{
    fn from(other: &i32) -> Self {
        Self::from(*other)
    }
}

/* impl<TMarker> From<GameValue<TMarker>> for i32
where
    TMarker: Copy + Clone + Default,
{
    fn from(other: GameValue<TMarker>) -> Self {
        other.raw()
    }
}

impl<TMarker> From<&GameValue<TMarker>> for i32
where
    TMarker: Copy + Clone + Default,
{
    fn from(other: &GameValue<TMarker>) -> Self {
        Self::from(*other)
    }
} */
