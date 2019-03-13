// External includes

// internal includes
use std::default::Default;
use crate::rrl_math::Position;

// #[derive(Default)]
pub struct PlayerPosition( pub Position );

impl Default for PlayerPosition
{
    fn default() -> Self
    {
        Self( Position::new( 0, 0 ) )
    }
}
