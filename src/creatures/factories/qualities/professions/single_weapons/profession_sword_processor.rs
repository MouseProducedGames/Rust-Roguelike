/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.

// Standard includes.

// Internal includes.
use super::TemplateProfessionWeaponProcessor;
use crate::creatures::factories::qualities::armaments::ArmedSwordProcessor;
use crate::creatures::factories::qualities::skills::SkillSwordProcessor;

pub type ProfessionSwordProcessor =
    TemplateProfessionWeaponProcessor<ArmedSwordProcessor, SkillSwordProcessor>;
