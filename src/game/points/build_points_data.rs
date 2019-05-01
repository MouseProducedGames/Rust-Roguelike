/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

 **/
// External includes.
use specs::Entity;

// Standard includes.

// Internal includes.
use super::BuildPointsValue;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct _BuildPointsData {
    evaluate_entity: Entity,
    build_points_value: BuildPointsValue,
}

impl _BuildPointsData {
    pub fn _new(evaluate_entity: Entity, build_points_value: BuildPointsValue) -> Self {
        Self {
            evaluate_entity,
            build_points_value,
        }
    }

    pub fn _evaluate_entity(&self) -> Entity {
        self.evaluate_entity
    }

    pub fn _build_points_value(&self) -> BuildPointsValue {
        self.build_points_value
    }

    pub fn _build_points_value_mut(&mut self) -> &mut BuildPointsValue {
        &mut self.build_points_value
    }
}
