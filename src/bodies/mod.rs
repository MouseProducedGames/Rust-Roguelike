/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.

// Standard includes.

// Internal includes.
mod body;
mod body_factory;
mod body_screen;
mod body_slot;
mod body_slot_flags;
mod body_slot_type;
pub use body::{Body, BodySystem, CreatureBodySystem};
pub use body_factory::BodyFactory;
pub use body_screen::BodyScreen;
pub use body_slot::BodySlot;
pub use body_slot_flags::{BodySlotFlags, ImplementBodySlotFlags};
pub use body_slot_type::BodySlotType;
