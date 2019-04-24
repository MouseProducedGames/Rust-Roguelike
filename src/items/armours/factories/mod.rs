/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.

// Standard includes.

// Internal includes.
mod armour_factory;
mod armour_processor;
mod chain_armour_processor;
mod cuirass_factory;
mod gauntlet_factory;
mod hand_factory;
mod leather_armour_processor;
mod plate_armour_processor;
mod process_armour_factory;
mod template_armour_factory;
mod torso_factory;
pub use armour_factory::ArmourFactory;
pub use armour_processor::ArmourProcessor;
pub use chain_armour_processor::{ChainArmourFactory, ChainArmourProcessor};
pub use cuirass_factory::CuirassFactory;
pub use gauntlet_factory::GauntletFactory;
pub use hand_factory::HandFactory;
pub use leather_armour_processor::{LeatherArmourFactory, LeatherArmourProcessor};
pub use plate_armour_processor::{PlateArmourFactory, PlateArmourProcessor};
pub use process_armour_factory::ProcessArmourFactory;
pub use template_armour_factory::TemplateArmourFactory;
pub use torso_factory::TorsoFactory;
