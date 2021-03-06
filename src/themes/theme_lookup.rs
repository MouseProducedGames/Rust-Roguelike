/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use rand::{thread_rng, Rng};

// Standard includes.
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

// Internal includes.
use super::Theme;
use crate::creatures::CreatureFactoryWrapper;
use crate::dungen::DungeonGenerator;
use crate::maps::MapProcessor;

pub struct ThemeLookup {
    values: HashMap<String, Arc<Mutex<Theme>>>,
    top_level_theme_names: HashSet<String>,
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
        creature_factories: &[Arc<Mutex<CreatureFactoryWrapper>>],
        dungeon_generators: &[Arc<Mutex<dyn DungeonGenerator>>],
        map_processors: &[Arc<Mutex<MapProcessor>>],
    ) -> Arc<Mutex<Theme>> {
        let output = self.values.entry(name.clone()).or_insert_with(|| {
            Arc::new(Mutex::new(Theme::new(
                name,
                sub_themes,
                creature_factories,
                dungeon_generators,
                map_processors,
            )))
        });

        output.clone()
    }

    pub fn _get_theme(&self, name: String) -> Option<&Arc<Mutex<Theme>>> {
        self.values.get(&name)
    }

    pub fn make_theme_top_level(&mut self, name: &str) -> (bool, String, &'static str) {
        if self.values.contains_key(name) {
            if self.top_level_theme_names.contains(name) {
                return (true, String::from(name), "Already a top-level theme.");
            } else {
                self.top_level_theme_names.insert(String::from(name));
                return (
                    true,
                    String::from(name),
                    "Theme exists and is now a top-level theme.",
                );
            }
        }

        (false, String::from(name), "No such theme exists.")
    }

    pub fn get_random_top_level_theme(&self) -> Arc<Mutex<Theme>> {
        let index = thread_rng().gen_range(0, self.top_level_theme_names.len());
        for (i, name) in self.top_level_theme_names.iter().enumerate() {
            if i == index {
                if let Some(output) = self.values.get(name) {
                    return output.clone();
                } else {
                    break;
                }
            }
        }
        panic!("Theme count shrunk while we were observing it!");
    }
}
