/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

 **/
// External includes.
use specs::{Component, Entities, NullStorage, ReadExpect, ReadStorage, System, WriteStorage};

// Standard includes.

// Internal includes.
use super::Command;
use crate::dice::get_random_move;
use crate::factions::Faction;
use crate::game::EntityPositionTracker;
use crate::rrl_math::{calculate_hash, Displacement, Position};
use crate::world::{Tilemap, Visibility};

#[derive(Default)]
pub struct LogicFaction;

impl Component for LogicFaction {
    type Storage = NullStorage<Self>;
}

pub struct LogicFactionSystem;

#[derive(SystemData)]
pub struct SystemDataT<'a> {
    entity_position_tracker: ReadExpect<'a, EntityPositionTracker>,
    entities: Entities<'a>,
    map: ReadExpect<'a, Tilemap>,
    visibility: ReadStorage<'a, Visibility>,
    factions: ReadStorage<'a, Faction>,
    logic: ReadStorage<'a, LogicFaction>,
    commands: WriteStorage<'a, Command>,
    pos: WriteStorage<'a, Position>,
}

impl<'a> System<'a> for LogicFactionSystem {
    type SystemData = SystemDataT<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        use specs::join::Join;

        let entity_position_tracker = data.entity_position_tracker;
        let factions = data.factions;
        let map = data.map;
        let map_hash = calculate_hash(&*map);
        let visibility = data.visibility;

        for (entity, _, command, pos, visibility) in (
            &data.entities,
            &data.logic,
            &mut data.commands,
            &mut data.pos,
            &visibility,
        )
            .join()
        {
            let target_move;
            if let Some(faction) = factions.get(entity) {
                // println!("Boo! 1");
                if let Some(visibility_map) = visibility.visibility_lookup().get(&map_hash) {
                    // println!("Boo! 2");
                    if let Some((_enemy, enemy_pos)) = entity_position_tracker.get_nearest_enemy(
                        *faction,
                        &factions,
                        visibility_map,
                    ) {
                        // println!("Boo! 3");
                        let disp = enemy_pos - *pos;
                        target_move = Displacement::new(disp.x.signum(), disp.y.signum());
                    } else if let Some((_friend, friend_pos)) = entity_position_tracker
                        .get_nearest_friend(entity, *faction, &factions, visibility_map)
                    {
                        // println!("Boo! 3");
                        let disp = friend_pos - *pos;
                        target_move = Displacement::new(disp.x.signum(), disp.y.signum());
                    } else {
                        target_move = get_random_move();
                    }
                } else {
                    target_move = get_random_move();
                }
            } else {
                target_move = get_random_move();
            }

            *command = Command::Move(target_move);

            /* let new_pos = *pos + target_move;

                if map.passable_pos( new_pos ) &&
                creature_tracker.check_collision( entity, new_pos ) == None
                {
                 *pos = new_pos;
            } */
        }
    }
}