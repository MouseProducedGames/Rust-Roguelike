/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

 **/
// External includes.

// Standard includes.

// Internal includes.
mod talent_funcs;
mod talent_lookup;
mod talent_type;
pub use talent_funcs::talent_range_func;
pub use talent_lookup::TalentLookup;
pub use talent_type::{TalentActivation, TalentActivationOp, TalentRange, TalentType};
