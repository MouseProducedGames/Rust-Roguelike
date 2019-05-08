/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.

// Standard includes.

// Internal includes.
mod damaging_weapon_processor;
mod fine_shield_processor;
mod fine_weapon_processor;
pub use damaging_weapon_processor::{DamagingWeaponFactory, DamagingWeaponProcessor};
pub use fine_shield_processor::{FineShieldFactory, FineShieldProcessor};
pub use fine_weapon_processor::{FineWeaponFactory, FineWeaponProcessor};
