/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

 **/
// External includes
use specs::{ReadExpect, ReadStorage, System, WriteExpect};
use std::sync::{Arc, Mutex};

// Internal includes
use crate::creature::PlayerPosition;
use crate::factions::Faction;
use crate::io::Display;
use crate::rrl_math::Position;

pub struct CreatureDisplaySystem;

#[derive(SystemData)]
pub struct SystemDataT<'a> {
    player_pos: ReadExpect<'a, PlayerPosition>,
    factions: ReadStorage<'a, Faction>,
    positions: ReadStorage<'a, Position>,
    display: WriteExpect<'a, Arc<Mutex<Display>>>,
}

impl<'a> System<'a> for CreatureDisplaySystem {
    type SystemData = SystemDataT<'a>;

    fn run(&mut self, data: Self::SystemData) {
        use specs::join::Join;

        let view_pos = data.player_pos.0;
        let mut window = data.display.lock().unwrap();

        for (faction, pos) in (&data.factions, &data.positions).join() {
            window.write_creature(*faction, *pos, view_pos);
        }
    }
}
