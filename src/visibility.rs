// External includes
use std::default::Default;

// Internal includes
use super::mapping::Mapping;
use super::multidim::Multidim;
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
    values: Multidim<VisibilityType>,
}

impl VisibilityMap
{
    pub fn new(width: usize, height: usize) -> Self
    {
        Self { values: Multidim::new( height, width ) }
    }

    pub fn width(&self) -> usize
    {
        self.values.len1()
    }
    
    pub fn height(&self) -> usize
    {
        self.values.len0()
    }

    pub fn value(&self, pos_x: usize, pos_y: usize) -> VisibilityType
    {
        if self.is_in_bounds( pos_x, pos_y )
        {
            *self.values.value( pos_y, pos_x )
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
        self.values.value_mut( pos_y, pos_x )
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
