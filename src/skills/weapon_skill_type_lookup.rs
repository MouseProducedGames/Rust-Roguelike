/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

 **/
// External includes.

// Standard includes.
use std::collections::HashMap;

// Internal includes.
use super::WeaponSkillTypeData;
use crate::items::weapons::WeaponGroup;

pub struct WeaponSkillTypeLookup {
    values: HashMap<WeaponGroup, WeaponSkillTypeData>,
}

impl WeaponSkillTypeLookup {
    pub fn new(initialization_data: &[(WeaponGroup, WeaponSkillTypeData)]) -> Self {
        let mut output = Self {
            values: HashMap::new(),
        };

        for (key, value) in initialization_data {
            if output.values.contains_key(key) {
                panic!("Key inserted twice!");
            }

            output.values.insert(*key, *value);
        }

        output
    }

    pub fn get(&self, key: WeaponGroup) -> WeaponSkillTypeData {
        self.values[&key]
    }
}
