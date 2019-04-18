/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.

// Standard includes.

// Internal includes.
mod event;
mod event_handler;
mod event_manager;
pub use event::Event;
pub use event_handler::{EventFn, EventHandler, RefEventFn};
pub use event_manager::EventManager;
