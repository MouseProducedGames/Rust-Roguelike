/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use specs::World;

// Standard includes.
use std::sync::{Arc, Mutex};

// Internal includes.
use super::{Screen, ScreenPushWrapper, ScreenState};
use crate::abilities::{Undead, UndeadEventHandler};
use crate::ai::maslow::MaslowTree;
use crate::ai::systems::{LogicMaslow, LogicPlayer};
use crate::ai::{Command, PlayerMarker, ViewpointMarker};
use crate::bodies::Body;
use crate::data_types::Name;
use crate::events::EventManager;
use crate::factions::Faction;
use crate::game::combat::{AttackPenaltyEventHandler, MultiAttackPenalty};
use crate::game::points::{BuildPoints, CurrencyValue};
use crate::game::{EntityPositionTracker, GameState, Time};
use crate::items::armours::{Armour, ArmourEventHandler};
use crate::items::weapons::{Weapon, WeaponEventHandler};
use crate::items::{Inventory, Item, LightSource, TransferItem};
use crate::maps::{PatternLookup, VisibilityMapLookup};
use crate::rrl_math::Position;
use crate::skills::{SkillEventHandler, SkillLookup, SkillPoints};
use crate::stats::{CreatureStats, StatEventHandler};
use crate::talents::TalentLookup;
use crate::themes::ThemeLookup;

pub struct WorldInitScreen {
    state: ScreenState,
}

impl WorldInitScreen {
    pub fn new() -> Self {
        Self {
            state: ScreenState::Started,
        }
    }
}

impl Screen for WorldInitScreen {
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
        world.add_resource(EntityPositionTracker::new());
        world.add_resource(Arc::new(Mutex::new(EventManager::new())));
        {
            let event_manager = world.write_resource::<Arc<Mutex<EventManager>>>().clone();
            let mut event_manager = event_manager.lock().unwrap();
            world.add_resource(AttackPenaltyEventHandler::new(&mut event_manager));
            world.add_resource(WeaponEventHandler::new(&mut event_manager));
            world.add_resource(SkillEventHandler::new(&mut event_manager));
            world.add_resource(StatEventHandler::new(&mut event_manager));
            world.add_resource(ArmourEventHandler::new(&mut event_manager));
            world.add_resource(UndeadEventHandler::new(&mut event_manager));
        }
        world.add_resource(GameState::new());
        world.add_resource(Arc::new(Mutex::new(PatternLookup::new())));
        world.add_resource(Arc::new(Mutex::new(ThemeLookup::new())));
        world.add_resource(Time::new(0));
        world.register::<Armour>();
        world.register::<Body>();
        world.register::<BuildPoints>();
        world.register::<Command>();
        world.register::<CreatureStats>();
        world.register::<CurrencyValue>();
        world.register::<Faction>();
        world.register::<Inventory>();
        world.register::<Item>();
        world.register::<LightSource>();
        world.register::<LogicMaslow>();
        world.register::<LogicPlayer>();
        world.register::<MaslowTree>();
        world.register::<MultiAttackPenalty>();
        world.register::<Name>();
        world.register::<PlayerMarker>();
        world.register::<Position>();
        world.register::<SkillLookup>();
        world.register::<SkillPoints>();
        world.register::<TalentLookup>();
        world.register::<TransferItem>();
        world.register::<Undead>();
        world.register::<ViewpointMarker>();
        world.register::<VisibilityMapLookup>();
        world.register::<Weapon>();

        self.state = ScreenState::Stopped;
    }

    fn state(&self) -> ScreenState {
        self.state
    }
}
