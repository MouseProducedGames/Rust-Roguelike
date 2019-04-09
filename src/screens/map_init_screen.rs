/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use rand::{thread_rng, Rng};
use specs::World;

// Standard includes.
use std::sync::{Arc, Mutex, MutexGuard};

// Internal includes.
use super::{Screen, ScreenPushWrapper, ScreenState};
use crate::dungen::{
    DungenCommon, DungeonGenerator, SplitDungeon, /* RandomlyTileDungeon, */ SplitType,
};
use crate::rrl_math::Bounds;
use crate::themes::ThemeLookup;
use crate::world::{Lightmap, Mapping, Tilemap, TILE_FUNC_INDEX_DOOR, TILE_FUNC_INDEX_SECRET_DOOR};

enum MapInitState {
    InitializingThemes,
    CreatingMap,
    Finished,
}

pub struct MapInitScreen {
    state: ScreenState,
    map_init_state: MapInitState,
}

impl MapInitScreen {
    pub fn new() -> Self {
        Self {
            state: ScreenState::Started,
            map_init_state: MapInitState::InitializingThemes,
        }
    }
    
    fn create_map(&self, world: &mut World) {
        
        let map;
        {
            let theme_lookup = world.write_resource::<Arc<Mutex<ThemeLookup>>>();
            let theme_lookup = theme_lookup.lock().unwrap();
            let theme = theme_lookup.get_random_top_level_theme();
            
            let dungen_index = thread_rng().gen_range(0, theme.dungeon_generator_count());
            let mut temp_map = Tilemap::new(40, 30);
            let mut call = 
                |
                    index: usize,
                    dungen: &Arc<Mutex<(dyn DungeonGenerator)>>,
                | {
                    if index == dungen_index {
                        dungen.lock().unwrap().apply(&mut temp_map);
                    }
                };
            theme.for_all_dungeon_generators(&mut call);

            map = temp_map.finish();
        }

        world.add_resource(Lightmap::new(map.width(), map.height()));
        world.add_resource(map);

        {
            let map_pos = world.read_resource::<Tilemap>().get_position(8, 5).unwrap();
            *world.write_resource::<Tilemap>().tile_type_mut(map_pos) = 2;
        }
    }
    
    fn initialize_themes(&self, world: &mut World) {
        let theme_lookup = world.write_resource::<Arc<Mutex<ThemeLookup>>>();
        let mut theme_lookup = theme_lookup.lock().unwrap();
        
        theme_lookup.add_theme(
            String::from("Split Rooms"),
            &[],
            &[Arc::new(Mutex::new(SplitDungeon::new(
                SplitType::LongestDimension,
                Bounds {
                    width: 6,
                    height: 6,
                },
                || -> (u32, u32) {
                    if thread_rng().gen_bool(0.1) {
                        (5, TILE_FUNC_INDEX_SECRET_DOOR)
                    } else {
                        (3, TILE_FUNC_INDEX_DOOR)
                    }
                },
                2,
                1,
            )))],
        );

        if let Some(split_rooms) = theme_lookup.get_theme(String::from("Split Rooms"))
        {
            let split_rooms = split_rooms.clone();
            theme_lookup.add_theme(
                String::from("Generic"),
                &[split_rooms],
                &[],
            );
        }
        
        theme_lookup.make_theme_top_level(String::from("Generic"));
    }
}

impl Screen for MapInitScreen {
    fn init(&mut self) {
        self.state = match self.state {
            ScreenState::Inactive => ScreenState::Inactive,
            ScreenState::Started => ScreenState::Running,
            ScreenState::Running => ScreenState::Running,
            ScreenState::Stopped => ScreenState::Inactive,
        }
    }

    fn close(&mut self) {
        self.state = match self.state {
            ScreenState::Inactive => ScreenState::Inactive,
            ScreenState::Started => ScreenState::Inactive,
            ScreenState::Running => ScreenState::Inactive,
            ScreenState::Stopped => ScreenState::Inactive,
        }
    }

    fn blocks_draw(&self) -> bool {
        true
    }

    fn blocks_update(&self) -> bool {
        true
    }

    fn draw(&mut self, _world: &mut World) {}

    fn update(&mut self, world: &mut World, _screen_push_wrapper: &mut ScreenPushWrapper) {
        self.map_init_state =
            match self.map_init_state {
                MapInitState::InitializingThemes => {
                    self.initialize_themes(world);
                    MapInitState::CreatingMap
                },
                MapInitState::CreatingMap => {
                    self.create_map(world);
                    self.state = ScreenState::Stopped;
                    MapInitState::Finished
                },
                MapInitState::Finished => {
                // This state is a placeholder that exists due to the
                // necessity of returning something from the MapInitState
                // match. As such, we should never actually reach it.
                panic!("We should have exited before getting here!");
                }
            };
    }

    fn state(&self) -> ScreenState {
        self.state
    }
}
