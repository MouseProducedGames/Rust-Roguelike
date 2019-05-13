/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.

// Standard includes.
use super::TemplateGameValue;
use std::convert::From;

// Internal includes.

pub type GameValue<TMarker> = TemplateGameValue<i32, TMarker>;

/* impl<TMarker> From<i32> for GameValue<TMarker>
where
    TMarker: Copy + Clone + Default,
{
    fn from(other: i32) -> Self {
        Self::new(other)
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

impl<TMarker> From<GameValue<TMarker>> for i32
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
