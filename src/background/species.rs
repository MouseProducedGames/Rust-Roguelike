/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.

// Standard includes.
use std::default::Default;

// internal includes.
use crate::stats::CreatureStats;

#[derive(Copy, Clone)]
pub enum SpeciesType {
    Dwarf,
    Elf,
    Halfling,
    Human,
}

impl SpeciesType {
    pub fn to_long_description_str(self) -> &'static str {
        match self {
            SpeciesType::Dwarf => concat!(
                "Dwarves stand an average of 4 feet, 8 inches tall",
                ", and are tough and hearty",
                ". Notable dwarves have lived late into their third century",
                ". Dwarves prefer the underground",
                ", and their cities are vast, carved stone halls",
                ", decorated with paint, precious metals, and gemstones",
                "."
            ),
            SpeciesType::Elf => concat!(
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
            ),
            SpeciesType::Halfling => concat!(
                "Halflings resemble short humans, about three feet tall",
                ". Particular to this is that, rather than proportions",
                " proportionate to their height",
                ", halflings are build to the same scale as a human",
                ". This gives halflings exceptional strength",
                ", and endurance",
                " for their size",
                ", and exceptional body control",
                ".",
            ),
            SpeciesType::Human => concat!(
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
            ),
        }
    }

    pub fn to_short_description_str(self) -> &'static str {
        match self {
            SpeciesType::Dwarf => "Dwarves are short, tough, and long-lived humanoids.",
            SpeciesType::Elf => "Elves are very long-lived, lithe humanoids.",
            SpeciesType::Halfling => {
                "Halflings, despite being 3' tall, ignore the square-cube law."
            }
            SpeciesType::Human => "Humans are highly social omnivorous pursuit predators.",
        }
    }

    pub fn to_str(self) -> &'static str {
        match self {
            SpeciesType::Dwarf => "Dwarf",
            SpeciesType::Elf => "Elf",
            SpeciesType::Halfling => "Halfling",
            SpeciesType::Human => "Human",
        }
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
