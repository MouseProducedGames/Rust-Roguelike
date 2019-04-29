/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.

// Standard includes.

// Internal includes.
mod armour_factory;
mod armour_processor;
mod process_armour_factory;
pub mod specific;
mod template_armour_factory;
pub mod traits;
pub use armour_factory::ArmourFactory;
pub use armour_processor::ArmourProcessor;
pub use process_armour_factory::ProcessArmourFactory;
pub use template_armour_factory::TemplateArmourFactory;
