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
use super::SpearFactory;
use crate::game::points::BuildLevel;
use crate::items::weapons::factories::flaws::RustyWeaponProcessor;
use crate::items::weapons::factories::qualities::{DamagingWeaponProcessor, FineWeaponProcessor};
use crate::items::weapons::factories::{
    GenerateWeaponGenerator, TemplatedLeveledWeaponGenerator, WeaponGenerator,
};

pub type LeveledSpearGenerator = GenerateWeaponGenerator<LeveledSpearGeneratorImpl>;

#[derive(Clone)]
pub struct LeveledSpearGeneratorImpl(TemplatedLeveledWeaponGenerator);

impl LeveledSpearGeneratorImpl {
    fn generator(&self) -> &TemplatedLeveledWeaponGenerator {
        &self.0
    }
}

impl Default for LeveledSpearGeneratorImpl {
    fn default() -> Self {
        Self {
            0: TemplatedLeveledWeaponGenerator::new(
                &[Arc::new(SpearFactory::default())],
                &[
                    Arc::new(DamagingWeaponProcessor::default()),
                    Arc::new(FineWeaponProcessor::default()),
                ],
                &[Arc::new(RustyWeaponProcessor::default())],
            ),
        }
    }
}

impl WeaponGenerator for LeveledSpearGeneratorImpl {
    fn create(&self, world: &mut World, level: BuildLevel) -> Entity {
        self.generator().create(world, level)
    }
}
