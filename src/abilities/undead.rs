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
use crate::game::combat::{InjuryData, InjuryValue};

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
            let mut injury_value = event_data.injury();
            injury_value = InjuryValue::from(i32::from(injury_value) / 2);
            *event_data.injury_mut() = injury_value;
        }

        *event.data_mut() = event_data;
    }
}
