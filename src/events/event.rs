/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.

// Standard includes.
use std::cmp::Ordering;

// Internal includes.
use crate::game::Time;

#[derive(Clone, Eq, PartialEq)]
pub struct Event<TData: Clone + Eq + PartialEq + Sized>(Time, TData);

impl<TData: Clone + Eq + PartialEq + Sized> Event<TData> {
    pub fn new(time: Time, data: TData) -> Self {
        Self(time, data)
    }

    pub fn data(&self) -> &TData {
        &self.1
    }

    pub fn data_mut(&mut self) -> &mut TData {
        &mut self.1
    }

    pub fn time(&self) -> Time {
        self.0
    }
}

impl<TData: Clone + Eq + PartialEq + Sized> Ord for Event<TData> {
    fn cmp(&self, other: &Event<TData>) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl<TData: Clone + Eq + PartialEq + Sized> PartialOrd for Event<TData> {
    fn partial_cmp(&self, other: &Event<TData>) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}
