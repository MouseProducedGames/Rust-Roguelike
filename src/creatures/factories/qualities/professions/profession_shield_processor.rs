/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.

// Standard includes.

// Internal includes.
use super::TemplateProfessionWeaponProcessor;
use crate::creatures::factories::qualities::armaments::ArmedShieldProcessor;
use crate::creatures::factories::qualities::skills::SkillShieldProcessor;

pub type ProfessionShieldProcessor =
    TemplateProfessionWeaponProcessor<ArmedShieldProcessor, SkillShieldProcessor>;
