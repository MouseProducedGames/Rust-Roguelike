/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use specs::{Component, VecStorage, World};
pub use specs::{Entities, ReadExpect, ReadStorage, System, WriteExpect};

// Standard includes.
use std::sync::{Arc, Mutex, MutexGuard};

// Internal includes.
use crate::events::{Event, EventManager};
use crate::game::combat::{DamageType, InjuryData, InjuryValue};

#[derive(Clone, Copy, Default)]
pub struct Undead;

impl Undead {}

impl Component for Undead {
    type Storage = VecStorage<Self>;
}

pub struct UndeadEventHandler;

impl UndeadEventHandler {
    pub fn new(event_manager: &mut MutexGuard<EventManager>) -> Self {
        event_manager.push_injury_handler(Arc::new(Mutex::new(Self::injury_event_handler)));

        Self {}
    }

    fn injury_event_handler(event: &mut Event<InjuryData>, world: &mut World) {
        let mut event_data = *event.data();
        let undead_lookup = world.read_storage::<Undead>();
        if undead_lookup.get(event_data.defender()).is_some() {
            let injury_value = i32::from(event_data.injury());
            *event_data.injury_mut() = InjuryValue::from(match event_data.damage_type() {
                DamageType::Blunt => (injury_value * 2) / 3,
                DamageType::Crushing => injury_value / 2,
                // The value-relative spillover of Blunt and Slashing, rounded to the tens place.
                DamageType::Piercing => (injury_value * 2) / 7,
                DamageType::Slashing => (injury_value * 2) / 3,
            });
        }

        *event.data_mut() = event_data;
    }
}
