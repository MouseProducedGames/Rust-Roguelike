/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use specs::World;

// Standard includes.
use std::sync::{Arc, Mutex};

// internal includes.
use super::{EventHandler, RefEventFn};
use crate::bodies::{Body, ImplementBodySlotFlags};
use crate::dice::roll_attack;
use crate::game::combat::{
    AttackActionData, AttackData, AttackValue, DamageData, DamageType, DamageValue, DefenceValue,
    InjuryData, ProtectionData, ProtectionValue,
};
use crate::game::Time;
use crate::items::armours::ArmourGroup;
use crate::items::weapons::{Weapon, WeaponGroup};
use crate::stats::{CreatureStats, Stat};

#[derive(Clone)]
pub struct EventManager {
    attack_events: Arc<Mutex<EventHandler<AttackData>>>,
    attack_action_events: Arc<Mutex<EventHandler<AttackActionData>>>,
    damage_events: Arc<Mutex<EventHandler<DamageData>>>,
    injury_events: Arc<Mutex<EventHandler<InjuryData>>>,
    protection_events: Arc<Mutex<EventHandler<ProtectionData>>>,
}

impl EventManager {
    pub fn new() -> Self {
        Self {
            attack_events: Arc::new(Mutex::new(EventHandler::new())),
            attack_action_events: Arc::new(Mutex::new(EventHandler::new())),
            damage_events: Arc::new(Mutex::new(EventHandler::new())),
            injury_events: Arc::new(Mutex::new(EventHandler::new())),
            protection_events: Arc::new(Mutex::new(EventHandler::new())),
        }
    }

    pub fn push_attack_handler(&mut self, handler: RefEventFn<AttackData>) {
        self.attack_events.lock().unwrap().push_handler(handler);
    }

    pub fn _push_attack_action_handler(&mut self, handler: RefEventFn<AttackActionData>) {
        self.attack_action_events
            .lock()
            .unwrap()
            .push_handler(handler);
    }

    pub fn push_damage_handler(&mut self, handler: RefEventFn<DamageData>) {
        self.damage_events.lock().unwrap().push_handler(handler);
    }

    pub fn push_injury_handler(&mut self, handler: RefEventFn<InjuryData>) {
        self.injury_events.lock().unwrap().push_handler(handler);
    }

    pub fn push_protection_handler(&mut self, handler: RefEventFn<ProtectionData>) {
        self.protection_events.lock().unwrap().push_handler(handler);
    }

    pub fn push_attack_action_event(&mut self, time: Time, attack_action_data: AttackActionData) {
        self.attack_action_events
            .lock()
            .unwrap()
            .push_event(time, attack_action_data)
    }

    /* pub fn push_attack_event(&mut self, time: Time, attack_data: AttackData) {
        self.attack_events
            .lock()
            .unwrap()
            .push_event(time, attack_data)
    } */

    pub fn run_now(&mut self, current_time: Time, world: &mut World) {
        {
            let mut attack_action_events = self.attack_action_events.lock().unwrap();
            let mut attack_events = self.attack_events.lock().unwrap();
            while let Some(event) = attack_action_events.run_once(current_time, world) {
                let bodies = world.read_storage::<Body>();
                let weapons = world.read_storage::<Weapon>();
                let event_data = event.data();
                if let Some(body) = bodies.get(event_data.attacker()) {
                    for body_slot in body.get().values() {
                        if body_slot.is_attack() {
                            let item_entity = body_slot.item();
                            let mut attack_modifier = AttackValue::from(0);
                            let mut weapon_group = WeaponGroup::Unarmed;
                            let mut damage_type = DamageType::Blunt;
                            if let Some(weapon) = weapons.get(item_entity) {
                                attack_modifier += weapon.attack_value();
                                damage_type = weapon.damage_type();
                                weapon_group = weapon.weapon_group();
                            }

                            let attack_modifier = attack_modifier;
                            let weapon_group = weapon_group;
                            attack_events.push_event(
                                current_time,
                                AttackData::new(
                                    event_data.attacker(),
                                    event_data.defender(),
                                    attack_modifier,
                                    DefenceValue::from(0),
                                    weapon_group,
                                    damage_type,
                                    WeaponGroup::Unarmed,
                                ),
                            );
                        }
                    }
                }
            }
        }

        {
            let mut attack_events = self.attack_events.lock().unwrap();
            let mut damage_events = self.damage_events.lock().unwrap();
            while let Some(event) = attack_events.run_once(current_time, world) {
                let result = roll_attack(
                    event.data().attack_modifier(),
                    event.data().defence_modifier(),
                );
                if result.is_success() {
                    let damage_data = DamageData::new(
                        event.data().attacker(),
                        event.data().defender(),
                        DamageValue::new(result.margin_of_success()),
                        event.data().weapon_group(),
                        event.data().damage_type(),
                    );
                    damage_events.push_event(current_time, damage_data);
                }
            }
        }

        {
            let mut damage_events = self.damage_events.lock().unwrap();
            let mut protection_events = self.protection_events.lock().unwrap();
            while let Some(event) = damage_events.run_once(current_time, world) {
                let event_data = *event.data();
                let damage_type = event_data.damage_type();
                let damage = damage_type.damage_to_damage(event_data.damage());
                if damage.raw() > 0 {
                    let protection_data = ProtectionData::new(
                        event_data.attacker(),
                        event_data.defender(),
                        damage,
                        ProtectionValue::from(0),
                        ArmourGroup::Default,
                        event_data.weapon_group(),
                        damage_type,
                    );
                    protection_events.push_event(current_time, protection_data);
                }
            }
        }

        {
            let mut protection_events = self.protection_events.lock().unwrap();
            let mut injury_events = self.injury_events.lock().unwrap();
            while let Some(event) = protection_events.run_once(current_time, world) {
                let event_data = *event.data();
                let damage = event_data.damage();
                let damage_type = event_data.damage_type();
                let protection = event_data.protection();
                let injury_total = damage_type.damage_to_injury(damage - protection);

                if injury_total.raw() > 0 {
                    let injury_data = InjuryData::new(
                        event_data.attacker(),
                        event_data.defender(),
                        injury_total,
                        event_data.weapon_group(),
                        damage_type,
                    );
                    injury_events.push_event(current_time, injury_data);
                }
            }
        }

        {
            let mut injury_events = self.injury_events.lock().unwrap();
            while let Some(event) = injury_events.run_once(current_time, world) {
                if event.data().injury().raw() > 0 {
                    let defender = event.data().defender();
                    let injury = event.data().injury();
                    if let Some(defender_stats) =
                        world.write_storage::<CreatureStats>().get_mut(defender)
                    {
                        *defender_stats.health_mut().value_mut() -= injury.raw();
                    }
                }
            }
        }
    }
}
