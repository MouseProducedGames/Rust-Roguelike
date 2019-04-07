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
    pub fn to_string(self) -> String {
        match self {
            SpeciesType::Dwarf => String::from("Dwarf"),
            SpeciesType::Elf => String::from("Elf"),
            SpeciesType::Halfling => String::from("Halfling"),
            SpeciesType::Human => String::from("Human"),
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
