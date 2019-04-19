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
use crate::items::WeaponType;
use crate::skills::{SkillActivation, SkillLookup, SkillPassiveOp, SkillTag, SkillType};

pub struct SkillEventHandler;

impl SkillEventHandler {
    pub fn new(event_manager: &mut MutexGuard<EventManager>) -> Self {
        event_manager.push_attack_handler(Arc::new(Mutex::new(
            SkillEventHandler::attack_event_handler,
        )));

        Self {}
    }

    fn apply_attack_skill(event_data: &mut AttackData, skill_lookup: &mut SkillLookup) {
        let passive_combat_skills = skill_lookup.get_set(SkillActivation::Passive(
            SkillTag::Combat,
            SkillPassiveOp::OnUse,
        ));
        for combat_skill in passive_combat_skills.iter() {
            if let SkillType::Weapon(weapon_type, attack_value, _) = *combat_skill {
                if weapon_type == event_data.weapon_type() {
                    *event_data.attack_modifier_mut() += attack_value
                }
            }
        }
    }

    fn apply_defence_skill(event_data: &mut AttackData, skill_lookup: &mut SkillLookup) {
        let passive_combat_skills = skill_lookup.get_set(SkillActivation::Passive(
            SkillTag::Combat,
            SkillPassiveOp::OnUse,
        ));
        for combat_skill in passive_combat_skills.iter() {
            if let SkillType::Weapon(weapon_type, _, defence_value) = *combat_skill {
                if weapon_type == WeaponType::Unarmed {
                    *event_data.defence_modifier_mut() += defence_value
                }
            }
        }
    }

    fn attack_event_handler(event: &mut Event<AttackData>, world: &mut World) {
        let mut event_data = *event.data();
        let mut skill_storage = world.write_storage::<SkillLookup>();

        if let Some(skill_lookup) = skill_storage.get_mut(event_data.attacker()) {
            SkillEventHandler::apply_attack_skill(&mut event_data, skill_lookup);
        }

        if let Some(skill_lookup) = skill_storage.get_mut(event_data.defender()) {
            SkillEventHandler::apply_defence_skill(&mut event_data, skill_lookup);
        }

        *event.data_mut() = event_data;
    }
}
