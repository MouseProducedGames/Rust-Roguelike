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
    IsDefence = 0b0000_0010,
}
