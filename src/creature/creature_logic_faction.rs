/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

 **/
// External dependencies.
use rand::Rng;
use specs::{
    Component, Entities, NullStorage, ReadExpect, ReadStorage, System, WriteExpect, WriteStorage,
};

// Internal dependencies.
use crate::creature::{Command, CreatureTracker, Visibility};
use crate::factions::Faction;
use crate::game::GameState;
use crate::rrl_math::{calculate_hash, Displacement, Position};
use crate::world::Tilemap;

#[derive(Default)]
pub struct CreatureLogicFaction;

impl Component for CreatureLogicFaction {
    type Storage = NullStorage<Self>;
}

pub struct CreatureLogicFactionSystem;

#[derive(SystemData)]
pub struct SystemDataT<'a> {
    creature_tracker: ReadExpect<'a, CreatureTracker>,
    entities: Entities<'a>,
    map: ReadExpect<'a, Tilemap>,
    visibility: ReadStorage<'a, Visibility>,
    factions: ReadStorage<'a, Faction>,
    logic: ReadStorage<'a, CreatureLogicFaction>,
    game_state: WriteExpect<'a, GameState>,
    commands: WriteStorage<'a, Command>,
    pos: WriteStorage<'a, Position>,
}

impl<'a> System<'a> for CreatureLogicFactionSystem {
    type SystemData = SystemDataT<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        use specs::join::Join;

        let creature_tracker = data.creature_tracker;
        let factions = data.factions;
        let mut game_state = data.game_state;
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
                    if let Some((_enemy, enemy_pos)) =
                        creature_tracker.get_nearest_enemy(*faction, &factions, visibility_map)
                    {
                        // println!("Boo! 3");
                        let disp = enemy_pos - *pos;
                        target_move = Displacement::new(disp.x.signum(), disp.y.signum());
                    } else if let Some((_friend, friend_pos)) = creature_tracker.get_nearest_friend(
                        entity,
                        *faction,
                        &factions,
                        visibility_map,
                    ) {
                        // println!("Boo! 3");
                        let disp = friend_pos - *pos;
                        target_move = Displacement::new(disp.x.signum(), disp.y.signum());
                    } else {
                        target_move = get_random_move(&mut game_state);
                    }
                } else {
                    target_move = get_random_move(&mut game_state);
                }
            } else {
                target_move = get_random_move(&mut game_state);
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

fn get_random_move<'a>(game_state: &mut WriteExpect<'a, GameState>) -> Displacement {
    let key_command = game_state.rng().gen_range(1, 10);
    let target_move;
    match key_command {
        1 => target_move = Displacement::new(-1, 1),
        2 => target_move = Displacement::new(0, 1),
        3 => target_move = Displacement::new(1, 1),
        4 => target_move = Displacement::new(-1, 0),
        5 => target_move = Displacement::new(0, 0),
        6 => target_move = Displacement::new(1, 0),
        7 => target_move = Displacement::new(-1, -1),
        8 => target_move = Displacement::new(0, -1),
        9 => target_move = Displacement::new(1, -1),
        _ => target_move = Displacement::new(0, 0),
    }

    target_move
}
