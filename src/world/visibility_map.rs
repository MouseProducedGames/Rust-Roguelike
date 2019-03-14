// External includes
use std::default::Default;

// Internal includes
use super::super::multimap::Multimap;
use crate::world::Mapping;
use crate::rrl_math::Position;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum VisibilityType
{
    None,
    Seen,
    Visible
}

impl Default for VisibilityType
{
    fn default() -> Self { VisibilityType::None }
}

pub struct VisibilityMap
{
    values: Multimap<VisibilityType>,
}

impl VisibilityMap
{
    pub fn new(width: usize, height: usize) -> Self
    {
        Self { values: Multimap::new( width, height ) }
    }

    pub fn height(&self) -> usize
    {
        self.values.height()
    }

    pub fn width(&self) -> usize
    {
        self.values.width()
    }
    
    pub fn value(&self, pos_x: usize, pos_y: usize) -> VisibilityType
    {
        if self.is_in_bounds( pos_x, pos_y )
        {
            *self.values.value( pos_x, pos_y )
        }
        else
        {
            VisibilityType::None
        }
    }

    pub fn value_pos(&self, pos: Position ) -> VisibilityType
    {
        if self.is_pos_in_bounds( pos )
        {
            self.value( pos.x as usize, pos.y as usize )
        }
        else
        {
            VisibilityType::None
        }
    }

    pub fn value_mut<'a>(&'a mut self, pos_x: usize, pos_y: usize) -> &'a mut VisibilityType
    {
        self.values.value_mut( pos_x, pos_y )
    }
}

impl Mapping for VisibilityMap
{
    fn height( &self ) -> usize
    {
        VisibilityMap::height( self )
    }

    fn width( &self ) -> usize
    {
        VisibilityMap::width( self )
    }
}
