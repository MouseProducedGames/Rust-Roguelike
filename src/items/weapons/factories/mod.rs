/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.

// Standard includes.

// Internal includes.
mod process_weapon_factory;
pub mod specific;
mod template_weapon_factory;
pub mod traits;
mod weapon_factory;
mod weapon_processor;
pub use process_weapon_factory::ProcessWeaponFactory;
pub use template_weapon_factory::TemplateWeaponFactory;
pub use weapon_factory::WeaponFactory;
pub use weapon_processor::WeaponProcessor;
