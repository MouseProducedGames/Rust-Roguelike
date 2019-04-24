/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use specs::World;

// Standard includes.
use std::convert::From;
use std::default::Default;
use std::fmt;

// internal includes.
use crate::bodies::{Body, BodyFactory, BodySlot, BodySlotType};
use crate::io::{LongDescription, ShortDescription};
use crate::items::armours::factories::{ArmourFactory, TorsoFactory};
use crate::items::weapons::factories::{HandFactory, WeaponFactory};
use crate::stats::CreatureStats;

#[derive(Copy, Clone)]
pub enum SpeciesType {
    Dwarf,
    Elf,
    Halfling,
    Human,
}

impl SpeciesType {
    fn create_dwarf_body(world: &mut World) -> Body {
        let hand_factory = HandFactory::new();
        Body::new(&[
            BodySlot::new(
                "Torso",
                700,
                BodySlotType::Torso,
                TorsoFactory::new().create_owned(world),
            ),
            BodySlot::new(
                "Left Hand",
                105,
                BodySlotType::Hand,
                hand_factory.create_owned(world),
            ),
            BodySlot::new(
                "Right Hand",
                105,
                BodySlotType::Hand,
                hand_factory.create_owned(world),
            ),
        ])
    }

    fn create_elf_body(world: &mut World) -> Body {
        let hand_factory = HandFactory::new();
        Body::new(&[
            BodySlot::new(
                "Torso",
                800,
                BodySlotType::Torso,
                TorsoFactory::new().create_owned(world),
            ),
            BodySlot::new(
                "Left Hand",
                133,
                BodySlotType::Hand,
                hand_factory.create_owned(world),
            ),
            BodySlot::new(
                "Right Hand",
                133,
                BodySlotType::Hand,
                hand_factory.create_owned(world),
            ),
        ])
    }

    fn create_halfling_body(world: &mut World) -> Body {
        let hand_factory = HandFactory::new();
        Body::new(&[
            BodySlot::new(
                "Torso",
                500,
                BodySlotType::Torso,
                TorsoFactory::new().create_owned(world),
            ),
            BodySlot::new(
                "Left Hand",
                75,
                BodySlotType::Hand,
                hand_factory.create_owned(world),
            ),
            BodySlot::new(
                "Right Hand",
                75,
                BodySlotType::Hand,
                hand_factory.create_owned(world),
            ),
        ])
    }

    fn create_human_body(world: &mut World) -> Body {
        let hand_factory = HandFactory::new();
        Body::new(&[
            BodySlot::new(
                "Torso",
                1000,
                BodySlotType::Torso,
                TorsoFactory::new().create_owned(world),
            ),
            BodySlot::new(
                "Left Hand",
                150,
                BodySlotType::Hand,
                hand_factory.create_owned(world),
            ),
            BodySlot::new(
                "Right Hand",
                150,
                BodySlotType::Hand,
                hand_factory.create_owned(world),
            ),
        ])
    }
}

#[derive(Copy, Clone)]
pub struct Species {
    stats: CreatureStats,
}

impl Species {
    pub fn new(stats: CreatureStats) -> Self {
        Self { stats }
    }

    pub fn default() -> Self {
        Self::new(CreatureStats::default())
    }

    pub fn create(species: SpeciesType) -> Self {
        match species {
            SpeciesType::Dwarf => Self::create_dwarf(),
            SpeciesType::Elf => Self::create_elf(),
            SpeciesType::Halfling => Self::create_halfling(),
            SpeciesType::Human => Self::create_human(),
        }
    }

    pub fn create_dwarf() -> Self {
        let mut output = Self::default();
        output.stats += Self::create_dwarf_stat_changes();
        output
    }

    pub fn create_dwarf_stat_changes() -> CreatureStats {
        CreatureStats::new(0, -2, 0, 2, 0, 2)
    }

    pub fn create_elf() -> Self {
        let mut output = Self::default();
        output.stats += Self::create_elf_stat_changes();
        output
    }

    pub fn create_elf_stat_changes() -> CreatureStats {
        CreatureStats::new(-2, 2, 0, -2, 2, -2)
    }

    pub fn create_halfling() -> Self {
        let mut output = Self::default();
        output.stats += Self::create_halfling_stat_changes();
        output
    }

    pub fn create_halfling_stat_changes() -> CreatureStats {
        CreatureStats::new(-2, 0, 4, -2, 0, -2)
    }

    pub fn create_human() -> Self {
        let mut output = Self::default();
        output.stats += Self::create_human_stat_changes();
        output
    }

    pub fn create_human_stat_changes() -> CreatureStats {
        CreatureStats::new(0, 0, 0, 0, 0, 0)
    }

    pub fn stats(&self) -> CreatureStats {
        self.stats
    }

    pub fn _stats_mut(&mut self) -> &mut CreatureStats {
        &mut self.stats
    }
}

impl BodyFactory for SpeciesType {
    fn create_body(&self, world: &mut World) -> Body {
        match *self {
            SpeciesType::Dwarf => Self::create_dwarf_body(world),
            SpeciesType::Elf => Self::create_elf_body(world),
            SpeciesType::Halfling => Self::create_halfling_body(world),
            SpeciesType::Human => Self::create_human_body(world),
        }
    }
}

impl fmt::Display for SpeciesType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SpeciesType::Dwarf => "Dwarf",
                SpeciesType::Elf => "Elf",
                SpeciesType::Halfling => "Halfling",
                SpeciesType::Human => "Human",
            }
        )
    }
}

impl From<&SpeciesType> for LongDescription {
    fn from(item: &SpeciesType) -> LongDescription {
        LongDescription::from(*item)
    }
}

impl From<SpeciesType> for LongDescription {
    fn from(item: SpeciesType) -> LongDescription {
        match item {
            SpeciesType::Dwarf => LongDescription(concat!(
                "Dwarves stand an average of 4 feet, 8 inches tall",
                ", and are tough and hearty",
                ". Notable dwarves have lived late into their third century",
                ". Dwarves prefer the underground",
                ", and their cities are vast, carved stone halls",
                ", decorated with paint, precious metals, and gemstones",
                "."
            )),
            SpeciesType::Elf => LongDescription(concat!(
                "Elves stand an average of 5 feet tall",
                ", and are more slender on average than a human of their size",
                ". Elves are very long-lived; some having even",
                " reached the age of seven centuries",
                ". Although some imput",
                " magical talent",
                ", natural agility",
                ", or a connection to nature",
                " to elves",
                ", it is skill learned over their long lives",
                "."
            )),
            SpeciesType::Halfling => LongDescription(concat!(
                "Halflings resemble short humans, about three feet tall",
                ". Particular to this is that, rather than proportions",
                " proportionate to their height",
                ", halflings are build to the same scale as a human",
                ". This gives halflings exceptional strength",
                ", and endurance",
                " for their size",
                ", and exceptional body control",
                ".",
            )),
            SpeciesType::Human => LongDescription(concat!(
                "Humans thrive in larger groups than most others",
                ", being comfortable even around hundreds or thousands",
                " of strangers",
                ". Omnivorous",
                ", humans can consome a wide variety of animal and plant parts",
                ", and generally experience only mild discomfort from new",
                " or unusual",
                " foods",
                ". Humans have exceptional travel endurance",
                ", able to match the long-term pace of the more sturdy dwarves.",
            )),
        }
    }
}

impl From<&SpeciesType> for ShortDescription {
    fn from(item: &SpeciesType) -> ShortDescription {
        ShortDescription::from(*item)
    }
}

impl From<SpeciesType> for ShortDescription {
    fn from(item: SpeciesType) -> ShortDescription {
        match item {
            SpeciesType::Dwarf => {
                ShortDescription("Dwarves are short, tough, and long-lived humanoids.")
            }
            SpeciesType::Elf => ShortDescription("Elves are very long-lived, lithe humanoids."),
            SpeciesType::Halfling => {
                ShortDescription("Halflings, despite being 3' tall, ignore the square-cube law.")
            }
            SpeciesType::Human => {
                ShortDescription("Humans are highly social omnivorous pursuit predators.")
            }
        }
    }
}
