/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/

// External includes
use specs::{ Component, NullStorage };

// internal includes

#[derive(Default)]
pub struct ViewpointMarker;

impl Component for ViewpointMarker
{
    type Storage = NullStorage<Self>;
}
