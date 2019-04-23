/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes

// Standard includes.

// internal includes
pub mod console;
mod description;
mod display;
mod display_option;
mod input;
mod input_instance;
pub mod r8g8b8_console;
pub use description::{LongDescription, ShortDescription};
pub use display::Display;
pub use display_option::DisplayOption;
pub use input::Input;
use input_instance::InputData;
pub use input_instance::InputInstance;
