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
use crate::game::combat::{
    AttackData, AttackValue, DamageData, DamageValue, DefenceValue, InjuryData, InjuryValue,
};
use crate::stats::{CreatureStats, Stat, StatModifier};

pub struct StatEventHandler;

impl StatEventHandler {
    pub fn new(event_manager: &mut MutexGuard<EventManager>) -> Self {
        event_manager
            .push_attack_handler(Arc::new(Mutex::new(StatEventHandler::attack_event_handler)));
        event_manager
            .push_damage_handler(Arc::new(Mutex::new(StatEventHandler::damage_event_handler)));
        /* event_manager
        .push_injury_handler(Arc::new(Mutex::new(StatEventHandler::injury_event_handler))); */

        Self {}
    }

    fn attack_event_handler(event: &mut Event<AttackData>, world: &mut World) {
        let mut event_data = *event.data();
        let creature_stats = world.read_storage::<CreatureStats>();
        if let Some(attacker_stats) = creature_stats.get(event_data.attacker()) {
            if let Some(defender_stats) = creature_stats.get(event_data.defender()) {
                *event_data.attack_modifier_mut() +=
                    AttackValue::from(attacker_stats.coordination().modifier());
                *event_data.defence_modifier_mut() +=
                    DefenceValue::from(defender_stats.agility().modifier());

                *event.data_mut() = event_data;
            }
        }
    }

    fn damage_event_handler(event: &mut Event<DamageData>, world: &mut World) {
        let mut event_data = *event.data();
        let creature_stats = world.read_storage::<CreatureStats>();
        if let Some(attacker_stats) = creature_stats.get(event_data.attacker()) {
            *event_data.damage_mut() += DamageValue::from(attacker_stats.strength().modifier());

            *event.data_mut() = event_data;
        }
    }

    fn _injury_event_handler(event: &mut Event<InjuryData>, world: &mut World) {
        let mut event_data = *event.data();
        let mut creature_stats = world.write_storage::<CreatureStats>();
        if let Some(defender_stats) = creature_stats.get_mut(event_data.defender()) {
            *defender_stats.health_mut().value_mut() -= i32::from(event_data.injury());
            *event_data.injury_mut() = InjuryValue::from(0);

            *event.data_mut() = event_data;
        }
    }
}
