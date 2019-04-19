/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use specs::World;

// Standard includes.
use std::collections::BinaryHeap;
use std::sync::{Arc, Mutex};

// internal includes.
use super::Event;
use crate::game::Time;

pub type EventFn<TData> = Fn(&mut Event<TData>, &mut World) + Send;
pub type RefEventFn<TData> = Arc<Mutex<EventFn<TData>>>;

pub struct EventHandler<TData: Clone + Eq + PartialEq + Sized> {
    event_handlers: Vec<RefEventFn<TData>>,
    events: BinaryHeap<Event<TData>>,
}

impl<TData: Clone + Eq + PartialEq + Sized> EventHandler<TData> {
    pub fn new() -> Self {
        Self {
            event_handlers: vec![],
            events: BinaryHeap::new(),
        }
    }

    pub fn push_handler(&mut self, handler: RefEventFn<TData>) {
        self.event_handlers.push(handler);
    }

    pub fn push_event(&mut self, time: Time, data: TData) {
        self.events.push(Event::new(time, data))
    }

    pub fn run_once(&mut self, current_time: Time, world: &mut World) -> Option<Event<TData>> {
        if let Some(mut event) = self.events.pop() {
            if event.time() <= current_time {
                let event = &mut event;
                for handler in self.event_handlers.iter() {
                    (*handler.lock().unwrap())(event, world);
                }

                return Some(event.clone());
            } else {
                return None;
            }
        } else {
            None
        }
    }
}
