/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.

// Standard includes.

// Internal includes.
use super::TemplateProfessionWeaponProcessor;
use crate::creatures::factories::qualities::armaments::ArmedSpearProcessor;
use crate::creatures::factories::qualities::skills::SkillSpearProcessor;

pub type ProfessionSpearProcessor =
    TemplateProfessionWeaponProcessor<ArmedSpearProcessor, SkillSpearProcessor>;
