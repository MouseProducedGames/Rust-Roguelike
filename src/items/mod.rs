/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.

// Standard includes.

// Internal includes.
pub mod armours;
mod inventory;
mod inventory_screen;
mod item;
mod item_display_system;
mod light_source;
mod transfer_item;
pub mod weapons;
pub use inventory::{CreatureInventorySystem, Inventory, InventorySystem};
pub use inventory_screen::InventoryScreen;
pub use item::Item;
pub use item_display_system::ItemDisplaySystem;
pub use light_source::{LightSource, LightSourceSystem};
pub use transfer_item::TransferItem;

pub static ITEM_ICON_INDEX_TORCH: u32 = 0;
pub static ITEM_ICON_INDEX_WEAPON: u32 = 1;
pub static ITEM_ICON_INDEX_ARMOUR: u32 = 2;
