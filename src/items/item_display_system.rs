/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

 **/
// External includes.
use specs::{ReadExpect, ReadStorage, System, WriteExpect};

// Standard includes.
use std::sync::{Arc, Mutex};

// Internal includes.
use crate::ai::PlayerPosition;
use crate::io::Display;
use crate::items::Item;
use crate::rrl_math::Position;

pub struct ItemDisplaySystem;

#[derive(SystemData)]
pub struct SystemDataT<'a> {
    player_pos: ReadExpect<'a, PlayerPosition>,
    display: WriteExpect<'a, Arc<Mutex<Display>>>,
    items: ReadStorage<'a, Item>,
    positions: ReadStorage<'a, Position>,
}

impl<'a> System<'a> for ItemDisplaySystem {
    type SystemData = SystemDataT<'a>;

    fn run(&mut self, data: Self::SystemData) {
        use specs::join::Join;

        let view_pos = data.player_pos.0;
        let mut display = data.display.lock().unwrap();

        for (item, pos) in (&data.items, &data.positions).join() {
            display.write_item(item.clone(), *pos, view_pos);
        }
    }
}
