/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use specs::{Component, Entity, World};

// Standard includes.
use std::sync::{Arc, Mutex};

// Internal includes.
use super::{
    SkillActivation, SkillLookup, SkillPassiveOp, SkillPoints, SkillTag, SkillType, SkillValue,
    WeaponSkillTypeLookup,
};
use crate::game::points::{BuildLevel, BuildPoints, HasBuildLevel};
use crate::game::{GameState, GameValueFixed};
use crate::io::{Display, Input};
use crate::screens::{Screen, ScreenPushWrapper, ScreenState};

pub struct SkillScreen {
    creature: Entity,
    state: ScreenState,
    skill_index: usize,
}

impl SkillScreen {
    pub fn new(creature: Entity) -> Self {
        Self {
            creature,
            state: ScreenState::Started,
            skill_index: 0,
        }
    }

    fn _get_storage_item<T: Clone + Component>(&self, world: &mut World) -> T {
        let mut items = world.write_storage::<T>();
        let item_option = items.get_mut(self.creature);;

        item_option.unwrap().clone()
    }
}

impl Screen for SkillScreen {
    fn init(&mut self) {
        self.state = match self.state {
            ScreenState::Inactive => ScreenState::Inactive,
            ScreenState::Started => ScreenState::Running,
            ScreenState::Running => ScreenState::Running,
            ScreenState::Stopped => ScreenState::Inactive,
        }
    }

    fn close(&mut self) {
        self.state = match self.state {
            ScreenState::Inactive => ScreenState::Inactive,
            ScreenState::Started => ScreenState::Inactive,
            ScreenState::Running => ScreenState::Inactive,
            ScreenState::Stopped => ScreenState::Inactive,
        }
    }

    fn blocks_draw(&self) -> bool {
        true
    }

    fn blocks_update(&self) -> bool {
        false
    }

    fn draw(&mut self, world: &mut World) {
        let skill_lookup_storage = world.read_storage::<SkillLookup>();
        let skill_lookup = skill_lookup_storage.get(self.creature).unwrap();

        let skill_points_storage = world.read_storage::<SkillPoints>();
        let skill_points = *skill_points_storage.get(self.creature).unwrap();

        let mutex_display = world.write_resource::<Arc<Mutex<Display>>>();
        let mut display = mutex_display.lock().unwrap();
        display.blit_skills(world, skill_lookup, skill_points, self.skill_index);
    }

    fn update(&mut self, world: &mut World, _screen_push_wrapper: &mut ScreenPushWrapper) {
        if world.read_resource::<GameState>().alive() == false {
            self.state = ScreenState::Stopped;
            return;
        }

        let mut skill_lookup_storage = world.write_storage::<SkillLookup>();
        let skill_lookup = skill_lookup_storage.get_mut(self.creature).unwrap();
        let skills = skill_lookup.get_or_add_set_mut(SkillActivation::Passive(
            SkillTag::Combat,
            SkillPassiveOp::OnUse,
        ));
        let skills_len = skills.len();

        let mut skill_points_storage = world.write_storage::<SkillPoints>();
        let skill_points = skill_points_storage.get_mut(self.creature).unwrap();

        let arc_mutex_input = world.read_resource::<Arc<Mutex<Input>>>().clone();
        let mut input = arc_mutex_input.lock().unwrap();
        let ch = input.instance_mut().consume_char();
        match ch {
            '8' => {
                if skills_len > 0 {
                    self.skill_index = self.skill_index.wrapping_sub(1);

                    if self.skill_index >= skills_len {
                        self.skill_index = skills_len - 1;
                    }
                }
            }
            '2' => {
                if skills_len > 0 {
                    self.skill_index = self.skill_index.wrapping_add(1);

                    if self.skill_index >= skills_len {
                        self.skill_index = 0;
                    }
                }
            }
            '+' => {
                if skills_len > 0 {
                    if let SkillType::Weapon(weapon_group, skill_value) =
                        &mut skills[self.skill_index]
                    {
                        let skill_level = skill_value.build_level_total(world);
                        let weapon_skill_type_lookup =
                            world.read_resource::<Arc<WeaponSkillTypeLookup>>();
                        let skill_cost_modifier =
                            weapon_skill_type_lookup.get(*weapon_group).cost_modifier();
                        let skill_cost_level = skill_level + skill_cost_modifier;
                        let bigger_numbers_skill_cost_level =
                            BuildLevel::new(skill_cost_level.raw() + GameValueFixed::from_int(10));
                        let current_skill_cost =
                            BuildPoints::from(bigger_numbers_skill_cost_level);
                        let next_skill_cost_level =
                            bigger_numbers_skill_cost_level + BuildLevel::from(1);
                        let next_skill_cost = BuildPoints::from(next_skill_cost_level);
                        let build_points_difference = next_skill_cost - current_skill_cost;
                        let skill_points_difference = SkillPoints::from(build_points_difference);

                        if skill_points_difference <= *skill_points {
                            *skill_points -= skill_points_difference;
                            *skill_value += SkillValue::from(1);
                        }
                    }
                }
            }
            '\r' => {
                self.state = ScreenState::Stopped;
                return;
            }
            _ => (),
        }
    }

    fn state(&self) -> ScreenState {
        self.state
    }
}
