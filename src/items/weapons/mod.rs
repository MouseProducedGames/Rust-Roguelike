/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.

// Standard includes.

// Internal includes.
pub mod factories;
mod weapon;
mod weapon_event_handler;
mod weapon_group;
pub use weapon::Weapon;
pub use weapon_event_handler::WeaponEventHandler;
pub use weapon_group::WeaponGroup;
