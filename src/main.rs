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
// External dependencies
#[macro_use]
extern crate derive_more;
extern crate rand;

extern crate rust_dice;

extern crate shred;
#[macro_use]

extern crate shred_derive;
use specs::World;
use std::sync::{Arc, Mutex};

// Internal dependencies.
mod abilities;
mod ai;
mod background;
mod data_types;
mod dice;
mod dungen;
mod factions;
mod game;
mod io;
mod items;
mod multidim;
mod multimap;
mod rrl_math;
mod screens;
mod skills;
mod stats;
mod talents;
mod tiled_shapes_2d;
mod world;
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
