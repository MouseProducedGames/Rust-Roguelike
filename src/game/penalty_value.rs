/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.

// Standard includes.
use std::cmp::{Ord, Ordering, PartialOrd};
use std::convert::From;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::ops::{Add, AddAssign, Deref, DerefMut, Sub, SubAssign};

// Internal includes.
use super::GameValue;

#[derive(Copy, Clone, Default)]
pub struct PenaltyValue<T, TPenalty>(GameValue<T>, PhantomData<TPenalty>)
where
    T: Copy + Clone + Default,
    TPenalty: Copy + Clone + Default;

impl<T, TPenalty> PenaltyValue<T, TPenalty>
where
    T: Copy + Clone + Default,
    TPenalty: Copy + Clone + Default,
{
    pub fn new(value: GameValue<T>) -> Self {
        Self(value.min(GameValue::from(0)), PhantomData)
    }
}

impl<T, TPenalty> Add for PenaltyValue<T, TPenalty>
where
    T: Copy + Clone + Default,
    TPenalty: Copy + Clone + Default,
{
    type Output = PenaltyValue<T, TPenalty>;

    fn add(self, other: PenaltyValue<T, TPenalty>) -> Self {
        PenaltyValue::from(self.0 + other.0)
    }
}

impl<T, TPenalty> AddAssign for PenaltyValue<T, TPenalty>
where
    T: Copy + Clone + Default,
    TPenalty: Copy + Clone + Default,
{
    fn add_assign(&mut self, other: PenaltyValue<T, TPenalty>) {
        *self = PenaltyValue::from(self.0 + other.0);
    }
}

impl<T, TPenalty> Eq for PenaltyValue<T, TPenalty>
where
    T: Copy + Clone + Default,
    TPenalty: Copy + Clone + Default,
{
}

impl<T, TPenalty> Deref for PenaltyValue<T, TPenalty>
where
    T: Copy + Clone + Default,
    TPenalty: Copy + Clone + Default,
{
    type Target = GameValue<T>;

    fn deref(&self) -> &GameValue<T> {
        &self.0
    }
}

impl<T, TPenalty> DerefMut for PenaltyValue<T, TPenalty>
where
    T: Copy + Clone + Default,
    TPenalty: Copy + Clone + Default,
{
    fn deref_mut(&mut self) -> &mut GameValue<T> {
        &mut self.0
    }
}

impl<T, TPenalty> From<GameValue<T>> for PenaltyValue<T, TPenalty>
where
    T: Copy + Clone + Default,
    TPenalty: Copy + Clone + Default,
{
    fn from(value: GameValue<T>) -> Self {
        Self::new(value)
    }
}

impl<T, TPenalty> From<&GameValue<T>> for PenaltyValue<T, TPenalty>
where
    T: Copy + Clone + Default,
    TPenalty: Copy + Clone + Default,
{
    fn from(value: &GameValue<T>) -> Self {
        Self::from(*value)
    }
}

impl<T, TPenalty> From<i32> for PenaltyValue<T, TPenalty>
where
    T: Copy + Clone + Default,
    TPenalty: Copy + Clone + Default,
{
    fn from(value: i32) -> Self {
        Self::new(GameValue::<T>::from(value))
    }
}

impl<T, TPenalty> From<&i32> for PenaltyValue<T, TPenalty>
where
    T: Copy + Clone + Default,
    TPenalty: Copy + Clone + Default,
{
    fn from(value: &i32) -> Self {
        Self::from(*value)
    }
}

impl<T, TPenalty> From<PenaltyValue<T, TPenalty>> for i32
where
    T: Copy + Clone + Default,
    TPenalty: Copy + Clone + Default,
{
    fn from(value: PenaltyValue<T, TPenalty>) -> Self {
        i32::from(value.0)
    }
}

impl<T, TPenalty> From<&PenaltyValue<T, TPenalty>> for i32
where
    T: Copy + Clone + Default,
    TPenalty: Copy + Clone + Default,
{
    fn from(value: &PenaltyValue<T, TPenalty>) -> Self {
        Self::from(*value)
    }
}

impl<T, TPenalty> Hash for PenaltyValue<T, TPenalty>
where
    T: Copy + Clone + Default,
    TPenalty: Copy + Clone + Default,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<T, TPenalty> Ord for PenaltyValue<T, TPenalty>
where
    T: Copy + Clone + Default,
    TPenalty: Copy + Clone + Default,
{
    fn cmp(&self, other: &PenaltyValue<T, TPenalty>) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl<T, TPenalty> PartialEq for PenaltyValue<T, TPenalty>
where
    T: Copy + Clone + Default,
    TPenalty: Copy + Clone + Default,
{
    fn eq(&self, other: &PenaltyValue<T, TPenalty>) -> bool {
        self.0 == other.0
    }
}

impl<T, TPenalty> PartialEq<GameValue<T>> for PenaltyValue<T, TPenalty>
where
    T: Copy + Clone + Default,
    TPenalty: Copy + Clone + Default,
{
    fn eq(&self, other: &GameValue<T>) -> bool {
        self.0 == *other
    }
}

impl<T, TPenalty> PartialEq<i32> for PenaltyValue<T, TPenalty>
where
    T: Copy + Clone + Default,
    TPenalty: Copy + Clone + Default,
{
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl<T, TPenalty> PartialOrd for PenaltyValue<T, TPenalty>
where
    T: Copy + Clone + Default,
    TPenalty: Copy + Clone + Default,
{
    fn partial_cmp(&self, other: &PenaltyValue<T, TPenalty>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T, TPenalty> PartialOrd<GameValue<T>> for PenaltyValue<T, TPenalty>
where
    T: Copy + Clone + Default,
    TPenalty: Copy + Clone + Default,
{
    fn partial_cmp(&self, other: &GameValue<T>) -> Option<Ordering> {
        Some(self.cmp(&PenaltyValue::<T, TPenalty>::from(other)))
    }
}

impl<T, TPenalty> PartialOrd<i32> for PenaltyValue<T, TPenalty>
where
    T: Copy + Clone + Default,
    TPenalty: Copy + Clone + Default,
{
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        Some(self.cmp(&PenaltyValue::<T, TPenalty>::from(other)))
    }
}

impl<T, TPenalty> Sub for PenaltyValue<T, TPenalty>
where
    T: Copy + Clone + Default,
    TPenalty: Copy + Clone + Default,
{
    type Output = PenaltyValue<T, TPenalty>;

    fn sub(self, other: PenaltyValue<T, TPenalty>) -> Self {
        PenaltyValue::from(self.0 - other.0)
    }
}

impl<T, TPenalty> SubAssign for PenaltyValue<T, TPenalty>
where
    T: Copy + Clone + Default,
    TPenalty: Copy + Clone + Default,
{
    fn sub_assign(&mut self, other: PenaltyValue<T, TPenalty>) {
        *self = PenaltyValue::from(self.0 - other.0);
    }
}
