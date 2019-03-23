/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

 **/
// External includes.

// Internal includes.
pub mod skill_lookup;
pub mod skill_type;
pub use skill_lookup::SkillLookup;
pub use skill_type::{
    SkillActivation, SkillActiveOp, SkillPassiveOp, SkillRange, SkillTag, SkillType,
};
