/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use rand::{thread_rng, Rng};
use specs::{Builder, World};

// Standard includes.
use std::sync::{Arc, Mutex, MutexGuard};

// Internal includes.
use super::{Screen, ScreenPushWrapper, ScreenState};
use crate::ai::{Command, LogicFaction};
use crate::creatures::CreatureFactory;
use crate::dungen::{SplitDungeon, /* RandomlyTileDungeon, */ SplitType,};
use crate::factions::Faction;
use crate::items::Inventory;
use crate::rrl_math::{Bounds, Position};
use crate::stats::CreatureStats;
use crate::talents::TalentLookup;
use crate::themes::ThemeLookup;
use crate::world::{
    TILE_FUNC_INDEX_DOOR, TILE_FUNC_INDEX_SECRET_DOOR,
    VisibilityMapLookup,
};

pub struct ThemeInitScreen {
    state: ScreenState,
}

impl ThemeInitScreen {
    pub fn new() -> Self {
        Self {
            state: ScreenState::Started,
        }
    }
}

impl Screen for ThemeInitScreen {
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
        let theme_lookup = world.write_resource::<Arc<Mutex<ThemeLookup>>>();
        let mut theme_lookup = theme_lookup.lock().unwrap();
        
        theme_lookup.add_theme(
            String::from("Split Rooms"),
            &[],
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
            let creature_factory =
                Arc::new(Mutex::new(CreatureFactory::new(
                    Arc::new(Mutex::new(|position: Position, world: &mut World| {
                        // panic!("This should happen, actually.");
                        if thread_rng().gen_range(1, 300) == 1 {
                            world
                                .create_entity()
                                .with(Command::None)
                                .with(CreatureStats::default())
                                .with(Faction::new(1))
                                .with(Inventory::new())
                                .with(LogicFaction)
                                .with(position)
                                .with(TalentLookup::new())
                                .with(VisibilityMapLookup::new())
                                .build();
                        }
                    }))
                )));
                
            let split_rooms = split_rooms.clone();
            theme_lookup.add_theme(
                String::from("Generic"),
                &[split_rooms],
                &[creature_factory],
                &[],
            );
        }
        
        theme_lookup.make_theme_top_level(String::from("Generic"));
        
        self.state = ScreenState::Stopped;
    }

    fn state(&self) -> ScreenState {
        self.state
    }
}
