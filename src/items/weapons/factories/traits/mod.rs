/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.

// Standard includes.

// Internal includes.
mod fine_weapon_processor;
mod rusty_weapon_processor;
pub use fine_weapon_processor::{FineWeaponFactory, FineWeaponProcessor};
pub use rusty_weapon_processor::{RustyWeaponFactory, RustyWeaponProcessor};
