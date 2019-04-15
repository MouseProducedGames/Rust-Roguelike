/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.

// Standard includes.
use std::convert::From;
use std::fmt;

// internal includes.
use crate::io::{LongDescription, ShortDescription};
use crate::stats::CreatureStats;

#[derive(Copy, Clone)]
pub enum OriginType {
    Farmer,
    Hunter,
    Jack,
    Rogue,
}

impl OriginType {
    fn create_stats_farmer() -> CreatureStats {
        CreatureStats::new(2, 0, 0, 2, 0, 2)
    }

    fn create_stats_hunter() -> CreatureStats {
        CreatureStats::new(0, 0, 0, 2, 2, 2)
    }

    fn create_stats_jack() -> CreatureStats {
        CreatureStats::new(1, 1, 1, 1, 1, 1)
    }

    fn create_stats_rogue() -> CreatureStats {
        CreatureStats::new(0, 0, 2, 0, 2, 0)
    }
}

impl From<OriginType> for CreatureStats {
    fn from(origin: OriginType) -> Self {
        match origin {
            OriginType::Farmer => OriginType::create_stats_farmer(),
            OriginType::Hunter => OriginType::create_stats_hunter(),
            OriginType::Jack => OriginType::create_stats_jack(),
            OriginType::Rogue => OriginType::create_stats_rogue(),
        }
    }
}

impl fmt::Display for OriginType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                OriginType::Farmer => "Farmer",
                OriginType::Hunter => "Hunter",
                OriginType::Jack => "Jack",
                OriginType::Rogue => "Rogue",
            }
        )
    }
}

impl From<&OriginType> for LongDescription {
    fn from(item: &OriginType) -> LongDescription {
        LongDescription::from(*item)
    }
}

impl From<OriginType> for LongDescription {
    fn from(item: OriginType) -> LongDescription {
        match item {
            OriginType::Farmer => LongDescription(concat!("Placeholder", ".",)),
            OriginType::Hunter => LongDescription(concat!("Placeholder", ".",)),
            OriginType::Jack => LongDescription(concat!("Placeholder", ".",)),
            OriginType::Rogue => LongDescription(concat!("Placeholder", ".",)),
        }
    }
}

impl From<&OriginType> for ShortDescription {
    fn from(item: &OriginType) -> ShortDescription {
        ShortDescription::from(*item)
    }
}

impl From<OriginType> for ShortDescription {
    fn from(item: OriginType) -> ShortDescription {
        match item {
            OriginType::Farmer => {
                ShortDescription("Work hard spring, summer, and fall. Freeze in the winter.")
            }
            OriginType::Hunter => {
                ShortDescription("Beats working, as long as you can catch something.")
            }
            OriginType::Jack => {
                ShortDescription("You do odd jobs. Some of the jobs have been very odd.")
            }
            OriginType::Rogue => ShortDescription("You do stuff. And things. For money."),
        }
    }
}
