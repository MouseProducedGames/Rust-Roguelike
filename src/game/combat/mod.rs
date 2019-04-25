/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.

// Standard includes.

// Internal includes.
mod attack_data;
mod attack_penalty_event_handler;
mod attack_value;
mod damage_data;
mod damage_value;
mod defence_value;
mod injury_data;
mod injury_value;
mod multi_attack_penalty;
mod protection_data;
mod protection_value;
pub use attack_data::AttackData;
pub use attack_penalty_event_handler::AttackPenaltyEventHandler;
pub use attack_value::{AttackMarker, AttackValue};
pub use damage_data::DamageData;
pub use damage_value::{DamageMarker, DamageValue};
pub use defence_value::{DefenceMarker, DefenceValue};
pub use injury_data::InjuryData;
pub use injury_value::{InjuryMarker, InjuryValue};
pub use multi_attack_penalty::MultiAttackPenalty;
pub use protection_data::ProtectionData;
pub use protection_value::{ProtectionMarker, ProtectionValue};
