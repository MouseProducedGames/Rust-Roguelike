// External includes
use specs::{ Component, NullStorage };

// internal includes

#[derive(Default)]
pub struct PlayerMarker;

impl Component for PlayerMarker
{
    type Storage = NullStorage<Self>;
}
