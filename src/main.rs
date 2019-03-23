/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

 **/
// External dependencies
extern crate rand;
use rand::rngs::ThreadRng;
use rand::Rng;

extern crate rust_dice;

extern crate shred;
#[macro_use]

extern crate shred_derive;
use specs::{Builder, RunNow, /* System, */ World};
use std::sync::{Arc, Mutex};

// Internal dependencies.
mod creature;
mod dice;
mod dungen;
mod faction;
mod game;
mod io;
mod multidim;
mod multimap;
mod rrl_math;
mod skills;
mod stats;
mod talents;
mod tiled_shapes_2d;
mod world;
use creature::background::{Species, SpeciesType};
use creature::{
    Command, CreatureAbilitySystem, CreatureCommandSystem, CreatureDisplaySystem,
    CreatureLastUpdateSystem, CreatureLogicFaction, CreatureLogicFactionSystem,
    CreatureLogicPlayer, CreatureLogicPlayerSystem, CreatureLogicWander, CreatureLogicWanderAttack,
    CreatureLogicWanderAttackSystem, CreatureLogicWanderSystem, CreatureStats, CreatureTracker,
    CreatureVisibilitySystem, PlayerDisplaySystem, PlayerMarker, PlayerPosition, SightRange,
    ViewpointMarker, Visibility,
};
use dungen::{DungeonGenerator, SplitDungeon, /* RandomlyTileDungeon, */ SplitType};
use faction::Faction;
use game::GameState;
use io::Display;
use rrl_math::{Bounds, Position};
use skills::{
    SkillActivation, SkillPassiveOp, SkillLookup, SkillTag, SkillType
};
use talents::{TalentActivation, TalentActivationOp, TalentLookup, TalentRange, TalentType};
use world::{Tilemap, TILE_FUNC_INDEX_DOOR, TILE_FUNC_INDEX_SECRET_DOOR};

fn main() {
    let display: Arc<Mutex<dyn Display>> = Arc::new(Mutex::new(io::console::ConsoleDisplay::new()));

    // Window::init();
    let mut game_state = GameState::new();

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
            |rnd: &mut ThreadRng| -> (u32, u32) {
                if rnd.gen_bool(0.9) {
                    (5, TILE_FUNC_INDEX_SECRET_DOOR)
                } else {
                    (3, TILE_FUNC_INDEX_DOOR)
                }
            },
            2,
            1,
            &mut game_state.rng(),
        )
        .apply(&mut temp_map);

        map = temp_map;
    }

    let mut world = World::new();
    world.add_resource(CreatureTracker::new());
    world.add_resource(game_state);
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
        let display = mutex_display.lock().unwrap();
        species_type = display.choose_species(&vec![
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
            .with(SightRange::new(5))
            .with(skills)
            .with(talents)
            .with(ViewpointMarker)
            .with(Visibility::new())
            .build();

        {
            *(&mut *world.write_resource::<Tilemap>()).tile_type_mut(8, 5) = 2;
        }
    }

    /* world
        .create_entity()
        .with(Command::None)
        .with(CreatureLogicFaction)
        .with(Faction::new(0))
        .with(CreatureStats::default())
        .with(Position::new(12, 8))
        .with(SightRange::new(5))
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
        .with(SightRange::new(5))
        .with(TalentLookup::new())
        .with(Visibility::new())
        .build(); */

    let mut creature_ability_system = CreatureAbilitySystem;
    let mut creature_command_system = CreatureCommandSystem;
    let mut creature_display_system = CreatureDisplaySystem;
    let mut creaature_last_update_system = CreatureLastUpdateSystem;
    let mut creature_faction_logic = CreatureLogicFactionSystem;
    let mut creature_player_logic = CreatureLogicPlayerSystem;
    let mut creature_wander_logic = CreatureLogicWanderSystem;
    let mut creature_wander_attack_logic = CreatureLogicWanderAttackSystem;
    let mut creature_visibility_system = CreatureVisibilitySystem;
    let mut player_display_system = PlayerDisplaySystem;

    while world.read_resource::<GameState>().alive() {
        creature_visibility_system.run_now(&world.res);

        player_display_system.run_now(&world.res);

        creature_display_system.run_now(&world.res);

        world.maintain();

        {
            let mutex_display = world.write_resource::<Arc<Mutex<Display>>>();
            let mut display = mutex_display.lock().unwrap();
            display.present();
        }

        creature_player_logic.run_now(&world.res);

        world.maintain();

        creature_wander_logic.run_now(&world.res);

        world.maintain();

        creature_wander_attack_logic.run_now(&world.res);

        world.maintain();

        creature_faction_logic.run_now(&world.res);

        world.maintain();

        creature_command_system.run_now(&world.res);

        world.maintain();

        creature_ability_system.run_now(&world.res);

        world.maintain();

        creaature_last_update_system.run_now(&world.res);

        world.maintain();
    }

    // Window::close();
}
