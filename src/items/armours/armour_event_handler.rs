/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use specs::World;

// Standard includes.
use std::sync::{Arc, Mutex, MutexGuard};

// Internal includes.
use super::Armour;
use crate::bodies::Body;
use crate::events::{Event, EventManager};
use crate::game::combat::{AttackData, ProtectionData};

pub struct ArmourEventHandler;

impl ArmourEventHandler {
    pub fn new(event_manager: &mut MutexGuard<EventManager>) -> Self {
        event_manager.push_attack_handler(Arc::new(Mutex::new(Self::attack_event_handler)));

        event_manager.push_protection_handler(Arc::new(Mutex::new(Self::protection_event_handler)));

        Self {}
    }

    fn attack_event_handler(event: &mut Event<AttackData>, world: &mut World) {
        let mut event_data = *event.data();
        let creature_bodies = world.read_storage::<Body>();
        let armours = world.read_storage::<Armour>();
        if let Some(body) = creature_bodies.get(event_data.defender()) {
            for body_slot in body.get().values() {
                let item = body_slot.item();
                if let Some(armour) = armours.get(item) {
                    *event_data.defence_modifier_mut() += armour.defence_value();
                    break;
                }
            }
        }

        *event.data_mut() = event_data;
    }

    fn protection_event_handler(event: &mut Event<ProtectionData>, world: &mut World) {
        let mut event_data = *event.data();
        let creature_bodies = world.read_storage::<Body>();
        let armours = world.read_storage::<Armour>();
        if let Some(body) = creature_bodies.get(event_data.defender()) {
            for body_slot in body.get().values() {
                let item = body_slot.item();
                if let Some(armour) = armours.get(item) {
                    *event_data.protection_mut() += armour.protection_value();
                    *event_data.armour_group_mut() = armour.armour_group();
                    break;
                }
            }
        }

        *event.data_mut() = event_data;
    }
}
