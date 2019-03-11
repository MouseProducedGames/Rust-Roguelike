use super::linear;
use super::linear::{ Position };

pub trait Mobile
{
    fn move_self(&mut self, move_x: i32, move_y: i32);
}

pub struct Creature
{
    pos: linear::Position,
}

impl Creature
{
    pub fn new(start_x: i32, start_y: i32) -> Self
    {
        Self {
            pos: Position::new(start_x, start_y),
        }
    }

    pub fn get_position(&self) -> Position
    {
        self.pos
    }
}

impl Mobile for Creature
{
    fn move_self(&mut self, move_x: i32, move_y: i32)
    {
        self.pos.x += move_x;
        self.pos.y += move_y;
    }
}
