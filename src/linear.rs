use std::cmp::{ Eq, PartialEq };
use std::convert::From;
use std::ops::{ Add, Sub };

#[derive(Copy, Clone)]
pub struct Displacement
{
    pub x: i32,
    pub y: i32,
}

#[derive(Copy, Clone)]
pub struct Position
{
    pub x: i32,
    pub y: i32,
}

impl Displacement
{
    pub fn new(x: i32, y: i32) -> Self
    {
        Self { x: x, y: y }
    }

    pub fn length_sqr(self) -> i32
    {
        return self.x * self.x + self.y * self.y;
    }
}

impl Add<Displacement> for Displacement
{
    type Output = Displacement;
    
    fn add(self, other: Displacement) -> Displacement
    {
        Displacement { x: self.x + other.x, y: self.y + other.y, }
    }
}

impl Sub for Displacement
{
    type Output = Displacement;

    fn sub(self, other: Displacement) -> Displacement
    {
        Displacement { x: self.x - other.x, y: self.y - other.y, }        
    }
}

impl From<Position> for Displacement
{
    fn from(item: Position ) -> Displacement
    {
        Displacement { x: item.x, y: item.y, }
    }
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
