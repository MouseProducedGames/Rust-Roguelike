/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use specs::{RunNow, World};

// Standard includes.
use std::sync::{Arc, Mutex};

// Internal includes.
use super::{Screen, ScreenPushWrapper, ScreenState};
use crate::ai::{
    CreatureAbilitySystem, CreatureCommandSystem, CreatureDisplaySystem, CreatureLastUpdateSystem,
    CreatureLogicFactionSystem, CreatureLogicPlayerSystem, CreatureLogicWanderAttackSystem,
    CreatureLogicWanderSystem, CreatureVisibilitySystem, PlayerDisplaySystem,
};
use crate::game::GameState;
use crate::io::Display;
use crate::world::Lightmap;

pub struct GameScreen {
    creature_ability_system: CreatureAbilitySystem,
    creature_command_system: CreatureCommandSystem,
    creature_display_system: CreatureDisplaySystem,
    creaature_last_update_system: CreatureLastUpdateSystem,
    creature_faction_logic: CreatureLogicFactionSystem,
    creature_player_logic: CreatureLogicPlayerSystem,
    creature_wander_logic: CreatureLogicWanderSystem,
    creature_wander_attack_logic: CreatureLogicWanderAttackSystem,
    creature_visibility_system: CreatureVisibilitySystem,
    player_display_system: PlayerDisplaySystem,
    state: ScreenState,
}

impl GameScreen {
    pub fn new() -> Self {
        Self {
            creature_ability_system: CreatureAbilitySystem,
            creature_command_system: CreatureCommandSystem,
            creature_display_system: CreatureDisplaySystem,
            creaature_last_update_system: CreatureLastUpdateSystem,
            creature_faction_logic: CreatureLogicFactionSystem,
            creature_player_logic: CreatureLogicPlayerSystem,
            creature_wander_logic: CreatureLogicWanderSystem,
            creature_wander_attack_logic: CreatureLogicWanderAttackSystem,
            creature_visibility_system: CreatureVisibilitySystem,
            player_display_system: PlayerDisplaySystem,
            state: ScreenState::Started,
        }
    }
}

impl Screen for GameScreen {
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
        true
    }

    fn draw(&mut self, world: &mut World) {
        self.creature_visibility_system.run_now(&world.res);

        self.player_display_system.run_now(&world.res);

        self.creature_display_system.run_now(&world.res);

        world.maintain();

        {
            let mutex_display = world.write_resource::<Arc<Mutex<Display>>>();
            let mut display = mutex_display.lock().unwrap();
            display.present();
        }
    }

    fn update(&mut self, world: &mut World, _screen_push_wrapper: &mut ScreenPushWrapper) {
        self.creature_player_logic.run_now(&world.res);

        world.maintain();

        self.creature_wander_logic.run_now(&world.res);

        world.maintain();

        self.creature_wander_attack_logic.run_now(&world.res);

        world.maintain();

        self.creature_faction_logic.run_now(&world.res);

        world.maintain();

        self.creature_command_system.run_now(&world.res);

        world.maintain();

        world.write_resource::<Lightmap>().clear();
        self.creature_ability_system.run_now(&world.res);

        world.maintain();

        self.creaature_last_update_system.run_now(&world.res);

        world.maintain();

        self.state = if world.read_resource::<GameState>().alive() {
            ScreenState::Running
        } else {
            ScreenState::Stopped
        }
    }

    fn state(&self) -> ScreenState {
        self.state
    }
}
