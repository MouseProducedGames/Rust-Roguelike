/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.

// Standard includes.

// Internal includes.
pub mod axes;
pub mod body_parts;
mod leveled_weapon_generator;
pub mod maces;
pub mod shields;
pub mod spears;
pub mod swords;
mod unmodified_weapon_factory;
pub use leveled_weapon_generator::_LeveledWeaponGenerator;
pub use unmodified_weapon_factory::UnmodifiedWeaponFactory;
