/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes
use rand::{thread_rng, Rng};
use specs::World;

// Internal includes
use super::screen::ScreenState;
use super::screen_manager::ScreenPushWrapper;
use super::Screen;
use crate::dungen::{DungeonGenerator, SplitDungeon, /* RandomlyTileDungeon, */ SplitType};
use crate::rrl_math::Bounds;
use crate::world::{Lightmap, Mapping, Tilemap, TILE_FUNC_INDEX_DOOR, TILE_FUNC_INDEX_SECRET_DOOR};

pub struct MapInitScreen {
    state: ScreenState,
}

impl MapInitScreen {
    pub fn new() -> Self {
        Self {
            state: ScreenState::Started,
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

    fn update(&mut self, world: &mut World, _screen_push_wrapper: &mut ScreenPushWrapper) {
        let map;
        {
            let mut temp_map: Tilemap = Tilemap::new(40, 30);
            //     let mut boxed_map: Box<dyn TiledArea> = Box::new( temp_map );
            SplitDungeon::new(
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
            )
            .apply(&mut temp_map);

            map = temp_map;
        }

        world.add_resource(Lightmap::new(map.width(), map.height()));
        world.add_resource(map);

        {
            let map_pos = world.read_resource::<Tilemap>().get_position(8, 5).unwrap();
            *world.write_resource::<Tilemap>().tile_type_mut(map_pos) = 2;
        }

        self.state = ScreenState::Stopped;
    }

    fn state(&self) -> ScreenState {
        self.state
    }
}
