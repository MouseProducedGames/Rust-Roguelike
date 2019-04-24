/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.

// Standard includes.

// Internal includes.

#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum BodySlotType {
    Hand,
    Palm,
    Torso,
}
