/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use specs::{Component, VecStorage};

// Standard includes.
use std::convert::From;
use std::ops::{Add, AddAssign, Sub, SubAssign};

// Internal includes.
use super::{Stat, StatModifier};
use crate::game::GameValueFixed;

#[derive(Copy, Clone)]
pub struct Attribute {
    value: GameValueFixed,
}

impl Attribute {
    pub fn new(value: GameValueFixed) -> Self {
        Self { value }
    }
}

impl Add<GameValueFixed> for Attribute {
    type Output = Attribute;

    fn add(self, other: GameValueFixed) -> Attribute {
        Attribute::new(self.value() + other)
    }
}

impl AddAssign<GameValueFixed> for Attribute {
    fn add_assign(&mut self, other: GameValueFixed) {
        *self.value_mut() += other;
    }
}

impl Component for Attribute {
    type Storage = VecStorage<Self>;
}

impl From<i32> for Attribute {
    fn from(other: i32) -> Self {
        Self::new(GameValueFixed::from_int(other))
    }
}

impl From<&i32> for Attribute {
    fn from(other: &i32) -> Self {
        Self::from(*other)
    }
}

impl StatModifier for Attribute {}

impl Stat for Attribute {
    fn value(&self) -> GameValueFixed {
        self.value
    }

    fn value_mut(&mut self) -> &mut GameValueFixed {
        &mut self.value
    }
}

impl Sub<GameValueFixed> for Attribute {
    type Output = Attribute;

    fn sub(self, other: GameValueFixed) -> Attribute {
        Attribute::new(self.value() - other)
    }
}

impl SubAssign<GameValueFixed> for Attribute {
    fn sub_assign(&mut self, other: GameValueFixed) {
        *self.value_mut() -= other;
    }
}
