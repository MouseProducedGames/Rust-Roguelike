/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use rand::{thread_rng, Rng};
use specs::{Entity, World};

// Standard includes.
use std::sync::Arc;

// Internal includes.
use crate::data_types::Name;
use crate::game::combat::AttackValue;
use crate::game::points::BuildLevel;
use crate::game::GameValueFixed;
use crate::items::weapons::factories::{WeaponFactory, WeaponGenerator, WeaponProcessor};
use crate::items::weapons::Weapon;

#[derive(Clone)]
pub struct TemplatedLeveledWeaponGenerator {
    weapon_factories: Arc<Vec<Arc<dyn WeaponFactory>>>,
    quality_processors: Arc<Vec<Arc<dyn WeaponProcessor>>>,
    flaw_processors: Arc<Vec<Arc<dyn WeaponProcessor>>>,
}

impl TemplatedLeveledWeaponGenerator {
    pub fn new(
        weapon_factories: &[Arc<dyn WeaponFactory>],
        quality_processors: &[Arc<dyn WeaponProcessor>],
        flaw_processors: &[Arc<dyn WeaponProcessor>],
    ) -> Self {
        Self {
            weapon_factories: Arc::new(weapon_factories.to_vec()),
            quality_processors: Arc::new(quality_processors.to_vec()),
            flaw_processors: Arc::new(flaw_processors.to_vec()),
        }
    }
}

impl WeaponGenerator for TemplatedLeveledWeaponGenerator {
    fn create(&self, world: &mut World, level: BuildLevel) -> Entity {
        let level = level.raw();

        let item_entity;
        {
            let index = thread_rng().gen_range(0, self.weapon_factories.len());
            item_entity = self.weapon_factories[index].create(world);
        }

        if level > 0 {
            let mut level = level;
            let mut quality_indices: Vec<usize> = (0..self.quality_processors.len()).collect();
            while level > 0 && quality_indices.is_empty() == false {
                let quality_indices_index = thread_rng().gen_range(0, quality_indices.len());
                let quality_index = quality_indices[quality_indices_index];
                self.quality_processors[quality_index].process(world, item_entity);

                quality_indices.remove(quality_indices_index);
                level -= GameValueFixed::from_int(1);
            }

            if level > 0 {
                {
                    let mut weapon_storage = world.write_storage::<Weapon>();
                    let weapon = weapon_storage.get_mut(item_entity).unwrap();
                    *weapon.attack_value_mut() += AttackValue::new(level);
                }

                {
                    let mut name_storage = world.write_storage::<Name>();
                    let name = name_storage.get_mut(item_entity).unwrap();
                    name.insert_str(0, " ");
                    name.insert_str(0, &level.to_string());
                    name.insert_str(0, "+");
                }
            }
        } else if level < 0 {
            let mut level = level;
            let mut flaw_indices: Vec<usize> = (0..self.flaw_processors.len()).collect();
            while level < 0 && flaw_indices.is_empty() == false {
                let flaw_indices_index = thread_rng().gen_range(0, flaw_indices.len());
                let flaw_index = flaw_indices[flaw_indices_index];
                self.flaw_processors[flaw_index].process(world, item_entity);

                flaw_indices.remove(flaw_indices_index);
                level += GameValueFixed::from_int(1);
            }

            if level < 0 {
                {
                    let mut weapon_storage = world.write_storage::<Weapon>();
                    let weapon = weapon_storage.get_mut(item_entity).unwrap();
                    *weapon.attack_value_mut() += AttackValue::new(level);
                }

                {
                    let mut name_storage = world.write_storage::<Name>();
                    let name = name_storage.get_mut(item_entity).unwrap();
                    name.insert_str(0, " ");
                    name.insert_str(0, &level.to_string());
                }
            }
        }

        item_entity
    }
}
