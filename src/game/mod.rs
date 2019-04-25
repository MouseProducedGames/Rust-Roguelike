/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.

// Standard includes.

// Internal includes.
pub mod combat;
mod entity_position_tracker;
mod game_state;
mod game_value;
mod last_update_system;
mod time;
pub use combat::{AttackData, DamageData, InjuryData};
pub use entity_position_tracker::EntityPositionTracker;
pub use game_state::GameState;
pub use game_value::GameValue;
pub use last_update_system::LastUpdateSystem;
pub use time::Time;
