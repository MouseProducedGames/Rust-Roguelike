/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.

// Standard includes.

// Internal includes.
pub mod systems;
mod command;
mod decisions_instinct;
mod love_fear_hate;
mod player_marker;
mod player_position;
mod viewpoint_marker;
pub use command::Command;
pub use decisions_instinct::_make_decision_attack;
pub use love_fear_hate::_LoveFearHate;
pub use player_marker::PlayerMarker;
pub use player_position::PlayerPosition;
pub use viewpoint_marker::ViewpointMarker;
