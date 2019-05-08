/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.

// Standard includes.

// Internal includes.
mod large_shield_processor;
mod medium_shield_processor;
mod small_shield_processor;
pub use large_shield_processor::{LargeShieldFactory, LargeShieldProcessor};
pub use medium_shield_processor::{MediumShieldFactory, MediumShieldProcessor};
pub use small_shield_processor::{SmallShieldFactory, SmallShieldProcessor};
