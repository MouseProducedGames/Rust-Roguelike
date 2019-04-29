/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.

// Standard includes.
use std::sync::{Arc, Mutex};

// Internal includes.
use crate::creatures::CreatureFactory;
use crate::dungen::DungeonGenerator;
use crate::maps::MapProcessor;

pub struct Theme {
    name: String,
    creature_factory_count: usize,
    dungeon_generator_count: usize,
    map_processor_count: usize,
    sub_themes: Vec<Arc<Mutex<Theme>>>,
    creature_factories: Vec<Arc<Mutex<CreatureFactory>>>,
    dungeon_generators: Vec<Arc<Mutex<dyn DungeonGenerator>>>,
    map_processors: Vec<Arc<Mutex<MapProcessor>>>,
}

impl Theme {
    pub(crate) fn new(
        name: String,
        sub_themes: &[Arc<Mutex<Theme>>],
        creature_factories: &[Arc<Mutex<CreatureFactory>>],
        dungeon_generators: &[Arc<Mutex<dyn DungeonGenerator>>],
        map_processors: &[Arc<Mutex<MapProcessor>>],
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

        let mut map_processor_count = map_processors.len();
        for sub_theme in sub_themes {
            let sub_theme = sub_theme.lock().unwrap();
            map_processor_count += sub_theme.map_processor_count();
        }

        let mut output = Self {
            name,
            creature_factory_count,
            dungeon_generator_count,
            map_processor_count,
            sub_themes: vec![],
            creature_factories: vec![],
            dungeon_generators: vec![],
            map_processors: vec![],
        };

        output.sub_themes.extend_from_slice(sub_themes);
        output
            .creature_factories
            .extend_from_slice(creature_factories);
        output
            .dungeon_generators
            .extend_from_slice(dungeon_generators);
        output.map_processors.extend_from_slice(map_processors);

        output
    }

    pub fn creature_factory_count(&self) -> usize {
        self.creature_factory_count
    }

    pub fn dungeon_generator_count(&self) -> usize {
        self.dungeon_generator_count
    }

    pub fn map_processor_count(&self) -> usize {
        self.map_processor_count
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

    pub fn for_all_map_processors<TFunc>(&self, call: &mut TFunc)
    where
        TFunc: FnMut(usize, &Arc<Mutex<MapProcessor>>),
    {
        let mut index: usize = 0;
        self.for_all_map_processors_impl(&mut index, call);
    }

    pub fn name(&self) -> &String {
        &self.name
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

    fn for_all_map_processors_impl<TFunc>(&self, index: &mut usize, call: &mut TFunc)
    where
        TFunc: FnMut(usize, &Arc<Mutex<MapProcessor>>),
    {
        for dungen in self.map_processors.iter() {
            call(*index, dungen);
            *index += 1;
        }

        for sub_theme in self.sub_themes.iter() {
            let sub_theme = sub_theme.lock().unwrap();
            sub_theme.for_all_map_processors_impl(index, call);
        }
    }
}
