/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use rand::{thread_rng, Rng};

// Standard includes.
use std::sync::{Arc, Mutex, MutexGuard};

// Internal includes.
use crate::creatures::CreatureFactory;
use crate::dungen::DungeonGenerator;

pub struct Theme {
    name: String,
    creature_factory_count: usize,
    dungeon_generator_count: usize,
    sub_themes: Vec<Arc<Mutex<Theme>>>,
    creature_factories: Vec<Arc<Mutex<CreatureFactory>>>,
    dungeon_generators: Vec<Arc<Mutex<dyn DungeonGenerator>>>,
}

impl Theme {
    pub(crate) fn new(
        name: String,
        sub_themes: &[Arc<Mutex<Theme>>],
        creature_factories: &[Arc<Mutex<CreatureFactory>>],
        dungeon_generators: &[Arc<Mutex<dyn DungeonGenerator>>],
    ) -> Self {
        let mut creature_factory_count = creature_factories.len();
        for sub_theme in sub_themes {
            let sub_theme = sub_theme.lock().unwrap();
            creature_factory_count += sub_theme.creature_factory_count();
        }

        let mut dungeon_generator_count = dungeon_generators.len();
        for sub_theme in sub_themes {
            let sub_theme = sub_theme.lock().unwrap();
            dungeon_generator_count += sub_theme.dungeon_generator_count();
        }

        let mut output = Self {
            name,
            creature_factory_count,
            dungeon_generator_count,
            sub_themes: vec![],
            creature_factories: vec![],
            dungeon_generators: vec![],
        };

        output.sub_themes.extend_from_slice(sub_themes);
        output
            .creature_factories
            .extend_from_slice(creature_factories);
        output
            .dungeon_generators
            .extend_from_slice(dungeon_generators);

        output
    }

    pub fn creature_factory_count(&self) -> usize {
        self.creature_factory_count
    }

    pub fn dungeon_generator_count(&self) -> usize {
        self.dungeon_generator_count
    }

    pub fn for_all_creature_factories<TFunc>(&self, call: &mut TFunc)
    where
        TFunc: FnMut(usize, &Arc<Mutex<(CreatureFactory)>>),
    {
        let mut index: usize = 0;
        self.for_all_creature_factories_impl(&mut index, call);
    }

    pub fn for_all_dungeon_generators<TFunc>(&self, call: &mut TFunc)
    where
        TFunc: FnMut(usize, &Arc<Mutex<(dyn DungeonGenerator + 'static)>>),
    {
        let mut index: usize = 0;
        self.for_all_dungeon_generators_impl(&mut index, call);
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn get_random_creature_factory<TFunc>(&self, call: &mut TFunc)
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

    fn for_all_creature_factories_impl<TFunc>(&self, index: &mut usize, call: &mut TFunc)
    where
        TFunc: FnMut(usize, &Arc<Mutex<(CreatureFactory)>>),
    {
        for dungen in self.creature_factories.iter() {
            call(*index, dungen);
            *index += 1;
        }

        for sub_theme in self.sub_themes.iter() {
            let sub_theme = sub_theme.lock().unwrap();
            sub_theme.for_all_creature_factories_impl(index, call);
        }
    }

    fn for_all_dungeon_generators_impl<TFunc>(&self, index: &mut usize, call: &mut TFunc)
    where
        TFunc: FnMut(usize, &Arc<Mutex<(dyn DungeonGenerator + 'static)>>),
    {
        for dungen in self.dungeon_generators.iter() {
            call(*index, dungen);
            *index += 1;
        }

        for sub_theme in self.sub_themes.iter() {
            let sub_theme = sub_theme.lock().unwrap();
            sub_theme.for_all_dungeon_generators_impl(index, call);
        }
    }
}
