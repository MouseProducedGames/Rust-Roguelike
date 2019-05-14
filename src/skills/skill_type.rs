/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

 **/
// External includes.
use specs::World;

// Standard includes.
use std::convert::From;
use std::sync::Arc;

// Internal includes.
use super::{SkillPoints, SkillValue, WeaponSkillTypeLookup};
use crate::game::points::CostsBuildPoints;
use crate::items::weapons::WeaponGroup;

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub enum SkillActivation {
    _Active(SkillTag, SkillActiveOp),
    Passive(SkillTag, SkillPassiveOp),
}

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub enum SkillActiveOp {
    _OnUse,
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
    Weapon(WeaponGroup, SkillValue),
    Skill(SkillValue),
}

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub enum SkillRange {
    _Radius(u32),
}

impl SkillType {
    pub fn skill_points(self, world: &World) -> SkillPoints {
        let skill_value = match self {
            SkillType::Weapon(weapon_group, skill_value) => {
                let weapon_skill_type_lookup = world.read_resource::<Arc<WeaponSkillTypeLookup>>();
                let cost_modifier = weapon_skill_type_lookup.get(weapon_group).cost_modifier();
                SkillValue::new(skill_value.raw() + cost_modifier.raw())
            }
            SkillType::Skill(skill_value) => skill_value,
        };

        let build_points = skill_value.build_points_total(world);
        SkillPoints::from(build_points)
    }

    pub fn skill_points_mut(&mut self, world: &World, skill_points: SkillPoints) {
        let new_skill_value = SkillValue::from(skill_points);
        match self {
            SkillType::Weapon(weapon_group, skill_value) => {
                let weapon_skill_type_lookup = world.read_resource::<Arc<WeaponSkillTypeLookup>>();
                let cost_modifier = weapon_skill_type_lookup.get(*weapon_group).cost_modifier();
                *skill_value = SkillValue::new(new_skill_value.raw() - cost_modifier.raw());
            }
            SkillType::Skill(skill_value) => *skill_value = new_skill_value,
        };
    }
}
