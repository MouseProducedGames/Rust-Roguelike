/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.

// Standard includes.

// Internal includes.
mod arming_sword_factory;
mod bastard_sword_factory;
mod battle_axe_factory;
mod fine_weapon_processor;
mod long_sword_factory;
mod palm_factory;
mod process_weapon_factory;
mod rusty_weapon_processor;
mod template_weapon_factory;
mod weapon_factory;
mod weapon_processor;
pub use arming_sword_factory::ArmingSwordFactory;
pub use bastard_sword_factory::BastardSwordFactory;
pub use battle_axe_factory::BattleAxeFactory;
pub use fine_weapon_processor::{FineWeaponFactory, FineWeaponProcessor};
pub use long_sword_factory::LongSwordFactory;
pub use palm_factory::PalmFactory;
pub use process_weapon_factory::ProcessWeaponFactory;
pub use rusty_weapon_processor::{RustyWeaponFactory, RustyWeaponProcessor};
pub use template_weapon_factory::TemplateWeaponFactory;
pub use weapon_factory::WeaponFactory;
pub use weapon_processor::WeaponProcessor;
