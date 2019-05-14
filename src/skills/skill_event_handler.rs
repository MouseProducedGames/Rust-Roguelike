/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use specs::World;

// Standard includes.
use std::sync::{Arc, Mutex, MutexGuard};

// Internal includes.
use super::{
    SkillActivation, SkillLookup, SkillPassiveOp, SkillPoints, SkillTag, SkillType,
    WeaponSkillTypeLookup,
};
use crate::events::{Event, EventManager};
use crate::game::combat::AttackData;
use crate::game::GameValueFixed;
use crate::items::weapons::WeaponGroup;

pub struct SkillEventHandler;

impl SkillEventHandler {
    pub fn new(event_manager: &mut MutexGuard<EventManager>) -> Self {
        event_manager.push_attack_handler(Arc::new(Mutex::new(
            SkillEventHandler::attack_event_handler,
        )));

        Self {}
    }

    fn apply_attack_skill(event_data: &mut AttackData, world: &mut World) {
        let mut found: Option<usize> = None;
        {
            let skill_lookup_storage = world.read_storage::<SkillLookup>();
            if let Some(skill_lookup) = skill_lookup_storage.get(event_data.attacker()) {
                let weapon_skill_type_lookup = world.read_resource::<Arc<WeaponSkillTypeLookup>>();

                if let Some(passive_combat_skills) = skill_lookup.get_set(SkillActivation::Passive(
                    SkillTag::Combat,
                    SkillPassiveOp::OnUse,
                )) {
                    for (i, combat_skill) in passive_combat_skills.iter().enumerate() {
                        if let SkillType::Weapon(weapon_group, skill_value) = combat_skill {
                            if *weapon_group == event_data.weapon_group() {
                                found = Some(i);
                                let attack_modifier = weapon_skill_type_lookup
                                    .get(*weapon_group)
                                    .attack_modifier();
                                *event_data.attack_modifier_mut() += attack_modifier + skill_value;
                            }
                        }
                    }
                }
            }
        }

        let found = found;
        if let Some(index) = found {
            let mut skill_lookup_storage = world.write_storage::<SkillLookup>();
            let skill_lookup = skill_lookup_storage.get_mut(event_data.attacker()).unwrap();
            let skills = skill_lookup.get_or_add_set_mut(SkillActivation::Passive(
                SkillTag::Combat,
                SkillPassiveOp::OnUse,
            ));

            let mut skill_points = skills[index].skill_points(world);
            skill_points += SkillPoints::new(GameValueFixed::from_int(1) / 10);
            skills[index].skill_points_mut(world, skill_points);
        }
    }

    fn apply_defence_skill(event_data: &mut AttackData, world: &mut World) {
        let mut found: Option<usize> = None;
        {
            let skill_lookup_storage = world.read_storage::<SkillLookup>();
            if let Some(skill_lookup) = skill_lookup_storage.get(event_data.defender()) {
                let weapon_skill_type_lookup = world.read_resource::<Arc<WeaponSkillTypeLookup>>();

                if let Some(passive_combat_skills) = skill_lookup.get_set(SkillActivation::Passive(
                    SkillTag::Combat,
                    SkillPassiveOp::OnUse,
                )) {
                    for (i, combat_skill) in passive_combat_skills.iter().enumerate() {
                        if let SkillType::Weapon(weapon_group, skill_value) = combat_skill {
                            if *weapon_group == WeaponGroup::Unarmed {
                                found = Some(i);
                                let defence_modifier = weapon_skill_type_lookup
                                    .get(*weapon_group)
                                    .defence_modifier();
                                *event_data.defence_modifier_mut() +=
                                    defence_modifier + skill_value;
                            }
                        }
                    }
                }
            }
        }

        let found = found;
        if let Some(index) = found {
            let mut skill_lookup_storage = world.write_storage::<SkillLookup>();
            let skill_lookup = skill_lookup_storage.get_mut(event_data.defender()).unwrap();
            let skills = skill_lookup.get_or_add_set_mut(SkillActivation::Passive(
                SkillTag::Combat,
                SkillPassiveOp::OnUse,
            ));

            let mut skill_points = skills[index].skill_points(world);
            skill_points += SkillPoints::new(GameValueFixed::from_int(1) / 10);
            skills[index].skill_points_mut(world, skill_points);
        }
    }

    fn attack_event_handler(event: &mut Event<AttackData>, world: &mut World) {
        let mut event_data = *event.data();

        SkillEventHandler::apply_attack_skill(&mut event_data, world);

        SkillEventHandler::apply_defence_skill(&mut event_data, world);

        *event.data_mut() = event_data;
    }
}
