/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

 **/
// External includes
use rand::Rng;

// internal include
use super::super::GameState;
use crate::creature::CreatureStats;
use crate::stats::{Stat, StatModifier};

pub enum CombatResult {
    Miss,
    Hit,
    DefenderDead,
}

pub struct Combat;

impl Combat {
    pub fn one_attack(
        game_state: &mut GameState,
        attacker_stats: &CreatureStats,
        defender_stats: &mut CreatureStats,
    ) -> CombatResult {
        let mut result = CombatResult::Miss;

        let attack_mod = attacker_stats.coordination().modifier();
        let defence_mod = defender_stats.agility().modifier();

        let attack_roll = game_state.rng().gen_range(1, 7)
            + game_state.rng().gen_range(1, 7)
            + game_state.rng().gen_range(1, 7)
            + attack_mod;
        let defence_total = 10 + defence_mod;

        if attack_roll > defence_total {
            result = CombatResult::Hit;

            let damage_mod = 5 + defender_stats.strength().modifier();
            let new_defender_health = defender_stats.health() - damage_mod;
            *defender_stats.health_mut() = new_defender_health;

            if defender_stats.health().value() <= 0 {
                result = CombatResult::DefenderDead;
            }
        }

        result
    }
}
