/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use specs::{Entity, World};

// Standard includes.
use std::sync::Arc;

// Internal includes.
use crate::creatures::factories::LeveledCreatureProcessor;
use crate::game::points::BuildLevel;
use crate::items::weapons::WeaponGroup;
use crate::skills::{
    SkillActivation, SkillLookup, SkillPassiveOp, SkillTag, SkillType, SkillValue,
    WeaponSkillTypeLookup,
};

#[derive(Clone, Copy)]
pub struct SpecifiedWeaponSkillProcessor(WeaponGroup);

impl SpecifiedWeaponSkillProcessor {
    pub fn new(weapon_group: WeaponGroup) -> Self {
        Self { 0: weapon_group }
    }

    fn weapon_group(self) -> WeaponGroup {
        self.0
    }
}

impl LeveledCreatureProcessor for SpecifiedWeaponSkillProcessor {
    fn process(&self, world: &mut World, creature_entity: Entity, level: BuildLevel) -> Entity {
        // Only in non-delagating processors.
        // The general creature level is too low to use for skills.
        let level = level + BuildLevel::from(2);
        {
            let mut skill_lookup_storage = world.write_storage::<SkillLookup>();
            let skill_lookup = skill_lookup_storage.get_mut(creature_entity).unwrap();

            let weapon_skill_type_lookup = world.read_resource::<Arc<WeaponSkillTypeLookup>>();

            let mut found = false;
            if let Some(skills) = skill_lookup.get_set_mut(SkillActivation::Passive(
                SkillTag::Combat,
                SkillPassiveOp::OnUse,
            )) {
                for skill in skills {
                    if let SkillType::Weapon(weapon_group, skill_value) = skill {
                        if *weapon_group == self.weapon_group() {
                            found = true;
                            // We do not apply the cost modifier here, because it was done when
                            // the skill was constructed.
                            // Hmm... bit of a problem.
                            *skill_value += SkillValue::new(level.raw());
                            break;
                        }
                    }
                }
            }
            let found = found;

            if found == false {
                let cost_modifier = weapon_skill_type_lookup
                    .get(self.weapon_group())
                    .cost_modifier();
                skill_lookup.insert(
                    SkillActivation::Passive(SkillTag::Combat, SkillPassiveOp::OnUse),
                    SkillType::Weapon(
                        self.weapon_group(),
                        // We apply the cost modifier here.
                        SkillValue::new(level.raw() - cost_modifier.raw()),
                    ),
                );
            }
        }

        creature_entity
    }
}
