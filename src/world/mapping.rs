use crate::rrl_math::Position;

pub trait Mapping
{
    fn height(&self) -> usize;

    fn width(&self) -> usize;
    
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
