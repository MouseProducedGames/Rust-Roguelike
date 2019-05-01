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
use std::ops::{Add, AddAssign, Sub, SubAssign};

// Internal includes.

#[derive(Copy, Clone, Default)]
pub struct GameValue<T>(i32, PhantomData<T>)
where
    T: Copy + Clone + Default;

impl<T> GameValue<T>
where
    T: Copy + Clone + Default,
{
    fn new(value: i32) -> Self {
        Self(value, PhantomData)
    }
}

impl<T> Add for GameValue<T>
where
    T: Copy + Clone + Default,
{
    type Output = GameValue<T>;

    fn add(self, other: GameValue<T>) -> Self {
        GameValue::from(self.0 + other.0)
    }
}

impl<T> AddAssign for GameValue<T>
where
    T: Copy + Clone + Default,
{
    fn add_assign(&mut self, other: GameValue<T>) {
        self.0 += other.0;
    }
}

impl<T> Eq for GameValue<T> where T: Copy + Clone + Default {}

impl<T> From<i32> for GameValue<T>
where
    T: Copy + Clone + Default,
{
    fn from(value: i32) -> Self {
        Self::new(value)
    }
}

impl<T> From<&i32> for GameValue<T>
where
    T: Copy + Clone + Default,
{
    fn from(value: &i32) -> Self {
        Self::from(*value)
    }
}

impl<T> From<GameValue<T>> for i32
where
    T: Copy + Clone + Default,
{
    fn from(value: GameValue<T>) -> Self {
        value.0
    }
}

impl<T> From<&GameValue<T>> for i32
where
    T: Copy + Clone + Default,
{
    fn from(value: &GameValue<T>) -> Self {
        Self::from(*value)
    }
}

impl<T> Hash for GameValue<T>
where
    T: Copy + Clone + Default,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<T> Ord for GameValue<T>
where
    T: Copy + Clone + Default,
{
    fn cmp(&self, other: &GameValue<T>) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl<T> PartialEq for GameValue<T>
where
    T: Copy + Clone + Default,
{
    fn eq(&self, other: &GameValue<T>) -> bool {
        self.0 == other.0
    }
}

impl<T> PartialEq<i32> for GameValue<T>
where
    T: Copy + Clone + Default,
{
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl<T> PartialOrd for GameValue<T>
where
    T: Copy + Clone + Default,
{
    fn partial_cmp(&self, other: &GameValue<T>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> PartialOrd<i32> for GameValue<T>
where
    T: Copy + Clone + Default,
{
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        Some(self.cmp(&GameValue::<T>::from(other)))
    }
}

impl<T> Sub for GameValue<T>
where
    T: Copy + Clone + Default,
{
    type Output = GameValue<T>;

    fn sub(self, other: GameValue<T>) -> Self {
        GameValue::from(self.0 - other.0)
    }
}

impl<T> SubAssign for GameValue<T>
where
    T: Copy + Clone + Default,
{
    fn sub_assign(&mut self, other: GameValue<T>) {
        self.0 -= other.0;
    }
}
