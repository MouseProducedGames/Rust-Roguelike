/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.

// Standard includes.

// internal includes.
mod ability;
mod ability_system;
mod func;
mod undead;
pub use ability::{Ability, AbilityActivation, AbilityActivationOp, AbilityRange};
pub use ability_system::AbilitySystem;
pub use undead::{Undead, UndeadEventHandler};
