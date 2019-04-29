/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use specs::World;

// Standard includes.
use std::sync::{Arc, Mutex, MutexGuard};

// Internal includes.
use super::Weapon;
use crate::bodies::{Body, ImplementBodySlotFlags};
use crate::events::{Event, EventManager};
use crate::game::combat::{AttackData, DamageData};

pub struct WeaponEventHandler;

impl WeaponEventHandler {
    pub fn new(event_manager: &mut MutexGuard<EventManager>) -> Self {
        event_manager.push_attack_handler(Arc::new(Mutex::new(
            WeaponEventHandler::attack_event_handler,
        )));
        event_manager.push_damage_handler(Arc::new(Mutex::new(
            WeaponEventHandler::damage_event_handler,
        )));

        Self {}
    }

    fn attack_event_handler(event: &mut Event<AttackData>, world: &mut World) {
        let mut event_data = *event.data();
        let creature_bodies = world.read_storage::<Body>();
        let weapons = world.read_storage::<Weapon>();
        /* if let Some(body) = creature_bodies.get(event_data.attacker()) {
            for body_slot in body.get().values() {
                let item = body_slot.item();
                if let Some(weapon) = weapons.get(item) {
                    *event_data.attack_modifier_mut() += weapon.attack_value();
                    *event_data.weapon_group_mut() = weapon.weapon_group();
                    break;
                }
            }
        } */

        if let Some(body) = creature_bodies.get(event_data.defender()) {
            for body_slot in body.get().values() {
                if body_slot.is_defence() {
                    let item = body_slot.item();
                    if let Some(weapon) = weapons.get(item) {
                        *event_data.defence_modifier_mut() += weapon.defence_value();
                        break;
                    }
                }
            }
        }

        *event.data_mut() = event_data;
    }

    fn damage_event_handler(event: &mut Event<DamageData>, world: &mut World) {
        let mut event_data = *event.data();
        let creature_bodies = world.read_storage::<Body>();
        let weapons = world.read_storage::<Weapon>();
        if let Some(body) = creature_bodies.get(event_data.attacker()) {
            for body_slot in body.get().values() {
                let item = body_slot.item();
                if let Some(weapon) = weapons.get(item) {
                    if weapon.weapon_group() == event_data.weapon_group() {
                        *event_data.damage_mut() += weapon.damage_value();
                        *event_data.weapon_group_mut() = weapon.weapon_group();
                        break;
                    }
                }
            }
        }

        *event.data_mut() = event_data;
    }
}
