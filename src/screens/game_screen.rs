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
use crate::abilities::AbilitySystem;
use crate::ai::{
    CommandSystem, LogicFactionSystem, LogicPlayerSystem, LogicWanderAttackSystem,
    LogicWanderSystem,
};
use crate::game::{GameState, LastUpdateSystem};
use crate::io::{CreatureDisplaySystem, Display, Input, PlayerDisplaySystem};
use crate::world::{Lightmap, VisibilitySystem};

pub struct GameScreen {
    ability_system: AbilitySystem,
    command_system: CommandSystem,
    creature_display_system: CreatureDisplaySystem,
    faction_logic: LogicFactionSystem,
    last_update_system: LastUpdateSystem,
    player_display_system: PlayerDisplaySystem,
    player_logic: LogicPlayerSystem,
    visibility_system: VisibilitySystem,
    wander_logic: LogicWanderSystem,
    wander_attack_logic: LogicWanderAttackSystem,
    state: ScreenState,
}

impl GameScreen {
    pub fn new() -> Self {
        Self {
            ability_system: AbilitySystem,
            command_system: CommandSystem,
            creature_display_system: CreatureDisplaySystem,
            faction_logic: LogicFactionSystem,
            last_update_system: LastUpdateSystem,
            player_display_system: PlayerDisplaySystem,
            player_logic: LogicPlayerSystem,
            visibility_system: VisibilitySystem,
            wander_logic: LogicWanderSystem,
            wander_attack_logic: LogicWanderAttackSystem,
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
        self.player_display_system.run_now(&world.res);

        self.creature_display_system.run_now(&world.res);

        world.maintain();

        {
            let mutex_display = world.write_resource::<Arc<Mutex<Display>>>();
            let mut display = mutex_display.lock().unwrap();
            display.present();
        }
    }
    
    fn pre_update(&mut self, world: &mut World) {
        let arc_mutex_display = world.read_resource::<Arc<Mutex<Display>>>();
        let display = arc_mutex_display.lock().unwrap();
        let arc_mutex_input = world.write_resource::<Arc<Mutex<Input>>>();
        let input = arc_mutex_input.lock().unwrap();
        display.update(input);
    }

    fn update(&mut self, world: &mut World, screen_push_wrapper: &mut ScreenPushWrapper) {
        self.player_logic.run_now(&world.res);

        world.maintain();

        self.wander_logic.run_now(&world.res);

        world.maintain();

        self.wander_attack_logic.run_now(&world.res);

        world.maintain();

        self.faction_logic.run_now(&world.res);

        world.maintain();

        self.command_system.run_now(&world.res);

        world.maintain();

        world.write_resource::<Lightmap>().clear();
        self.ability_system.run_now(&world.res);

        world.maintain();

        self.last_update_system.run_now(&world.res);

        world.maintain();

        self.state = if world.read_resource::<GameState>().alive() {
            ScreenState::Running
        } else {
            ScreenState::Stopped
        };

        {
            let mut game_state = world.write_resource::<GameState>();
            let mut new_screens = game_state.lock_new_screens();
            while new_screens.len() > 0 {
                if let Some(new_screen) = new_screens.pop() {
                    screen_push_wrapper.push(new_screen);
                } else {
                    panic!("New screens should not be added elsewhere while we have a lock!");
                }
            }
        }
    }
    
    fn post_update(&mut self, world: &mut World) {
        self.visibility_system.run_now(&world.res);

        world.maintain();
    }

    fn state(&self) -> ScreenState {
        self.state
    }
}
