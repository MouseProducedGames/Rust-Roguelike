/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.

// Standard includes.
use std::convert::From;

// internal includes.
use crate::stats::CreatureStats;

#[derive(Copy, Clone)]
pub enum OriginType {
    Farmer,
    Hunter,
    Jack,
    Rogue,
}

impl OriginType {
    pub fn to_string(self) -> String {
        match self {
            OriginType::Farmer => String::from("Farmer"),
            OriginType::Hunter => String::from("Hunter"),
            OriginType::Jack => String::from("jack"),
            OriginType::Rogue => String::from("Rogue"),
        }
    }
    
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
