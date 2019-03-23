/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

 **/
// External dependencies.
use rand::Rng;

extern crate rust_dice;
use rust_dice::{Die, Roll, RollSet};

use specs::{Entities, Entity, ReadExpect, ReadStorage, System, WriteExpect, WriteStorage};

// Internal dependencies.
use super::super::GameState;
use crate::creature::{Command, CreatureStats, CreatureTracker, Visibility};
use crate::faction::Faction;
use crate::game::{Combat, CombatResult};
use crate::rrl_math::{calculate_hash, Displacement, Position};
use crate::skills::{
    SkillActivation, SkillActiveOp, SkillPassiveOp, SkillLookup, SkillRange, SkillTag, SkillType
};
use crate::stats::Stat;
use crate::talents::{TalentActivation, TalentActivationOp, TalentLookup, TalentRange, TalentType};
use crate::world::{TileFunc, TileFuncOp, Tilemap, VisibilityType};

pub struct CreatureCommandSystem;

#[derive(SystemData)]
pub struct SystemDataT<'a> {
    creature_tracker: ReadExpect<'a, CreatureTracker>,
    entities: Entities<'a>,
    map: WriteExpect<'a, Tilemap>,
    game_state: WriteExpect<'a, GameState>,
    command: ReadStorage<'a, Command>,
    factions: ReadStorage<'a, Faction>,
    visibility: ReadStorage<'a, Visibility>,
    stats: WriteStorage<'a, CreatureStats>,
    pos: WriteStorage<'a, Position>,
    skills: WriteStorage<'a, SkillLookup>,
    talents: WriteStorage<'a, TalentLookup>,
}

impl<'a> System<'a> for CreatureCommandSystem {
    type SystemData = SystemDataT<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        use specs::join::Join;

        let creature_tracker = &*data.creature_tracker;
        let factions = data.factions;
        let mut game_state = data.game_state;
        let mut map = data.map;
        let map_hash = calculate_hash(&*map);
        let stats = &mut data.stats;

        for (entity, command, pos, visibility) in (
            &data.entities,
            &data.command,
            &mut data.pos,
            &data.visibility,
        )
            .join()
        {
            let command = command;

            match *command {
                Command::Move(disp) => {
                    let new_pos = *pos + disp;

                    if map.passable_pos(new_pos) {
                        impassable_movement(
                            entity,
                            new_pos,
                            pos,
                            creature_tracker,
                            &mut game_state,
                            &factions,
                            stats,
                        );
                    }

                    let visibility_type;
                    if let Some(visibility_map) = visibility.visibility_lookup().get(&map_hash) {
                        visibility_type = visibility_map.value_pos(new_pos);
                    } else {
                        visibility_type = VisibilityType::None;
                    }

                    execute_tile_func(
                        false,
                        100,
                        &mut game_state,
                        &mut map,
                        visibility_type,
                        new_pos,
                    );
                }
                _ => (),
            }
        }

        for (pos, skills, talent, visibility) in
            (&data.pos, &mut data.skills, &mut data.talents, &data.visibility).join() {
                
                let maybe_visibility_map = visibility.visibility_lookup().get(&map_hash);

                for talent_type in
                    talent.get_set(TalentActivation::Passive(TalentActivationOp::EveryRound))
                {
                    match talent_type {
                        TalentType::ScanForSecrets(talent_bonus, talent_range) => {
                            let set = skills.get_set(
                                SkillActivation::Passive(SkillTag::Perception, SkillPassiveOp::EveryRound)
                            );

                                let mut skill_total = *talent_bonus as i64;
                                for skill in set
                                {
                                    match skill {
                                        SkillType::Skill(v) => skill_total += *v as i64,
                                        _ => (),
                                    };
                                }
                                
                                match talent_range {
                                    TalentRange::Radius(radius) => {
                                        let iradius = *radius as i32;
                                        for yd in -iradius..=iradius {
                                            for xd in -iradius..=iradius {
                                                let scan_pos = *pos + Displacement::new(xd, yd);

                                                let visibility_type;
                                                if let Some(visibility_map) = maybe_visibility_map {
                                                    visibility_type = visibility_map.value_pos(scan_pos);
                                                } else {
                                                    visibility_type = VisibilityType::None;
                                                }

                                                execute_tile_func(
                                                    true,
                                                    skill_total,
                                                    &mut game_state,
                                                    &mut map,
                                                    visibility_type,
                                                    scan_pos,
                                                );
                                            }
                                        }
                                    },
                                }
                            
                        },
                    }
                }
            }
    }
}

fn impassable_movement<'a>(
    entity: Entity,
    new_pos: Position,
    pos: &mut Position,
    creature_tracker: &CreatureTracker,
    game_state: &mut WriteExpect<'a, GameState>,
    factions: &ReadStorage<'a, Faction>,
    stats: &mut WriteStorage<'a, CreatureStats>,
) {
    match creature_tracker.check_collision(entity, new_pos) {
        Some(other_entity) => {
            if let Some(faction_a) = factions.get(entity) {
                if let Some(faction_b) = factions.get(other_entity) {
                    if faction_a == faction_b {
                        return;
                    }
                }
            }

            let attacker_stats;
            let defender_stats;
            match stats.get(entity) {
                Some(stats) => attacker_stats = *stats,
                _ => return,
            }
            match stats.get_mut(other_entity) {
                Some(stats) => defender_stats = stats,
                _ => return,
            }

            if let CombatResult::DefenderDead =
                Combat::one_attack(&mut *game_state, &attacker_stats, defender_stats)
            {
                (*defender_stats.health_mut().value_mut()).min(-100);
            }
        }
        None => *pos = new_pos,
    }
}

fn execute_tile_func<'a>(
    harmless: bool,
    skill_total: i64,
    game_state: &mut WriteExpect<'a, GameState>,
    map: &mut WriteExpect<'a, Tilemap>,
    visibility_type: VisibilityType,
    pos: Position,
) {
    let mut success_roll = RollSet::new(3, Die::new(6), 0);
    
    match map.tile_func_pos(pos) {
        TileFunc::None => (),
        TileFunc::OnEnterTile(tile_func_op) => match tile_func_op {
            TileFuncOp::ChangeTileType(new_tile_type, new_tile_func_type) => {
                if harmless == false && success_roll.roll().total() <= skill_total {
                    *map.tile_type_mut(pos.x as u32, pos.y as u32) = new_tile_type;
                    *map.tile_func_type_mut(pos.x as u32, pos.y as u32) = new_tile_func_type;
                }
            }
            TileFuncOp::DiscoverTileType(new_tile_type, new_tile_func_type) => {
                if visibility_type == VisibilityType::Visible &&
                    success_roll.roll().total() <= skill_total
                {
                    *map.tile_type_mut(pos.x as u32, pos.y as u32) = new_tile_type;
                    *map.tile_func_type_mut(pos.x as u32, pos.y as u32) = new_tile_func_type;
                }
            }
        },
    }
}
