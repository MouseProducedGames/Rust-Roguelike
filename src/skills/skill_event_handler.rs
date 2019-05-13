/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use specs::World;

// Standard includes.
use std::sync::{Arc, Mutex, MutexGuard};

// Internal includes.
use crate::events::{Event, EventManager};
use crate::game::combat::AttackData;
use crate::items::weapons::WeaponGroup;
use crate::skills::{
    SkillActivation, SkillLookup, SkillPassiveOp, SkillTag, SkillType, WeaponSkillTypeLookup,
};

pub struct SkillEventHandler;

impl SkillEventHandler {
    pub fn new(event_manager: &mut MutexGuard<EventManager>) -> Self {
        event_manager.push_attack_handler(Arc::new(Mutex::new(
            SkillEventHandler::attack_event_handler,
        )));

        Self {}
    }

    fn apply_attack_skill(
        event_data: &mut AttackData,
        skill_lookup: &SkillLookup,
        weapon_skill_type_lookup: &Arc<WeaponSkillTypeLookup>,
    ) {
        if let Some(passive_combat_skills) = skill_lookup.get_set(SkillActivation::Passive(
            SkillTag::Combat,
            SkillPassiveOp::OnUse,
        )) {
            for combat_skill in passive_combat_skills.iter() {
                if let SkillType::Weapon(weapon_group, skill_value) = *combat_skill {
                    if weapon_group == event_data.weapon_group() {
                        let attack_modifier =
                            weapon_skill_type_lookup.get(weapon_group).attack_modifier();
                        *event_data.attack_modifier_mut() += attack_modifier + skill_value;
                    }
                }
            }
        }
    }

    fn apply_defence_skill(
        event_data: &mut AttackData,
        skill_lookup: &SkillLookup,
        weapon_skill_type_lookup: &Arc<WeaponSkillTypeLookup>,
    ) {
        if let Some(passive_combat_skills) = skill_lookup.get_set(SkillActivation::Passive(
            SkillTag::Combat,
            SkillPassiveOp::OnUse,
        )) {
            for combat_skill in passive_combat_skills.iter() {
                if let SkillType::Weapon(weapon_group, skill_value) = *combat_skill {
                    if weapon_group == WeaponGroup::Unarmed {
                        let defence_modifier = weapon_skill_type_lookup
                            .get(weapon_group)
                            .defence_modifier();
                        *event_data.defence_modifier_mut() += defence_modifier + skill_value;
                    }
                }
            }
        }
    }

    fn attack_event_handler(event: &mut Event<AttackData>, world: &mut World) {
        let mut event_data = *event.data();
        let skill_storage = world.read_storage::<SkillLookup>();
        let weapon_skill_type_lookup = world.read_resource::<Arc<WeaponSkillTypeLookup>>();

        if let Some(skill_lookup) = skill_storage.get(event_data.attacker()) {
            SkillEventHandler::apply_attack_skill(
                &mut event_data,
                skill_lookup,
                &weapon_skill_type_lookup,
            );
        }

        if let Some(skill_lookup) = skill_storage.get(event_data.defender()) {
            SkillEventHandler::apply_defence_skill(
                &mut event_data,
                skill_lookup,
                &weapon_skill_type_lookup,
            );
        }

        *event.data_mut() = event_data;
    }
}
