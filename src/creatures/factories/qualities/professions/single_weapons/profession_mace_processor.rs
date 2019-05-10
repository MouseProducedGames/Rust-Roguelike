/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.

// Standard includes.

// Internal includes.
use super::TemplateProfessionWeaponProcessor;
use crate::creatures::factories::qualities::armaments::ArmedMaceProcessor;
use crate::creatures::factories::qualities::skills::SkillMaceProcessor;

pub type ProfessionMaceProcessor =
    TemplateProfessionWeaponProcessor<ArmedMaceProcessor, SkillMaceProcessor>;
