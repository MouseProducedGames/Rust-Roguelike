// External includes
use std::default::Default;

// Internal includes
use super::mapping::Mapping;
use super::multidim::Multidim;
use super::linear::Position;

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
    fn bounds(&self) -> ( usize, usize)
    {
        ( self.width(), self.height() )
    }

    fn is_i32_in_bounds(&self, pos_x: i32, pos_y: i32) -> bool
    {
        self.is_i32_in_height( pos_y ) && self.is_i32_in_width( pos_x )
    }

    fn is_in_bounds(&self, pos_x: usize, pos_y: usize) -> bool
    {
        self.is_in_height( pos_y ) && self.is_in_width( pos_x )
    }

    fn is_pos_in_bounds(&self, pos: Position) -> bool
    {
        self.is_i32_in_bounds( pos.x, pos.y )
    }

    fn is_i32_in_height(&self, pos_y: i32) -> bool
    {
        return pos_y >= 0 && self.is_in_height( pos_y as usize)
    }

    fn is_i32_in_width(&self, pos_x: i32) -> bool
    {
        return pos_x >= 0 && self.is_in_width( pos_x as usize)
    }

    fn is_in_height(&self, pos_y: usize) -> bool
    {
        pos_y < self.height()
    }

    fn is_in_width(&self, pos_x: usize) -> bool
    {
        pos_x < self.width()
    }
}
