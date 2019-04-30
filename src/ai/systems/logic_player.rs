/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use specs::{
    Component, DenseVecStorage, Entities, ReadExpect, ReadStorage, System, WriteExpect,
    WriteStorage,
};

// Standard includes.
use std::sync::{Arc, Mutex};

// Internal includes.
use crate::ai::{Command, PlayerMarker};
use crate::bodies::BodyScreen;
use crate::game::GameState;
use crate::io::Input;
use crate::items::{InventoryScreen, TransferItem};
use crate::rrl_math::Displacement;
use crate::screens::PickupScreen;

pub struct LogicPlayer {}

impl Component for LogicPlayer {
    type Storage = DenseVecStorage<Self>;
}

pub struct LogicPlayerSystem;

#[derive(SystemData)]
pub struct SystemDataT<'a> {
    input: ReadExpect<'a, Arc<Mutex<Input>>>,
    game_state: WriteExpect<'a, GameState>,
    entities: Entities<'a>,
    player_marker: ReadStorage<'a, PlayerMarker>,
    commands: WriteStorage<'a, Command>,
    logic: WriteStorage<'a, LogicPlayer>,
    transfer_items: WriteStorage<'a, TransferItem>,
}

impl<'a> System<'a> for LogicPlayerSystem {
    type SystemData = SystemDataT<'a>;

    fn run(&mut self, mut data: SystemDataT) {
        use specs::Join;

        let input = data.input.lock().unwrap();
        let mut game_state = data.game_state;

        for (_, entity, command, _, transfer_item) in (
            &mut data.logic,
            &data.entities,
            &mut data.commands,
            &data.player_marker,
            &mut data.transfer_items,
        )
            .join()
        {
            let key_command = input.instance().get_char();

            *command = match key_command {
                '1' => Command::Move(Displacement::new(-1, 1)),
                '2' => Command::Move(Displacement::new(0, 1)),
                '3' => Command::Move(Displacement::new(1, 1)),
                '4' => Command::Move(Displacement::new(-1, 0)),
                '6' => Command::Move(Displacement::new(1, 0)),
                '7' => Command::Move(Displacement::new(-1, -1)),
                '8' => Command::Move(Displacement::new(0, -1)),
                '9' => Command::Move(Displacement::new(1, -1)),
                'b' => {
                    *transfer_item = TransferItem::None;

                    let body_screen_ref = Arc::new(Mutex::new(BodyScreen::new(entity)));
                    game_state.lock_new_screens().push(body_screen_ref);

                    Command::Move(Displacement::new(0, 0))
                }
                'i' => {
                    *transfer_item = TransferItem::None;

                    let inventory_screen_ref = Arc::new(Mutex::new(InventoryScreen::new(entity)));
                    game_state.lock_new_screens().push(inventory_screen_ref);

                    Command::Move(Displacement::new(0, 0))
                }
                'p' => {
                    *transfer_item = TransferItem::None;

                    let pickup_screen_ref = Arc::new(Mutex::new(PickupScreen::new(entity)));
                    game_state.lock_new_screens().push(pickup_screen_ref);

                    Command::Move(Displacement::new(0, 0))
                }
                _ => Command::Move(Displacement::new(0, 0)),
            }
        }
    }
}
