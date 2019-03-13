// External includes
use specs::{ Component, VecStorage };
use std::cmp::{ Eq, PartialEq };
use std::convert::From;
use std::ops::{ Add, Sub };

// Internal includes
use super::Displacement;


#[derive(Copy, Clone, Debug, Default)]
pub struct Position
{
    pub x: i32,
    pub y: i32,
}

impl Component for Position
{
    type Storage = VecStorage<Self>;
}

impl Position
{
    pub fn new(x: i32, y: i32) -> Self
    {
        Self { x: x, y: y }
    }

    /* pub fn length_sqr(self) -> i32
    {
        return self.x * self.x + self.y * self.y;
    } */
}

impl Add<Displacement> for Position
{
    type Output = Position;

    fn add(self, other: Displacement) -> Position
    {
        Position { x: self.x + other.x, y: self.y + other.y }
    }
}

impl Eq for Position {}

impl From<Displacement> for Position
{
    fn from(item: Displacement ) -> Position
    {
        Position::new( item.x, item.y, )
    }
}

impl PartialEq for Position
{
    fn eq(&self, other: &Position) -> bool
    {
        self.x == other.x && self.y == other.y
    }
}

impl Sub<Displacement> for Position
{
    type Output = Position;

    fn sub(self, other: Displacement) -> Position
    {
        Position { x: self.x - other.x, y: self.y - other.y }
    }
}

impl Sub<Position> for Position
{
    type Output = Displacement;

    fn sub(self, other: Position) -> Displacement
    {
        Displacement { x: self.x - other.x, y: self.y - other.y }
    }

}
