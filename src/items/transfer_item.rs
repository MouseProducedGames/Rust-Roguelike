/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use specs::{Component, Entity, VecStorage};

// Standard includes.

// Internal includes.

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TransferItem {
    None,
    Fetch,
    FromInventory(Entity),
    _ToInventory(Entity),
}

impl Component for TransferItem {
    type Storage = VecStorage<Self>;
}
