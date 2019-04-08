/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes

// Standard includes.

// internal includes
pub mod console;
mod creature_display_system;
mod display;
mod input;
mod input_instance;
mod player_display_system;
use input_instance::InputData;
pub use creature_display_system::CreatureDisplaySystem;
pub use display::Display;
pub use input::Input;
pub use input_instance::InputInstance;
pub use player_display_system::PlayerDisplaySystem;
