/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External dependencies.
use specs::{Component, VecStorage};

// Internal dependencies.
use crate::rrl_math::Displacement;

#[derive(Copy, Clone)]
pub enum Command {
    None,
    Move(Displacement),
}

impl Component for Command {
    type Storage = VecStorage<Self>;
}

/* #[derive(Copy, Clone)]
pub struct DisplaceCommand
{
    pub cmd: CommandType,
    pub disp: Displacement,
}

impl DisplaceCommand
{
    pub fn new( cmd: CommandType, disp: Displacement ) -> Self
    {
        Self { cmd: cmd, disp: disp }
    }
}

#[derive(Copy, Clone)]
pub struct UnknownCommand
{
    pub cmd: CommandType,
}

impl UnknownCommand
{
    pub fn new( cmd: CommandType ) -> Self
    {
        Self { cmd: cmd }
    }
} */
