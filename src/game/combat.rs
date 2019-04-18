/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

 **/
// External includes.
use specs::Entity;

// Standard includes.

// Internal includes.
use crate::dice::roll_success;
use crate::stats::{CreatureStats, Stat, StatModifier};

pub enum CombatResult {
    Miss,
    Hit,
    DefenderDead,
}

pub struct Combat;

impl Combat {
    pub fn one_attack(
        attacker_stats: &CreatureStats,
        defender_stats: &mut CreatureStats,
    ) -> CombatResult {
        let mut result = CombatResult::Miss;

        let attack_mod = i64::from(attacker_stats.coordination().modifier());
        let defence_mod = i64::from(defender_stats.agility().modifier());

        /* let attack_roll = game_state.rng().gen_range(1, 7)
        + game_state.rng().gen_range(1, 7)
        + game_state.rng().gen_range(1, 7)
        + attack_mod; */
        // let defence_total = 10 + defence_mod;

        if roll_success(attack_mod - defence_mod) {
            result = CombatResult::Hit;

            let damage_mod = 5 + attacker_stats.strength().modifier();
            let new_defender_health = defender_stats.health() - damage_mod;
            *defender_stats.health_mut() = new_defender_health;

            if defender_stats.health().value() <= 0 {
                result = CombatResult::DefenderDead;
            }
        }

        result
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AttackData {
    attacker: Entity,
    defender: Entity,
    attack_modifier: i32,
    defence_modifier: i32,
}

impl AttackData {
    pub fn new(
        attacker: Entity,
        defender: Entity,
        attack_modifier: i32,
        defence_modifier: i32,
    ) -> Self {
        Self {
            attacker,
            defender,
            attack_modifier,
            defence_modifier,
        }
    }

    pub fn attacker(&self) -> Entity {
        self.attacker
    }

    pub fn defender(&self) -> Entity {
        self.defender
    }

    pub fn attack_modifier(&self) -> i32 {
        self.attack_modifier
    }

    pub fn defence_modifier(&self) -> i32 {
        self.defence_modifier
    }

    pub fn attack_modifier_mut(&mut self) -> &mut i32 {
        &mut self.attack_modifier
    }

    pub fn defence_modifier_mut(&mut self) -> &mut i32 {
        &mut self.defence_modifier
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DamageData {
    attacker: Entity,
    defender: Entity,
    damage: i32,
}

impl DamageData {
    pub fn new(attacker: Entity, defender: Entity, damage: i32) -> Self {
        Self {
            attacker,
            defender,
            damage,
        }
    }

    pub fn attacker(&self) -> Entity {
        self.attacker
    }

    pub fn defender(&self) -> Entity {
        self.defender
    }

    pub fn damage(&self) -> i32 {
        self.damage
    }

    pub fn damage_mut(&mut self) -> &mut i32 {
        &mut self.damage
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct InjuryData {
    attacker: Entity,
    defender: Entity,
    pub injury: i32,
}

impl InjuryData {
    pub fn new(attacker: Entity, defender: Entity, injury: i32) -> Self {
        Self {
            attacker,
            defender,
            injury,
        }
    }

    pub fn attacker(&self) -> Entity {
        self.attacker
    }

    pub fn defender(&self) -> Entity {
        self.defender
    }

    pub fn injury(&self) -> i32 {
        self.injury
    }

    pub fn injury_mut(&mut self) -> &mut i32 {
        &mut self.injury
    }
}
