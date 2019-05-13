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
use crate::game::points::BuildLevel;
use crate::items::weapons::WeaponGroup;
use crate::skills::{
    SkillActivation, SkillLookup, SkillPassiveOp, SkillTag, SkillType, SkillValue,
};

#[derive(Clone)]
pub struct SkillMaceProcessor;

impl SkillMaceProcessor {}

impl Default for SkillMaceProcessor {
    fn default() -> Self {
        Self {}
    }
}

impl LeveledCreatureProcessor for SkillMaceProcessor {
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
                    if let SkillType::Weapon(weapon_group, skill_value) = skill {
                        if *weapon_group == WeaponGroup::Maces {
                            found = true;
                            // We do not add ?? here, because it was done when the skill was
                            // constructed.
                            // Hmm... bit of a problem.
                            *skill_value += SkillValue::new(level.raw());
                        }
                    }
                }
            }
            let found = found;

            if found == false {
                skill_lookup.insert(
                    SkillActivation::Passive(SkillTag::Combat, SkillPassiveOp::OnUse),
                    SkillType::Weapon(
                        WeaponGroup::Maces,
                        // We add ?? here to apply the BuildLevel reduction.
                        // Hmm... bit of a problem.
                        SkillValue::new(level.raw()),
                    ),
                );
            }
        }

        creature_entity
    }
}
