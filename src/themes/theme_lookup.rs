/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use rand::{thread_rng, Rng};

// Standard includes.
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex, MutexGuard};

// Internal includes.
use super::Theme;
use crate::dungen::DungeonGenerator;

pub struct ThemeLookup {
    values: HashMap<String, Arc<Mutex<Theme>>>,
    top_level_theme_names: HashSet<String>
}

impl ThemeLookup {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            top_level_theme_names: HashSet::new(),
        }
    }
    
    pub fn add_theme(
        &mut self,
        name: String,
        sub_themes: &[Arc<Mutex<Theme>>],
        dungeon_generators: &[Arc<Mutex<dyn DungeonGenerator>>]
    ) -> MutexGuard<Theme> {
        let output =
            self.values
            .entry(name.clone())
            .or_insert_with(|| Arc::new(Mutex::new(Theme::new(name, sub_themes, dungeon_generators))));
        let output = output.lock().unwrap();
        output
    }
    
    pub fn make_theme_top_level(&mut self, name: String) -> (bool, String, &'static str) {
        if self.values.contains_key(&name) {
            if self.top_level_theme_names.contains(&name) {
                return (true, name, "Already a top-level theme.");
            } else {
                self.top_level_theme_names.insert(name.clone());
                return (true, name, "Theme exists and is now a top-level theme.");
            }
        }
        
        return (false, name, "No such theme exists.");
    }
    
    pub fn get_random_top_level_theme(&self) -> MutexGuard<Theme> {
        let index = thread_rng().gen_range(0, self.top_level_theme_names.len());
        for (i, name) in self.top_level_theme_names.iter().enumerate() {
            if i == index {
                if let Some(output) = self.values.get(name) {
                    let output = output.lock().unwrap();
                    return output;
                } else {
                    break;
                }
            }
        }
        panic!("Theme count shrunk while we were observing it!");
    }
}