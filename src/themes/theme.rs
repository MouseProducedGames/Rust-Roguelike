/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use rand::{thread_rng, Rng};

// Standard includes.
use std::sync::{Arc, Mutex, MutexGuard};

// Internal includes.
use crate::dungen::DungeonGenerator;

#[derive(Clone)]
pub struct Theme {
    name: String,
    dungeon_generator_count: usize,
    sub_themes: Arc<Mutex<Vec<Arc<Mutex<Theme>>>>>,
    dungeon_generators: Arc<Mutex<Vec<Arc<Mutex<dyn DungeonGenerator>>>>>,
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
                sub_themes: Arc::new(Mutex::new(vec![])),
                dungeon_generators: Arc::new(Mutex::new(vec![])),
            };
        
        output.sub_themes.lock().unwrap().extend_from_slice(sub_themes);
        output.dungeon_generators.lock().unwrap().extend_from_slice(dungeon_generators);
        
        output
    }
    
    pub fn dungeon_generator_count(&self) -> usize {
        self.dungeon_generator_count
    }
    
    pub fn get_dungeon_generator(&self) -> MutexGuard<(dyn DungeonGenerator + 'static)> {
        let mut index = thread_rng().gen_range(0, self.dungeon_generator_count());
        if let Some(output) = self.get_dungeon_generator_impl(&mut index) {
            return output;
        } else {
            panic!("Dungeon generator counts don't match!");
        }
    }
    
    pub fn name(&self) -> &String {
        &self.name
    }
    
    fn get_dungeon_generator_impl(
        &self,
        index: &mut usize
    ) -> Option<MutexGuard<(dyn DungeonGenerator + 'static)>> {
        let dungeon_generators = self.dungeon_generators.lock().unwrap();
        if *index < dungeon_generators.len() {
            let dungeon_generator = dungeon_generators[*index];
            let dungeon_generator = dungeon_generator.lock().unwrap();
            return Some(dungeon_generator);
        } else {
            *index -= dungeon_generators.len();
            let sub_themes = self.sub_themes.lock().unwrap();
            for sub_theme in sub_themes.iter() {
                let sub_theme = sub_theme.lock().unwrap();
                let output = sub_theme.get_dungeon_generator_impl(index);
                if let Some(temp) = output {
                    return output;
                }
            }
            return None;
        }
    }
}