/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use specs::World;

// Standard includes.
use std::sync::{Arc, Mutex, MutexGuard};

// Internal includes.
use super::{AttackValue, MultiAttackPenalty};
use crate::events::{Event, EventManager};
use crate::game::combat::AttackData;

pub struct AttackPenaltyEventHandler;

impl AttackPenaltyEventHandler {
    pub fn new(event_manager: &mut MutexGuard<EventManager>) -> Self {
        event_manager.push_attack_handler(Arc::new(Mutex::new(Self::attack_event_handler)));

        Self {}
    }

    fn attack_event_handler(event: &mut Event<AttackData>, world: &mut World) {
        let mut event_data = *event.data();
        let mut multi_attack_penalties = world.write_storage::<MultiAttackPenalty>();
        let multi_attack_penalty = multi_attack_penalties.get_mut(event_data.attacker());
        if let Some(multi_attack_penalty) = multi_attack_penalty {
            let attack_value = **multi_attack_penalty;
            *event_data.attack_modifier_mut() -= attack_value;
            **multi_attack_penalty += AttackValue::new(-4);
        }

        *event.data_mut() = event_data;
    }
}
