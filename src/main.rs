// Linting
//
// "if x == false" is clear and easily understandable.
// "if !x" is only superficially good.
// Consider "if !list_item".
// Consider also the rate of spelling errors in code.
#![cfg_attr(feature = "cargo-clippy", allow(clippy::bool_comparison))]

/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/

// External includes.
#[macro_use]
extern crate derive_more;
extern crate enumflags2;
#[macro_use]
extern crate enumflags2_derive;
extern crate rand;
extern crate shred;
#[macro_use]
extern crate shred_derive;

use specs::World;

// Standard includes.
use std::sync::{Arc, Mutex};

// Internal includes.
extern crate rust_dice;
mod abilities;
mod ai;
mod background;
mod bodies;
mod creatures;
mod data_types;
mod dice;
mod dungen;
mod events;
mod factions;
mod game;
mod io;
mod items;
mod maps;
mod multidim;
mod rrl_math;
mod screens;
mod skills;
mod stats;
mod talents;
mod themes;
mod tiled_shapes_2d;
use screens::{ScreenManager, StartScreen};

fn main() {
    let mut world = World::new();

    let mut screen_manager = ScreenManager::new();

    screen_manager.push(Arc::new(Mutex::new(StartScreen::new())));

    while screen_manager.screen_count() > 0 {
        {
            screen_manager.update_start();
            screen_manager.draw(&mut world);
            screen_manager.update(&mut world);
        }
    }

    // Window::close();
}
