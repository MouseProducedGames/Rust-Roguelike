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
use crate::ai::maslow::{faction_reaction, random_wander, MaslowNode, MaslowTree};
use crate::ai::systems::LogicMaslow;
use crate::ai::Command;
use crate::creatures::CreatureFactory;
use crate::dungen::{Catacombs, SplitDungeon, /* RandomlyTileDungeon, */ SplitType};
use crate::factions::Faction;
use crate::items::{Inventory, Item, LightSource};
use crate::rrl_math::{Bounds, Position};
use crate::skills::SkillLookup;
use crate::stats::CreatureStats;
use crate::talents::TalentLookup;
use crate::themes::ThemeLookup;
use crate::world::{
    match_pattern, MapProcessor, Mapping, PatternFlags, PatternLookup, Tilemap,
    VisibilityMapLookup, TILE_FUNC_INDEX_DOOR, TILE_FUNC_INDEX_SECRET_DOOR, TILE_FUNC_INDEX_VOID,
    TILE_TYPE_INDEX_DOOR, TILE_TYPE_INDEX_FLOOR, TILE_TYPE_INDEX_VOID, TILE_TYPE_INDEX_WALL,
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
        {
            let pattern_lookup = world.write_resource::<Arc<Mutex<PatternLookup>>>();
            let mut pattern_lookup = pattern_lookup.lock().unwrap();

            let mut crypt_niche = Tilemap::new(3, 3);

            *crypt_niche.tile_type_mut(crypt_niche.get_position(0, 0).unwrap()) =
                TILE_TYPE_INDEX_WALL;
            *crypt_niche.tile_type_mut(crypt_niche.get_position(1, 0).unwrap()) =
                TILE_TYPE_INDEX_WALL;
            *crypt_niche.tile_type_mut(crypt_niche.get_position(2, 0).unwrap()) =
                TILE_TYPE_INDEX_WALL;

            *crypt_niche.tile_type_mut(crypt_niche.get_position(0, 1).unwrap()) =
                TILE_TYPE_INDEX_WALL;
            *crypt_niche.tile_type_mut(crypt_niche.get_position(1, 1).unwrap()) =
                TILE_TYPE_INDEX_FLOOR;
            *crypt_niche.tile_type_mut(crypt_niche.get_position(2, 1).unwrap()) =
                TILE_TYPE_INDEX_WALL;

            *crypt_niche.tile_type_mut(crypt_niche.get_position(0, 2).unwrap()) =
                TILE_TYPE_INDEX_WALL;
            *crypt_niche.tile_type_mut(crypt_niche.get_position(1, 2).unwrap()) =
                TILE_TYPE_INDEX_DOOR;
            *crypt_niche.tile_type_mut(crypt_niche.get_position(2, 2).unwrap()) =
                TILE_TYPE_INDEX_WALL;

            pattern_lookup.insert("Crypt Niche", crypt_niche);
        }

        let theme_lookup = world.write_resource::<Arc<Mutex<ThemeLookup>>>();
        let mut theme_lookup = theme_lookup.lock().unwrap();

        let catacombs = theme_lookup.add_theme(
            String::from("Catacombs"),
            &[],
            &[],
            &[Arc::new(Mutex::new(Catacombs::new()))],
            &[],
        );

        let split_rooms = theme_lookup.add_theme(
            String::from("Split Rooms"),
            &[],
            &[],
            &[Arc::new(Mutex::new(SplitDungeon::new(
                SplitType::LongestDimension,
                Bounds {
                    width: 6,
                    height: 6,
                },
            )))],
            &[],
        );

        let creature_factory = Arc::new(Mutex::new(CreatureFactory::new(Arc::new(Mutex::new(
            |position: Position, world: &mut World| {
                if thread_rng().gen_range(1, 300) == 1 {
                    let faction_reaction_func = Arc::new(Mutex::new(faction_reaction));
                    let random_wander_func = Arc::new(Mutex::new(random_wander));

                    let faction_reaction_node = Arc::new(Mutex::new(MaslowNode::new(
                        &"Faction Reaction",
                        faction_reaction_func,
                        &[],
                    )));
                    let random_wander_node = Arc::new(Mutex::new(MaslowNode::new(
                        &"Faction Reaction",
                        random_wander_func,
                        &[],
                    )));

                    let maslow_tree = MaslowTree::new(
                        &"Faction/Wander Tree",
                        &[faction_reaction_node, random_wander_node],
                    );

                    world
                        .create_entity()
                        .with(Command::None)
                        .with(LogicMaslow)
                        .with(CreatureStats::default())
                        .with(Faction::new(1))
                        .with(Inventory::new())
                        .with(maslow_tree)
                        .with(position)
                        .with(SkillLookup::new())
                        .with(TalentLookup::new())
                        .with(VisibilityMapLookup::new())
                        .build();
                }
            },
        )))));

        let generic = theme_lookup.add_theme(
            String::from("Generic"),
            &[split_rooms.clone()],
            &[creature_factory.clone()],
            &[],
            &[Arc::new(Mutex::new(MapProcessor::new(Arc::new(
                Mutex::new(|meta_tile_map: &Tilemap, _world: &mut World| {
                    let mut output = Tilemap::new(meta_tile_map.width(), meta_tile_map.height());
                    for pos in meta_tile_map.get_position(0, 0) {
                        let (tile_type, tile_func_type) = match meta_tile_map.tile_type(pos) {
                            TILE_TYPE_INDEX_DOOR => {
                                let (mut tile_type, mut tile_func_type) =
                                    (TILE_TYPE_INDEX_DOOR, TILE_FUNC_INDEX_DOOR);
                                if thread_rng().gen_bool(0.1) {
                                    tile_type = 5;
                                    tile_func_type = TILE_FUNC_INDEX_SECRET_DOOR;
                                }

                                (tile_type, tile_func_type)
                            }
                            TILE_TYPE_INDEX_FLOOR => (TILE_TYPE_INDEX_FLOOR, TILE_FUNC_INDEX_VOID),
                            TILE_TYPE_INDEX_WALL => (TILE_TYPE_INDEX_WALL, TILE_FUNC_INDEX_VOID),
                            _ => (TILE_TYPE_INDEX_VOID, TILE_FUNC_INDEX_VOID),
                        };
                        *output.tile_type_mut(pos) = tile_type;
                        *output.tile_func_type_mut(pos) = tile_func_type;
                    }
                    output
                }),
            ))))],
        );

        let undead = theme_lookup.add_theme(
            String::from("Undead"),
            &[catacombs.clone()],
            &[creature_factory.clone()],
            &[],
            &[Arc::new(Mutex::new(MapProcessor::new(Arc::new(
                Mutex::new(|meta_tile_map: &Tilemap, world: &mut World| {
                    let pattern_lookup;
                    {
                        let ref_pattern_lookup =
                            world.write_resource::<Arc<Mutex<PatternLookup>>>();
                        pattern_lookup = ref_pattern_lookup.clone();
                    }
                    let pattern_lookup = pattern_lookup.lock().unwrap();
                    let crypt_niche = pattern_lookup.get("Crypt Niche");
                    let crypt_niche = crypt_niche.unwrap();

                    let mut output = Tilemap::new(meta_tile_map.width(), meta_tile_map.height());
                    for y in 0..meta_tile_map.height() {
                        for x in 0..meta_tile_map.width() {
                            let scan_pos = meta_tile_map.get_position(x, y);
                            let pos = scan_pos.unwrap();
                            let (tile_type, tile_func_type) = match meta_tile_map.tile_type(pos) {
                                TILE_TYPE_INDEX_DOOR => {
                                    if thread_rng().gen_bool(0.1) {
                                        if thread_rng().gen_bool(0.3) {
                                            (5, TILE_FUNC_INDEX_SECRET_DOOR)
                                        } else {
                                            (TILE_TYPE_INDEX_DOOR, TILE_FUNC_INDEX_DOOR)
                                        }
                                    } else {
                                        (TILE_TYPE_INDEX_FLOOR, TILE_FUNC_INDEX_VOID)
                                    }
                                }
                                TILE_TYPE_INDEX_FLOOR => {
                                    (TILE_TYPE_INDEX_FLOOR, TILE_FUNC_INDEX_VOID)
                                }
                                TILE_TYPE_INDEX_WALL => {
                                    (TILE_TYPE_INDEX_WALL, TILE_FUNC_INDEX_VOID)
                                }
                                _ => (TILE_TYPE_INDEX_VOID, TILE_FUNC_INDEX_VOID),
                            };
                            *output.tile_type_mut(pos) = tile_type;
                            *output.tile_func_type_mut(pos) = tile_func_type;

                            if thread_rng().gen_range(1, 300) == 1 {
                                let flags = PatternFlags::Rotate0
                                    | PatternFlags::Rotate90
                                    | PatternFlags::Rotate180
                                    | PatternFlags::Rotate270;
                                if match_pattern(meta_tile_map, scan_pos, crypt_niche, flags) {
                                    let creature_map_position =
                                        meta_tile_map.get_position(x + 1, y + 1).unwrap();
                                    let creature_position = Position::new(
                                        i32::from(creature_map_position.x()),
                                        i32::from(creature_map_position.y()),
                                    );

                                    let faction_reaction_func =
                                        Arc::new(Mutex::new(faction_reaction));
                                    let random_wander_func = Arc::new(Mutex::new(random_wander));

                                    let faction_reaction_node =
                                        Arc::new(Mutex::new(MaslowNode::new(
                                            &"Faction Reaction",
                                            faction_reaction_func,
                                            &[],
                                        )));
                                    let random_wander_node = Arc::new(Mutex::new(MaslowNode::new(
                                        &"Faction Reaction",
                                        random_wander_func,
                                        &[],
                                    )));

                                    let maslow_tree = MaslowTree::new(
                                        &"Faction/Wander Tree",
                                        &[faction_reaction_node, random_wander_node],
                                    );

                                    world
                                        .create_entity()
                                        .with(Command::None)
                                        .with(LogicMaslow)
                                        .with(CreatureStats::default())
                                        .with(Faction::new(1))
                                        .with(Inventory::new())
                                        .with(maslow_tree)
                                        .with(creature_position)
                                        .with(SkillLookup::new())
                                        .with(TalentLookup::new())
                                        .with(VisibilityMapLookup::new())
                                        .build();
                                }
                            }
                        }
                    }
                    output
                }),
            ))))],
        );

        // theme_lookup.make_theme_top_level(generic.lock().unwrap().name());
        theme_lookup.make_theme_top_level(undead.lock().unwrap().name());

        self.state = ScreenState::Stopped;
    }

    fn state(&self) -> ScreenState {
        self.state
    }
}
