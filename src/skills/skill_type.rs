/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

 **/

// External includes.

// Internal includes.

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub enum SkillActivation
{
    Active(SkillActiveOp),
    _Passive(SkillPassiveOp),
}

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub enum SkillActiveOp
{
    OnUse,
}

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub enum _SkillPassiveOp
{
    EveryRound,
}

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub enum SkillType
{
    Attack(SkillRange, i32, i32),
}

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub enum SkillRange
{
    Radius(u32),
}
