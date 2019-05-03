/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

 **/
// External includes.

// Standard includes.

// Internal includes.
use super::SkillValue;
use crate::game::combat::{AttackValue, DefenceValue};
use crate::items::weapons::WeaponGroup;

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub enum SkillActivation {
    Active(SkillTag, SkillActiveOp),
    Passive(SkillTag, SkillPassiveOp),
}

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub enum SkillActiveOp {
    OnUse,
}

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub enum SkillPassiveOp {
    EveryRound,
    OnUse,
}

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub enum SkillTag {
    Combat,
    Perception,
}

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub enum SkillType {
    Weapon(WeaponGroup, SkillValue, AttackValue, DefenceValue),
    Skill(i32),
}

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub enum SkillRange {
    Radius(u32),
}
