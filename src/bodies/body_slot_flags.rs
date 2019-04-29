/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.

// Standard includes.

// Internal includes.

#[derive(EnumFlags, Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum BodySlotFlags {
    IsAttack = 0b0000_0001,
    IsDefault = 0b0000_0010,
    IsDefence = 0b0000_0100,
}

pub trait ImplementBodySlotFlags {
    fn is_attack(&self) -> bool;

    fn make_attack(&mut self, value: bool) -> bool;

    fn is_default(&self) -> bool;

    fn make_default(&mut self, value: bool) -> bool;

    fn is_defence(&self) -> bool;

    fn make_defence(&mut self, value: bool) -> bool;
}
