/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use specs::{Entity, World};

// Standard includes.
use std::default::Default;

// Internal includes.
use super::{ArmourFactory, TemplateArmourFactory};
use crate::bodies::BodySlotType;
use crate::data_types::Name;
use crate::game::combat::{DefenceValue, ProtectionValue};
use crate::items::armours::{Armour, ArmourGroup};

#[derive(Clone)]
pub struct GauntletFactory(TemplateArmourFactory);

impl GauntletFactory {}

impl Default for GauntletFactory {
    fn default() -> Self {
        Self {
            0: TemplateArmourFactory::new(
                Name::new("Gauntlet"),
                BodySlotType::Hand,
                Armour::new(
                    ArmourGroup::Default,
                    DefenceValue::from(0),
                    ProtectionValue::from(0),
                ),
            ),
        }
    }
}

impl ArmourFactory for GauntletFactory {
    fn create(&self, world: &mut World) -> Entity {
        self.0.create(world)
    }
}
