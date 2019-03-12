use super::linear::Position;

pub trait Mapping
{
    fn bounds(&self) -> ( usize, usize);

    fn is_i32_in_bounds(&self, pos_x: i32, pos_y: i32) -> bool;

    fn is_in_bounds(&self, pos_x: usize, pos_y: usize) -> bool;

    fn is_pos_in_bounds(&self, pos: Position) -> bool;

    fn is_i32_in_height(&self, pos_y: i32) -> bool;

    fn is_i32_in_width(&self, pos_x: i32) -> bool;

    fn is_in_height(&self, pos_y: usize) -> bool;

    fn is_in_width(&self, pos_x: usize) -> bool;
}
