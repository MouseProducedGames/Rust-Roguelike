/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use rand::{thread_rng, Rng};

// Standard includes.
use std::sync::{Arc, Mutex};

// Internal includes.
use super::Theme;
use crate::creatures::CreatureFactory;
use crate::dungen::DungeonGenerator;
use crate::world::MapProcessor;

pub trait ThemeHelper {
    fn get_random_creature_factory<TFunc>(&self, call: &mut TFunc)
    where
        TFunc: FnMut(usize, &Arc<Mutex<CreatureFactory>>);

    fn get_random_dungeon_generator<TFunc>(&self, call: &mut TFunc)
    where
        TFunc: FnMut(usize, &Arc<Mutex<dyn DungeonGenerator>>);

    fn get_random_map_processor<TFunc>(&self, call: &mut TFunc)
    where
        TFunc: FnMut(usize, &Arc<Mutex<MapProcessor>>);
}

impl ThemeHelper for Theme {
    fn get_random_creature_factory<TFunc>(&self, call: &mut TFunc)
    where
        TFunc: FnMut(usize, &Arc<Mutex<CreatureFactory>>),
    {
        let index = thread_rng().gen_range(0, self.creature_factory_count());
        self.for_all_creature_factories(
            &mut |current_index: usize, dungen: &Arc<Mutex<CreatureFactory>>| {
                if current_index == index {
                    // let dungen = dungen.clone();
                    call(current_index, &dungen);
                }
            },
        );
    }

    fn get_random_dungeon_generator<TFunc>(&self, call: &mut TFunc)
    where
        TFunc: FnMut(usize, &Arc<Mutex<dyn DungeonGenerator>>),
    {
        let index = thread_rng().gen_range(0, self.creature_factory_count());
        self.for_all_dungeon_generators(
            &mut |current_index: usize, dungen: &Arc<Mutex<dyn DungeonGenerator>>| {
                if current_index == index {
                    // let dungen = dungen.clone();
                    call(current_index, &dungen);
                }
            },
        );
    }

    fn get_random_map_processor<TFunc>(&self, call: &mut TFunc)
    where
        TFunc: FnMut(usize, &Arc<Mutex<MapProcessor>>),
    {
        let index = thread_rng().gen_range(0, self.creature_factory_count());
        self.for_all_map_processors(
            &mut |current_index: usize, dungen: &Arc<Mutex<MapProcessor>>| {
                if current_index == index {
                    // let dungen = dungen.clone();
                    call(current_index, &dungen);
                }
            },
        );
    }
}
