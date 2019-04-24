/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use specs::{Entity, World};

// Standard includes.

// Internal includes.
use super::{ArmourFactory, TemplateArmourFactory};
use crate::bodies::BodySlotType;
use crate::data_types::Name;
use crate::game::combat::{DefenceValue, ProtectionValue};
use crate::items::armours::{Armour, ArmourGroup};

#[derive(Clone)]
pub struct ChainGauntletFactory(TemplateArmourFactory);

impl ChainGauntletFactory {
    pub fn new() -> Self {
        Self {
            0: TemplateArmourFactory::new(
                Name::new("Chain Gauntlet"),
                BodySlotType::Hand,
                Armour::new(
                    ArmourGroup::Default,
                    DefenceValue::from(0),
                    ProtectionValue::from(3),
                ),
            ),
        }
    }
}

impl ArmourFactory for ChainGauntletFactory {
    fn create(&self, world: &mut World) -> Entity {
        self.0.create(world)
    }
}
