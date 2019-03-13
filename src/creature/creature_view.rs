/* use super::{ Creature, Mobile };
use crate::rrl_math::Position;

pub trait CreatureView
{
    fn get_position(&self) -> Position;
    fn move_self(&mut self, move_x: i32, move_y: i32);
}

impl<'a> CreatureView for Creature<'a> where Creature<'a>: Mobile
{
    fn get_position(&self) -> Position
    {
        Creature::get_position(self)
    }
    
    fn move_self(&mut self, move_x: i32, move_y: i32)
    {
        // self.move_self(move_x, move_y);
        Mobile::move_self(self, move_x, move_y);
        // Mobile::move_self(& mut self, move_x, move_y);
//         self.pos.x += move_x;
//         self.pos.y += move_y;

    }
}
 */
