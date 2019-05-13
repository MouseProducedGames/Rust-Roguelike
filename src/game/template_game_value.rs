/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.

// Standard includes.
use std::cmp::{Ord, Ordering, PartialOrd};
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::ops::{Add, AddAssign, Sub, SubAssign};

// Internal includes.

#[derive(Copy, Clone, Default)]
pub struct TemplateGameValue<TData, TMarker>(TData, PhantomData<TMarker>)
where
    TData: Add<Output = TData>
        + AddAssign
        + Eq
        + Copy
        + Clone
        + Default
        + Hash
        + Ord
        + PartialEq
        + PartialOrd
        + Sized
        + Sub<Output = TData>
        + SubAssign,
    TMarker: Copy + Clone + Default;

impl<TData, TMarker> TemplateGameValue<TData, TMarker>
where
    TData: Add<Output = TData>
        + AddAssign
        + Eq
        + Copy
        + Clone
        + Default
        + Hash
        + Ord
        + PartialEq
        + PartialOrd
        + Sized
        + Sub<Output = TData>
        + SubAssign,
    TMarker: Copy + Clone + Default,
{
    pub fn new(value: TData) -> Self {
        Self(value, PhantomData)
    }

    pub fn raw(self) -> TData {
        self.0
    }
}

impl<TData, TMarker> Add for TemplateGameValue<TData, TMarker>
where
    TData: Add<Output = TData>
        + AddAssign
        + Eq
        + Copy
        + Clone
        + Default
        + Hash
        + Ord
        + PartialEq
        + PartialOrd
        + Sized
        + Sub<Output = TData>
        + SubAssign,
    TMarker: Copy + Clone + Default,
{
    type Output = TemplateGameValue<TData, TMarker>;

    fn add(self, other: TemplateGameValue<TData, TMarker>) -> Self {
        TemplateGameValue::new(self.0 + other.0)
    }
}

impl<TData, TMarker> AddAssign for TemplateGameValue<TData, TMarker>
where
    TData: Add<Output = TData>
        + AddAssign
        + Eq
        + Copy
        + Clone
        + Default
        + Hash
        + Ord
        + PartialEq
        + PartialOrd
        + Sized
        + Sub<Output = TData>
        + SubAssign,
    TMarker: Copy + Clone + Default,
{
    fn add_assign(&mut self, other: TemplateGameValue<TData, TMarker>) {
        self.0 += other.0;
    }
}

impl<TData, TMarker> Eq for TemplateGameValue<TData, TMarker>
where
    TData: Add<Output = TData>
        + AddAssign
        + Eq
        + Copy
        + Clone
        + Default
        + Hash
        + Ord
        + PartialEq
        + PartialOrd
        + Sized
        + Sub<Output = TData>
        + SubAssign,
    TMarker: Copy + Clone + Default,
{
}

impl<TData, TMarker> Hash for TemplateGameValue<TData, TMarker>
where
    TData: Add<Output = TData>
        + AddAssign
        + Eq
        + Copy
        + Clone
        + Default
        + Hash
        + Ord
        + PartialEq
        + PartialOrd
        + Sized
        + Sub<Output = TData>
        + SubAssign,
    TMarker: Copy + Clone + Default,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<TData, TMarker> Ord for TemplateGameValue<TData, TMarker>
where
    TData: Add<Output = TData>
        + AddAssign
        + Eq
        + Copy
        + Clone
        + Default
        + Hash
        + Ord
        + PartialEq
        + PartialOrd
        + Sized
        + Sub<Output = TData>
        + SubAssign,
    TMarker: Copy + Clone + Default,
{
    fn cmp(&self, other: &TemplateGameValue<TData, TMarker>) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl<TData, TMarker> PartialEq for TemplateGameValue<TData, TMarker>
where
    TData: Add<Output = TData>
        + AddAssign
        + Eq
        + Copy
        + Clone
        + Default
        + Hash
        + Ord
        + PartialEq
        + PartialOrd
        + Sized
        + Sub<Output = TData>
        + SubAssign,
    TMarker: Copy + Clone + Default,
{
    fn eq(&self, other: &TemplateGameValue<TData, TMarker>) -> bool {
        self.0 == other.0
    }
}

impl<TData, TMarker> PartialEq<TData> for TemplateGameValue<TData, TMarker>
where
    TData: Add<Output = TData>
        + AddAssign
        + Eq
        + Copy
        + Clone
        + Default
        + Hash
        + Ord
        + PartialEq
        + PartialOrd
        + Sized
        + Sub<Output = TData>
        + SubAssign,
    TMarker: Copy + Clone + Default,
{
    fn eq(&self, other: &TData) -> bool {
        self.0 == *other
    }
}

impl<TData, TMarker> PartialOrd for TemplateGameValue<TData, TMarker>
where
    TData: Add<Output = TData>
        + AddAssign
        + Eq
        + Copy
        + Clone
        + Default
        + Hash
        + Ord
        + PartialEq
        + PartialOrd
        + Sized
        + Sub<Output = TData>
        + SubAssign,
    TMarker: Copy + Clone + Default,
{
    fn partial_cmp(&self, other: &TemplateGameValue<TData, TMarker>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<TData, TMarker> PartialOrd<TData> for TemplateGameValue<TData, TMarker>
where
    TData: Add<Output = TData>
        + AddAssign
        + Eq
        + Copy
        + Clone
        + Default
        + Hash
        + Ord
        + PartialEq
        + PartialOrd
        + Sized
        + Sub<Output = TData>
        + SubAssign,
    TMarker: Copy + Clone + Default,
{
    fn partial_cmp(&self, other: &TData) -> Option<Ordering> {
        Some(self.cmp(&Self::new(*other)))
    }
}

impl<TData, TMarker> Sub for TemplateGameValue<TData, TMarker>
where
    TData: Add<Output = TData>
        + AddAssign
        + Eq
        + Copy
        + Clone
        + Default
        + Hash
        + Ord
        + PartialEq
        + PartialOrd
        + Sized
        + Sub<Output = TData>
        + SubAssign,
    TMarker: Copy + Clone + Default,
{
    type Output = TemplateGameValue<TData, TMarker>;

    fn sub(self, other: TemplateGameValue<TData, TMarker>) -> Self {
        TemplateGameValue::new(self.0 - other.0)
    }
}

impl<TData, TMarker> SubAssign for TemplateGameValue<TData, TMarker>
where
    TData: Add<Output = TData>
        + AddAssign
        + Eq
        + Copy
        + Clone
        + Default
        + Hash
        + Ord
        + PartialEq
        + PartialOrd
        + Sized
        + Sub<Output = TData>
        + SubAssign,
    TMarker: Copy + Clone + Default,
{
    fn sub_assign(&mut self, other: TemplateGameValue<TData, TMarker>) {
        self.0 -= other.0;
    }
}
