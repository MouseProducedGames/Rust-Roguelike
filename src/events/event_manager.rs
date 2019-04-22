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
use crate::dice::roll_attack;
use crate::game::combat::{AttackData, DamageData, DamageValue, InjuryData, InjuryValue};
use crate::game::Time;
use crate::stats::{CreatureStats, Stat};

#[derive(Clone)]
pub struct EventManager {
    attack_events: Arc<Mutex<EventHandler<AttackData>>>,
    damage_events: Arc<Mutex<EventHandler<DamageData>>>,
    injury_events: Arc<Mutex<EventHandler<InjuryData>>>,
}

impl EventManager {
    pub fn new() -> Self {
        Self {
            attack_events: Arc::new(Mutex::new(EventHandler::new())),
            damage_events: Arc::new(Mutex::new(EventHandler::new())),
            injury_events: Arc::new(Mutex::new(EventHandler::new())),
        }
    }

    pub fn push_attack_handler(&mut self, handler: RefEventFn<AttackData>) {
        self.attack_events.lock().unwrap().push_handler(handler);
    }

    pub fn push_damage_handler(&mut self, handler: RefEventFn<DamageData>) {
        self.damage_events.lock().unwrap().push_handler(handler);
    }

    pub fn push_injury_handler(&mut self, handler: RefEventFn<InjuryData>) {
        self.injury_events.lock().unwrap().push_handler(handler);
    }

    pub fn push_attack_event(&mut self, time: Time, attack_data: AttackData) {
        self.attack_events
            .lock()
            .unwrap()
            .push_event(time, attack_data)
    }

    pub fn run_now(&mut self, current_time: Time, world: &mut World) {
        {
            let mut attack_events = self.attack_events.lock().unwrap();
            let mut damage_events = self.damage_events.lock().unwrap();
            while let Some(event) = attack_events.run_once(current_time, world) {
                if roll_attack(
                    event.data().attack_modifier(),
                    event.data().defence_modifier(),
                ) {
                    let damage_data = DamageData::new(
                        event.data().attacker(),
                        event.data().defender(),
                        DamageValue::from(5),
                        event.data().weapon_type(),
                    );
                    damage_events.push_event(current_time, damage_data);
                }
            }
        }

        {
            let mut damage_events = self.damage_events.lock().unwrap();
            let mut injury_events = self.injury_events.lock().unwrap();
            while let Some(event) = damage_events.run_once(current_time, world) {
                if event.data().damage() > 0 {
                    let injury_data = InjuryData::new(
                        event.data().attacker(),
                        event.data().defender(),
                        InjuryValue::from(event.data().damage()),
                        event.data().weapon_type(),
                    );
                    injury_events.push_event(current_time, injury_data);
                }
            }
        }

        {
            let mut injury_events = self.injury_events.lock().unwrap();
            while let Some(event) = injury_events.run_once(current_time, world) {
                if event.data().injury() > 0 {
                    let defender = event.data().defender();
                    let injury = event.data().injury();
                    if let Some(defender_stats) =
                        world.write_storage::<CreatureStats>().get_mut(defender)
                    {
                        *defender_stats.health_mut().value_mut() -= i32::from(injury);
                    }
                }
            }
        }
    }
}
