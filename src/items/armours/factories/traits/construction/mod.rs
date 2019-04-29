/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.

// Standard includes.

// Internal includes.
mod chain_armour_processor;
mod leather_armour_processor;
mod plate_armour_processor;
pub use chain_armour_processor::{ChainArmourFactory, ChainArmourProcessor};
pub use leather_armour_processor::{LeatherArmourFactory, LeatherArmourProcessor};
pub use plate_armour_processor::{PlateArmourFactory, PlateArmourProcessor};
