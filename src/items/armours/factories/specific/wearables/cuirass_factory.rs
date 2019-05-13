/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use specs::{Entity, World};

// Standard includes.
use std::default::Default;

// Internal includes.
use crate::bodies::BodySlotType;
use crate::data_types::Name;
use crate::game::combat::{DefenceValue, ProtectionValue};
use crate::items::armours::factories::{ArmourFactory, TemplateArmourFactory};
use crate::items::armours::{Armour, ArmourGroup};

#[derive(Clone)]
pub struct CuirassFactory(TemplateArmourFactory);

impl CuirassFactory {}

impl Default for CuirassFactory {
    fn default() -> Self {
        Self {
            0: TemplateArmourFactory::new(
                Name::new("Cuirass"),
                BodySlotType::Torso,
                Armour::new(
                    ArmourGroup::Default,
                    DefenceValue::from(0),
                    ProtectionValue::from(0),
                ),
            ),
        }
    }
}

impl ArmourFactory for CuirassFactory {
    fn create(&self, world: &mut World) -> Entity {
        self.0.create(world)
    }
}
