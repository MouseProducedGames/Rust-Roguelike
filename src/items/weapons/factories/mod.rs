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
mod hand_factory;
mod long_sword_factory;
mod template_weapon_factory;
mod weapon_factory;
pub use arming_sword_factory::ArmingSwordFactory;
pub use bastard_sword_factory::BastardSwordFactory;
pub use battle_axe_factory::BattleAxeFactory;
pub use hand_factory::HandFactory;
pub use long_sword_factory::LongSwordFactory;
pub use template_weapon_factory::TemplateWeaponFactory;
pub use weapon_factory::WeaponFactory;
