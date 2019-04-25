/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.

// Standard includes.

// Internal includes.
use crate::game::GameValue;

#[derive(Copy, Clone, Default)]
pub struct ProtectionMarker;

pub type ProtectionValue = GameValue<ProtectionMarker>;
