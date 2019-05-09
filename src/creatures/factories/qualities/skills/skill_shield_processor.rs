/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use specs::{Entity, World};

// Standard includes.
use std::default::Default;

// Internal includes.
use crate::creatures::factories::LeveledCreatureProcessor;
use crate::game::combat::{AttackValue, DefenceValue};
use crate::game::points::BuildLevel;
use crate::items::weapons::WeaponGroup;
use crate::skills::{
    SkillActivation, SkillLookup, SkillPassiveOp, SkillTag, SkillType, SkillValue,
};

#[derive(Clone)]
pub struct SkillShieldProcessor;

impl SkillShieldProcessor {}

impl Default for SkillShieldProcessor {
    fn default() -> Self {
        Self {}
    }
}

impl LeveledCreatureProcessor for SkillShieldProcessor {
    fn process(&self, world: &mut World, creature_entity: Entity, level: BuildLevel) -> Entity {
        // Only in non-delagating processors.
        // The general creature level is too low to use for skills.
        let level = level + BuildLevel::from(2);
        {
            let mut skill_lookup_storage = world.write_storage::<SkillLookup>();
            let skill_lookup = skill_lookup_storage.get_mut(creature_entity).unwrap();

            let mut found = false;
            if let Some(skills) = skill_lookup.get_set_mut(SkillActivation::Passive(
                SkillTag::Combat,
                SkillPassiveOp::OnUse,
            )) {
                for skill in skills {
                    if let SkillType::Weapon(weapon_group, skill_value, _, _, _) = skill {
                        if *weapon_group == WeaponGroup::Shields {
                            found = true;
                            // We do not subtract 1 here, because it was done when the skill was
                            // constructed.
                            *skill_value += SkillValue::from(i32::from(level));
                        }
                    }
                }
            }
            let found = found;

            if found == false {
                skill_lookup.insert(
                    SkillActivation::Passive(SkillTag::Combat, SkillPassiveOp::OnUse),
                    SkillType::Weapon(
                        WeaponGroup::Shields,
                        // We subtract 1 here to apply the BuildLevel reduction.
                        SkillValue::from(i32::from(level) + 1),
                        BuildLevel::from(-10),
                        AttackValue::from(-2),
                        DefenceValue::from(0),
                    ),
                );
            }
        }

        creature_entity
    }
}
