/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

 **/
// External includes.

// Standard includes.

// Internal includes.
mod skill_lookup;
mod skill_type;
pub use skill_lookup::SkillLookup;
pub use skill_type::{
    SkillActivation, SkillActiveOp, SkillPassiveOp, SkillRange, SkillTag, SkillType,
};
