/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use specs::{Builder, World};

// Standard includes.
use std::convert::From;
use std::sync::{Arc, Mutex};

// Internal includes.
use super::{Screen, ScreenPushWrapper, ScreenState};
use crate::ai::{Command, LogicPlayer, PlayerMarker, PlayerPosition, ViewpointMarker, Visibility};
use crate::background::{OriginType, Species, SpeciesType};
use crate::factions::Faction;
use crate::io::Display;
use crate::rrl_math::Position;
use crate::skills::{SkillActivation, SkillLookup, SkillPassiveOp, SkillTag, SkillType};
use crate::stats::{CreatureStats, SightRange};
use crate::talents::{TalentActivation, TalentActivationOp, TalentLookup, TalentRange, TalentType};

pub struct CharacterCreationScreen {
    state: ScreenState,
}

impl CharacterCreationScreen {
    pub fn new() -> Self {
        Self {
            state: ScreenState::Started,
        }
    }
}

impl Screen for CharacterCreationScreen {
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
        world.add_resource(PlayerPosition(Position::new(8, 5)));

        let origin_type;
        {
            let mutex_display = world.write_resource::<Arc<Mutex<Display>>>();
            let mut display = mutex_display.lock().unwrap();
            origin_type = display.choose_origin(&[
                OriginType::Farmer,
                OriginType::Hunter,
                OriginType::Jack,
                OriginType::Rogue,
            ]);
        }

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
                .with(LogicPlayer {})
                .with(Faction::new(0))
                .with(species.stats() + CreatureStats::from(origin_type))
                .with(Position::new(8, 5))
                .with(PlayerMarker)
                .with(SightRange::new(5.0))
                .with(skills)
                .with(talents)
                .with(ViewpointMarker)
                .with(Visibility::new())
                .build();
        }

        self.state = ScreenState::Stopped;
    }

    fn state(&self) -> ScreenState {
        self.state
    }
}
