/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use specs::{Entity, World};

// Standard includes.
use std::default::Default;
use std::sync::Arc;

// Internal includes.
use super::{RoundShieldFactory, TemplatedLeveledShieldGenerator};
use crate::game::points::BuildLevel;
use crate::items::weapons::factories::flaws::DentedShieldProcessor;
use crate::items::weapons::factories::qualities::FineShieldProcessor;
use crate::items::weapons::factories::traits::{
    LargeShieldFactory, MediumShieldFactory, SmallShieldFactory,
};
use crate::items::weapons::factories::{GenerateWeaponGenerator, WeaponGenerator};

pub type LeveledShieldGenerator = GenerateWeaponGenerator<LeveledShieldGeneratorImpl>;

#[derive(Clone)]
pub struct LeveledShieldGeneratorImpl(TemplatedLeveledShieldGenerator);

impl LeveledShieldGeneratorImpl {
    fn generator(&self) -> &TemplatedLeveledShieldGenerator {
        &self.0
    }
}

impl Default for LeveledShieldGeneratorImpl {
    fn default() -> Self {
        Self {
            0: TemplatedLeveledShieldGenerator::new(
                &[
                    Arc::new(LargeShieldFactory::<RoundShieldFactory>::default()),
                    Arc::new(MediumShieldFactory::<RoundShieldFactory>::default()),
                    Arc::new(SmallShieldFactory::<RoundShieldFactory>::default()),
                ],
                &[Arc::new(FineShieldProcessor::default())],
                &[Arc::new(DentedShieldProcessor::default())],
            ),
        }
    }
}

impl WeaponGenerator for LeveledShieldGeneratorImpl {
    fn create(&self, world: &mut World, level: BuildLevel) -> Entity {
        self.generator().create(world, level)
    }
}
