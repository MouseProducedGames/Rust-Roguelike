/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use specs::World;

// Standard includes.
use std::sync::{Arc, Mutex};

// Internal includes.
use super::{Lightmap, MapProcessor, Mapping, PatternsInitScreen, Tilemap};
use crate::creatures::{CreatureFactoryWrapper, CreaturePlot};
use crate::dungen::{DungenCommon, DungeonGenerator};
use crate::rrl_math::Position;
use crate::screens::{Screen, ScreenPushWrapper, ScreenState};
use crate::themes::{ThemeHelper, ThemeInitScreen, ThemeLookup};

enum MapInitState {
    InitializingPatterns,
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
            map_init_state: MapInitState::InitializingPatterns,
        }
    }

    fn create_map(&self, world: &mut World) {
        let mut map;
        {
            let theme_lookup;
            {
                let ref_theme_lookup = world.write_resource::<Arc<Mutex<ThemeLookup>>>();
                theme_lookup = ref_theme_lookup.clone();
            }
            let theme_lookup = theme_lookup.lock().unwrap();
            let theme = theme_lookup.get_random_top_level_theme();
            let theme = theme.lock().unwrap();

            let mut generation_areas: Vec<(Position, Position)> = vec![];
            {
                let mut temp_map = Tilemap::new(25, 34);
                theme.get_random_dungeon_generator(&mut |_index: usize,
                                                         dungen: &Arc<
                    Mutex<dyn DungeonGenerator>,
                >| {
                    dungen
                        .lock()
                        .unwrap()
                        .apply(&mut temp_map, &mut generation_areas);
                });

                map = temp_map.finish();
            }

            {
                let mut creature_plots: Vec<CreaturePlot> = vec![];
                theme.get_random_map_processor(
                    &mut |_index: usize, dungen: &Arc<Mutex<MapProcessor>>| {
                        let result = dungen.lock().unwrap().gen_once(&map, world);
                        map = result.0;
                        creature_plots = result.1;
                    },
                );

                let creature_plots = creature_plots;
                for creature_plot in creature_plots {
                    if let Some(position) = creature_plot.check() {
                        theme.get_random_creature_factory(&mut |_index: usize,
                                                                creature_factory: &Arc<
                            Mutex<CreatureFactoryWrapper>,
                        >| {
                            creature_factory.lock().unwrap().gen_once(position, world);
                        })
                    }
                }
            }
        }

        world.add_resource(Lightmap::new(map.width(), map.height()));
        world.add_resource(map);

        {
            let map_pos = world.read_resource::<Tilemap>().get_position(8, 5).unwrap();
            *world.write_resource::<Tilemap>().tile_type_mut(map_pos) = 2;
        }
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

    fn update(&mut self, world: &mut World, screen_push_wrapper: &mut ScreenPushWrapper) {
        self.map_init_state = match self.map_init_state {
            MapInitState::InitializingPatterns => {
                let patterns_init_screen = Arc::new(Mutex::new(PatternsInitScreen::new()));
                screen_push_wrapper.push(patterns_init_screen);
                MapInitState::InitializingThemes
            }
            MapInitState::InitializingThemes => {
                let theme_init_screen = Arc::new(Mutex::new(ThemeInitScreen::new()));
                screen_push_wrapper.push(theme_init_screen);
                MapInitState::CreatingMap
            }
            MapInitState::CreatingMap => {
                self.create_map(world);
                self.state = ScreenState::Stopped;
                MapInitState::Finished
            }
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
