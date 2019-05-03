/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use enumflags2::BitFlags;
use specs::{Builder, World};

// Standard includes.
use std::convert::From;
use std::default::Default;
use std::sync::{Arc, Mutex};

// Internal includes.
use crate::abilities::{Ability, AbilityActivation, AbilityActivationOp, AbilityRange};
use crate::ai::systems::LogicPlayer;
use crate::ai::{Command, PlayerMarker, PlayerPosition, ViewpointMarker};
use crate::background::{OriginType, Species, SpeciesType};
use crate::bodies::{BodyFactory, BodySlotType};
use crate::data_types::Name;
use crate::factions::Faction;
use crate::game::combat::{AttackValue, DefenceValue, MultiAttackPenalty};
use crate::io::Display;
use crate::items::armours::factories::specific::wearables::{CuirassFactory, GauntletFactory};
use crate::items::armours::factories::traits::construction::{
    ChainArmourFactory, LeatherArmourFactory, PlateArmourFactory,
};
use crate::items::armours::factories::traits::quality::{FineArmourFactory, RustyArmourFactory};
use crate::items::armours::factories::ArmourFactory;
use crate::items::weapons::factories::specific::axes::BattleAxeFactory;
use crate::items::weapons::factories::specific::maces::RoundMaceFactory;
use crate::items::weapons::factories::specific::shields::RoundShieldFactory;
use crate::items::weapons::factories::specific::spears::{
    LongSpearFactory, MediumSpearFactory, ShortSpearFactory,
};
use crate::items::weapons::factories::specific::swords::{
    ArmingSwordFactory, BastardSwordFactory, LongSwordFactory, RapierFactory,
};
use crate::items::weapons::factories::traits::{
    FineShieldFactory, FineWeaponFactory, LargeShieldFactory, MediumShieldFactory,
    SmallShieldFactory,
};
use crate::items::weapons::factories::WeaponFactory;
use crate::items::weapons::WeaponGroup;
use crate::items::{Inventory, Item, LightSource, TransferItem, ITEM_ICON_INDEX_TORCH};
use crate::maps::VisibilityMapLookup;
use crate::rrl_math::Position;
use crate::screens::{Screen, ScreenPushWrapper, ScreenState};
use crate::skills::{
    SkillActivation, SkillLookup, SkillPassiveOp, SkillPoints, SkillTag, SkillType, SkillValue,
};
use crate::stats::CreatureStats;
use crate::talents::TalentLookup;

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
                    WeaponGroup::Axes,
                    SkillValue::from(0),
                    AttackValue::from(-1),
                    DefenceValue::from(1),
                ),
            );

            skills.insert(
                SkillActivation::Passive(SkillTag::Combat, SkillPassiveOp::OnUse),
                SkillType::Weapon(
                    WeaponGroup::Maces,
                    SkillValue::from(0),
                    AttackValue::from(1),
                    DefenceValue::from(-1),
                ),
            );

            skills.insert(
                SkillActivation::Passive(SkillTag::Combat, SkillPassiveOp::OnUse),
                SkillType::Weapon(
                    WeaponGroup::Shields,
                    SkillValue::from(0),
                    AttackValue::from(-1),
                    DefenceValue::from(1),
                ),
            );

            skills.insert(
                SkillActivation::Passive(SkillTag::Combat, SkillPassiveOp::OnUse),
                SkillType::Weapon(
                    WeaponGroup::Spears,
                    SkillValue::from(0),
                    AttackValue::from(1),
                    DefenceValue::from(-1),
                ),
            );

            skills.insert(
                SkillActivation::Passive(SkillTag::Combat, SkillPassiveOp::OnUse),
                SkillType::Weapon(
                    WeaponGroup::Swords,
                    SkillValue::from(0),
                    AttackValue::from(0),
                    DefenceValue::from(0),
                ),
            );

            skills.insert(
                SkillActivation::Passive(SkillTag::Combat, SkillPassiveOp::OnUse),
                SkillType::Weapon(
                    WeaponGroup::Unarmed,
                    SkillValue::from(0),
                    AttackValue::from(0),
                    DefenceValue::from(0),
                ),
            );

            let mut talents = TalentLookup::new();

            talents.insert(
                AbilityActivation::Passive(AbilityActivationOp::EveryRound),
                Ability::ScanForSecrets(-2, AbilityRange::Radius(1)),
            );

            let species = Species::create(species_type);

            let chain_cuirass_factory: ChainArmourFactory<CuirassFactory> = Default::default();
            let leather_gauntlet_factory: FineArmourFactory<LeatherArmourFactory<GauntletFactory>> =
                Default::default();
            let plate_cuirass_factory: RustyArmourFactory<PlateArmourFactory<CuirassFactory>> =
                Default::default();

            let battle_axe_factory: FineWeaponFactory<BattleAxeFactory> = Default::default();

            let round_mace_factory: FineWeaponFactory<RoundMaceFactory> = Default::default();

            let short_spear_factory: FineWeaponFactory<ShortSpearFactory> = Default::default();
            let medium_spear_factory: FineWeaponFactory<MediumSpearFactory> = Default::default();
            let long_spear_factory: FineWeaponFactory<LongSpearFactory> = Default::default();

            let large_round_shield_factory: FineShieldFactory<
                LargeShieldFactory<RoundShieldFactory>,
            > = Default::default();
            let medium_round_shield_factory: FineShieldFactory<
                MediumShieldFactory<RoundShieldFactory>,
            > = Default::default();
            let small_round_shield_factory: FineShieldFactory<
                SmallShieldFactory<RoundShieldFactory>,
            > = Default::default();

            let arming_sword_factory: FineWeaponFactory<ArmingSwordFactory> = Default::default();
            let bastard_sword_factory: FineWeaponFactory<BastardSwordFactory> = Default::default();
            let long_sword_factory: FineWeaponFactory<LongSwordFactory> = Default::default();
            let rapier_factory: FineWeaponFactory<RapierFactory> = Default::default();

            let torch = world
                .create_entity()
                .with(Item::new(
                    ITEM_ICON_INDEX_TORCH,
                    true,
                    BitFlags::empty(),
                    BodySlotType::Hand,
                ))
                .with(Name::new("Torch"))
                .with(LightSource::new(5.0))
                .build();

            let mut inventory = Inventory::new();
            inventory.push(plate_cuirass_factory.create_owned(world));
            inventory.push(battle_axe_factory.create_owned(world));
            inventory.push(round_mace_factory.create_owned(world));
            inventory.push(short_spear_factory.create_owned(world));
            inventory.push(medium_spear_factory.create_owned(world));
            inventory.push(long_spear_factory.create_owned(world));
            inventory.push(large_round_shield_factory.create_owned(world));
            inventory.push(small_round_shield_factory.create_owned(world));
            inventory.push(bastard_sword_factory.create_owned(world));
            inventory.push(long_sword_factory.create_owned(world));
            inventory.push(rapier_factory.create_owned(world));
            inventory.push(torch);
            let inventory = inventory;

            let body = species_type.create_body(world);
            body.get()
                .get_mut("Torso")
                .unwrap()
                .hold_item(chain_cuirass_factory.create_owned(world));
            body.get()
                .get_mut("Left Hand")
                .unwrap()
                .hold_item(leather_gauntlet_factory.create_owned(world));
            body.get()
                .get_mut("Left Palm")
                .unwrap()
                .hold_item(medium_round_shield_factory.create_owned(world));
            body.get()
                .get_mut("Right Hand")
                .unwrap()
                .hold_item(leather_gauntlet_factory.create_owned(world));
            body.get()
                .get_mut("Right Palm")
                .unwrap()
                .hold_item(arming_sword_factory.create_owned(world));

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
                        + CreatureStats::new(0, 0, 0, 0, 4, 0),
                )
                .with(MultiAttackPenalty::new(AttackValue::from(0)))
                .with(Position::new(8, 5))
                .with(PlayerMarker)
                .with(skills)
                .with(SkillPoints::from(120))
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
