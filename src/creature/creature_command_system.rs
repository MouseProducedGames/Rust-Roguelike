/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

 **/
// External dependencies.
use specs::{Entities, Entity, ReadExpect, ReadStorage, System, WriteExpect, WriteStorage};

// Internal dependencies.
use crate::creature::{Command, CreatureStats, CreatureTracker, Visibility};
use crate::factions::Faction;
use crate::game::{Combat, CombatResult};
use crate::rrl_math::{calculate_hash, Position};
use crate::stats::Stat;
use crate::world::{execute_tile_func, Tilemap, VisibilityType};

pub struct CreatureCommandSystem;

#[derive(SystemData)]
pub struct SystemDataT<'a> {
    creature_tracker: ReadExpect<'a, CreatureTracker>,
    entities: Entities<'a>,
    map: WriteExpect<'a, Tilemap>,
    command: ReadStorage<'a, Command>,
    factions: ReadStorage<'a, Faction>,
    visibility: ReadStorage<'a, Visibility>,
    stats: WriteStorage<'a, CreatureStats>,
    pos: WriteStorage<'a, Position>,
}

impl<'a> System<'a> for CreatureCommandSystem {
    type SystemData = SystemDataT<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        use specs::join::Join;

        let creature_tracker = &*data.creature_tracker;
        let factions = data.factions;
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

                    execute_tile_func(false, 100, &mut map, visibility_type, new_pos);
                }
                _ => (),
            }
        }
    }
}

fn impassable_movement<'a>(
    entity: Entity,
    new_pos: Position,
    pos: &mut Position,
    creature_tracker: &CreatureTracker,
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

            if let CombatResult::DefenderDead = Combat::one_attack(&attacker_stats, defender_stats)
            {
                (*defender_stats.health_mut().value_mut()).min(-100);
            }
        }
        None => *pos = new_pos,
    }
}
