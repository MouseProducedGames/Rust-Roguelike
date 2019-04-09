/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.

// Standard includes.
use std::sync::{Arc, Mutex, MutexGuard};

// Internal includes.
use crate::dungen::DungeonGenerator;

pub struct Theme {
    name: String,
    dungeon_generator_count: usize,
    sub_themes: Vec<Arc<Mutex<Theme>>>,
    dungeon_generators: Vec<Arc<Mutex<dyn DungeonGenerator>>>,
}

impl Theme {
    pub(crate) fn new(
        name: String,
        sub_themes: &[Arc<Mutex<Theme>>],
        dungeon_generators: &[Arc<Mutex<dyn DungeonGenerator>>]
    ) -> Self {
        let mut dungeon_generator_count = dungeon_generators.len();
        for sub_theme in sub_themes {
            let sub_theme = sub_theme.lock().unwrap();
            dungeon_generator_count += sub_theme.dungeon_generator_count();
        }
        
        let mut output =
            Self {
                name,
                dungeon_generator_count,
                sub_themes: vec![],
                dungeon_generators: vec![],
            };
        
        output.sub_themes.extend_from_slice(sub_themes);
        output.dungeon_generators.extend_from_slice(dungeon_generators);
        
        output
    }
    
    pub fn dungeon_generator_count(&self) -> usize {
        self.dungeon_generator_count
    }
    
    pub fn for_all_dungeon_generators(
        &self,
        call: fn(dungen: MutexGuard<(dyn DungeonGenerator + 'static)>)
    ) {
        for dungen in self.dungeon_generators.iter() {
            let dungen = dungen.lock().unwrap();
            call(dungen);
        }
        
        for sub_theme in self.sub_themes.iter() {
            let sub_theme = sub_theme.lock().unwrap();
            sub_theme.for_all_dungeon_generators(call);
        }
    }
    
    pub fn name(&self) -> &String {
        &self.name
    }
}