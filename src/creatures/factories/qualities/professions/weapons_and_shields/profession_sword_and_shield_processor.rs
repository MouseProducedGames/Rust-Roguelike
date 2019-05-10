/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.

// Standard includes.

// Internal includes.
use super::TemplateProfessionWeaponAndShieldProcessor;
use crate::creatures::factories::qualities::professions::shields::ProfessionShieldProcessor;
use crate::creatures::factories::qualities::professions::single_weapons::ProfessionSwordProcessor;

pub type ProfessionSwordAndShieldProcessor =
    TemplateProfessionWeaponAndShieldProcessor<ProfessionShieldProcessor, ProfessionSwordProcessor>;
