/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.

// Standard includes.

// Internal includes.
mod attribute;
mod creature_stats;
mod stat;
mod stat_event_handler;
mod stat_modifier;
pub use attribute::Attribute;
pub use creature_stats::CreatureStats;
pub use stat::Stat;
pub use stat_event_handler::StatEventHandler;
pub use stat_modifier::StatModifier;
