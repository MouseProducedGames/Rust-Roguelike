// Linting
//
// "if x == false" is clear and easily understandable.
// "if !x" is only superficially good.
// Consider "if !list_item".
// Consider also the rate of spelling errors in code.
#![cfg_attr(feature = "cargo-clippy", allow(clippy::bool_comparison))]

/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

 **/
// External dependencies
#[macro_use]
extern crate derive_more;
extern crate rand;
use rand::{thread_rng, Rng};

extern crate rust_dice;

extern crate shred;
#[macro_use]

extern crate shred_derive;
use specs::{Builder, World};
use std::sync::{Arc, Mutex};

// Internal dependencies.
mod abilities;
mod ai;
mod background;
mod data_types;
mod dice;
mod dungen;
mod factions;
mod game;
mod io;
mod items;
mod multidim;
mod multimap;
mod rrl_math;
mod screens;
mod skills;
mod stats;
mod talents;
mod tiled_shapes_2d;
mod world;
use ai::{
    Command, CreatureLogicFaction, CreatureLogicPlayer, CreatureLogicWander,
    CreatureLogicWanderAttack, CreatureTracker, PlayerMarker, PlayerPosition,
    ViewpointMarker, Visibility,
};
use background::{Species, SpeciesType};
use dungen::{DungeonGenerator, SplitDungeon, /* RandomlyTileDungeon, */ SplitType};
use factions::Faction;
use game::GameState;
use io::Display;
use rrl_math::{Bounds, Position};
use screens::{GameScreen, ScreenManager};
use skills::{SkillActivation, SkillLookup, SkillPassiveOp, SkillTag, SkillType};
use stats::{CreatureStats, SightRange};
use talents::{TalentActivation, TalentActivationOp, TalentLookup, TalentRange, TalentType};
use world::{Lightmap, Mapping, Tilemap, TILE_FUNC_INDEX_DOOR, TILE_FUNC_INDEX_SECRET_DOOR};

fn main() {
    let display: Arc<Mutex<dyn Display>> = Arc::new(Mutex::new(io::console::ConsoleDisplay::new()));

    // Window::init();

    let mut screen_manager = ScreenManager::new();

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

    let mut world = World::new();
    world.add_resource(CreatureTracker::new());
    world.add_resource(GameState::new());
    world.add_resource(Lightmap::new(map.width(), map.height()));
    world.add_resource(map);
    world.add_resource(display);
    world.add_resource(PlayerPosition(Position::new(8, 5)));
    world.register::<Command>();
    world.register::<CreatureLogicFaction>();
    world.register::<CreatureLogicPlayer>();
    world.register::<CreatureLogicWander>();
    world.register::<CreatureLogicWanderAttack>();
    world.register::<CreatureStats>();
    world.register::<Faction>();
    world.register::<PlayerMarker>();
    world.register::<Position>();
    world.register::<SightRange>();
    world.register::<SkillLookup>();
    world.register::<TalentLookup>();
    world.register::<ViewpointMarker>();
    world.register::<Visibility>();

    let species_type;
    {
        let mutex_display = world.write_resource::<Arc<Mutex<Display>>>();
        let mut display = mutex_display.lock().unwrap();
        species_type = display.choose_species(&[
            SpeciesType::Dwarf,
            SpeciesType::Elf,
            SpeciesType::Halfling,
            SpeciesType::Human,
        ]);
    }

    {
        let mut skills = SkillLookup::new();

        skills.insert(
            SkillActivation::Passive(SkillTag::Perception, SkillPassiveOp::EveryRound),
            SkillType::Skill(2),
        );

        let mut talents = TalentLookup::new();

        talents.insert(
            TalentActivation::Passive(TalentActivationOp::EveryRound),
            TalentType::ScanForSecrets(-2, TalentRange::Radius(1)),
        );

        let species = Species::create(species_type);
        world
            .create_entity()
            .with(Command::None)
            .with(CreatureLogicPlayer {})
            .with(Faction::new(0))
            .with(species.stats())
            .with(Position::new(8, 5))
            .with(PlayerMarker)
            .with(SightRange::new(5.0))
            .with(skills)
            .with(talents)
            .with(ViewpointMarker)
            .with(Visibility::new())
            .build();

        {
            let map_pos = world.read_resource::<Tilemap>().get_position(8, 5).unwrap();
            *world.write_resource::<Tilemap>().tile_type_mut(map_pos) = 2;
        }
    }

    world
        .create_entity()
        .with(Command::None)
        .with(CreatureLogicFaction)
        .with(Faction::new(0))
        .with(CreatureStats::default())
        .with(Position::new(12, 8))
        .with(SightRange::new(5.0))
        .with(TalentLookup::new())
        .with(Visibility::new())
        .build();

    world
        .create_entity()
        .with(Command::None)
        .with(CreatureLogicFaction)
        .with(Faction::new(1))
        .with(CreatureStats::default())
        .with(Position::new(8, 12))
        .with(SightRange::new(5.0))
        .with(TalentLookup::new())
        .with(Visibility::new())
        .build();

    let game_screen = Arc::new(Mutex::new(GameScreen::new()));
    
    screen_manager.push(game_screen);

    while screen_manager.screen_count() > 0 {
        {
            screen_manager.update_start();
            screen_manager.update(&mut world);
            screen_manager.draw(&mut world);
        }
    }

    // Window::close();
}
