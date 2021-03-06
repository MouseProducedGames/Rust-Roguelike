/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.

// Standard includes.

// Internal includes.
mod creature_factory;
mod creature_processor;
mod leveled_creature_processor;
mod process_creature_factory;
pub mod qualities;
pub mod specific;
mod template_creature_factory;
pub mod traits;
pub use creature_factory::CreatureFactory;
pub use creature_processor::CreatureProcessor;
pub use leveled_creature_processor::LeveledCreatureProcessor;
pub use process_creature_factory::ProcessCreatureFactory;
pub use template_creature_factory::TemplateCreatureFactory;
