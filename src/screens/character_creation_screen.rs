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
use crate::abilities::{Ability, AbilityActivation, AbilityActivationOp, AbilityRange};
use crate::ai::systems::LogicPlayer;
use crate::ai::{Command, PlayerMarker, PlayerPosition, ViewpointMarker};
use crate::background::{OriginType, Species, SpeciesType};
use crate::bodies::{Body, BodySlot, BodySlotType};
use crate::data_types::Name;
use crate::factions::Faction;
use crate::game::combat::{AttackValue, DefenceValue};
use crate::io::Display;
use crate::items::weapons::factories::{
    ArmingSwordFactory, BastardSwordFactory, BattleAxeFactory, HandFactory, LongSwordFactory,
    WeaponFactory,
};
use crate::items::weapons::WeaponGroup;
use crate::items::{Inventory, Item, LightSource, TransferItem, ITEM_ICON_INDEX_TORCH};
use crate::rrl_math::Position;
use crate::skills::{SkillActivation, SkillLookup, SkillPassiveOp, SkillTag, SkillType};
use crate::stats::CreatureStats;
use crate::talents::TalentLookup;
use crate::world::VisibilityMapLookup;

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

            skills.insert(
                SkillActivation::Passive(SkillTag::Combat, SkillPassiveOp::OnUse),
                SkillType::Weapon(
                    WeaponGroup::Unarmed,
                    AttackValue::from(2),
                    DefenceValue::from(2),
                ),
            );

            skills.insert(
                SkillActivation::Passive(SkillTag::Combat, SkillPassiveOp::OnUse),
                SkillType::Weapon(
                    WeaponGroup::Swords,
                    AttackValue::from(2),
                    DefenceValue::from(2),
                ),
            );

            let mut talents = TalentLookup::new();

            talents.insert(
                AbilityActivation::Passive(AbilityActivationOp::EveryRound),
                Ability::ScanForSecrets(-2, AbilityRange::Radius(1)),
            );

            let species = Species::create(species_type);

            let hand_factory = HandFactory::new();

            let arming_sword_factory = ArmingSwordFactory::new();

            let bastard_sword_factory = BastardSwordFactory::new();

            let battle_axe_factory = BattleAxeFactory::new();

            let long_sword_factory = LongSwordFactory::new();

            let torch = world
                .create_entity()
                .with(Item::new(ITEM_ICON_INDEX_TORCH, true, BodySlotType::Hand))
                .with(Name::new("Torch"))
                .with(LightSource::new(5.0))
                .build();

            let mut inventory = Inventory::new();
            inventory.push(arming_sword_factory.create_owned(world));
            inventory.push(bastard_sword_factory.create_owned(world));
            inventory.push(battle_axe_factory.create_owned(world));
            inventory.push(long_sword_factory.create_owned(world));
            let inventory = inventory;

            let body = Body::new(&[
                BodySlot::with_held_item(
                    "Left Hand",
                    BodySlotType::Hand,
                    hand_factory.create_owned(world),
                    torch,
                ),
                BodySlot::new(
                    "Right Hand",
                    BodySlotType::Hand,
                    hand_factory.create_owned(world),
                ),
            ]);

            world
                .create_entity()
                .with(body)
                .with(Command::None)
                .with(LogicPlayer {})
                .with(Faction::new(0))
                .with(inventory)
                .with(
                    species.stats()
                        + CreatureStats::from(origin_type)
                        + CreatureStats::new(0, 0, 0, 4, 4, 4),
                )
                .with(Position::new(8, 5))
                .with(PlayerMarker)
                .with(skills)
                .with(talents)
                .with(TransferItem::None)
                .with(ViewpointMarker)
                .with(VisibilityMapLookup::new())
                .build();
        }

        self.state = ScreenState::Stopped;
    }

    fn state(&self) -> ScreenState {
        self.state
    }
}
