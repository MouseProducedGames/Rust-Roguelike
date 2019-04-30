/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use specs::{Builder, World};

// Standard includes.
use std::sync::{Arc, Mutex};

// Internal includes.
use super::{Screen, ScreenPushWrapper, ScreenState, WorldInitScreen};
use crate::ai::maslow::{MaslowInitScreen, MaslowTreeLookup};
use crate::ai::systems::LogicMaslow;
use crate::ai::Command;
use crate::background::SpeciesType;
use crate::bodies::BodyFactory;
use crate::creatures::CharacterCreationScreen;
use crate::factions::Faction;
use crate::game::combat::{AttackValue, DefenceValue, MultiAttackPenalty};
use crate::game::GameScreen;
use crate::io::DisplayInitScreen;
use crate::items::weapons::WeaponGroup;
use crate::items::Inventory;
use crate::maps::{MapInitScreen, VisibilityMapLookup};
use crate::rrl_math::Position;
use crate::skills::{SkillActivation, SkillLookup, SkillPassiveOp, SkillTag, SkillType};
use crate::stats::CreatureStats;
use crate::talents::TalentLookup;

enum StartState {
    SetupDisplay,
    InitializeAI,
    InitializeWorld,
    CharacterCreation,
    MapCreation,
    MonsterCreation,
    StartGame,
    Finished,
}

pub struct StartScreen {
    start_state: StartState,
    state: ScreenState,
}

impl StartScreen {
    pub fn new() -> Self {
        Self {
            start_state: StartState::SetupDisplay,
            state: ScreenState::Started,
        }
    }

    fn create_monsters(&mut self, world: &mut World) {
        let maslow_tree;
        {
            let maslow_tree_lookup = world
                .read_resource::<Arc<Mutex<MaslowTreeLookup>>>()
                .clone();
            let maslow_tree_lookup = maslow_tree_lookup.lock().unwrap();
            maslow_tree = maslow_tree_lookup.get("Faction/Wander").unwrap().clone();
        }

        let mut skills = SkillLookup::new();

        skills.insert(
            SkillActivation::Passive(SkillTag::Combat, SkillPassiveOp::OnUse),
            SkillType::Weapon(
                WeaponGroup::Unarmed,
                AttackValue::from(2),
                DefenceValue::from(2),
            ),
        );

        let body = SpeciesType::Human.create_body(world);

        world
            .create_entity()
            .with(body)
            .with(Command::None)
            .with(LogicMaslow)
            .with(Faction::new(0))
            .with(CreatureStats::default() + CreatureStats::new(4, 0, 0, 4, 4, 4))
            .with(Inventory::new())
            .with(maslow_tree)
            .with(MultiAttackPenalty::new(AttackValue::from(0)))
            .with(Position::new(12, 8))
            .with(skills)
            .with(TalentLookup::new())
            .with(VisibilityMapLookup::new())
            .build();
    }
}

impl Screen for StartScreen {
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
        self.start_state = match self.start_state {
            StartState::SetupDisplay => {
                let display_init_screen = Arc::new(Mutex::new(DisplayInitScreen::new()));
                screen_push_wrapper.push(display_init_screen);

                StartState::InitializeAI
            }
            StartState::InitializeAI => {
                let maslow_init_screen = Arc::new(Mutex::new(MaslowInitScreen::new()));
                screen_push_wrapper.push(maslow_init_screen);

                StartState::InitializeWorld
            }
            StartState::InitializeWorld => {
                let world_init_screen = Arc::new(Mutex::new(WorldInitScreen::new()));
                screen_push_wrapper.push(world_init_screen);

                StartState::CharacterCreation
            }
            StartState::CharacterCreation => {
                let character_creation_screen =
                    Arc::new(Mutex::new(CharacterCreationScreen::new()));
                screen_push_wrapper.push(character_creation_screen);

                StartState::MapCreation
            }
            StartState::MapCreation => {
                let map_init_screen = Arc::new(Mutex::new(MapInitScreen::new()));
                screen_push_wrapper.push(map_init_screen);

                StartState::MonsterCreation
            }
            StartState::MonsterCreation => {
                self.create_monsters(world);

                StartState::StartGame
            }
            StartState::StartGame => {
                let game_screen = Arc::new(Mutex::new(GameScreen::new()));
                screen_push_wrapper.push(game_screen);

                self.state = ScreenState::Stopped;

                StartState::Finished
            }
            StartState::Finished => {
                // This state is a placeholder that exists due to the
                // necessity of returning something from the StartGame
                // match. As such, we should never actually reach it.
                panic!("We should have exited before getting here!");
            }
        }
    }

    fn state(&self) -> ScreenState {
        self.state
    }
}
