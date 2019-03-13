// External includes
use std::cmp::{ Eq, PartialEq };
use std::convert::From;
use std::ops::{ Add, Sub };

// Internal includes
use super::Position;

#[derive(Copy, Clone)]
pub struct Displacement
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

    /* pub fn length_sqr(self) -> i32
    {
        return self.x * self.x + self.y * self.y;
    } */
}

impl Add<Displacement> for Displacement
{
    type Output = Displacement;
    
    fn add(self, other: Displacement) -> Displacement
    {
        Displacement { x: self.x + other.x, y: self.y + other.y, }
    }
}

impl Eq for Displacement {}

impl From<Position> for Displacement
{
    fn from(item: Position ) -> Displacement
    {
        Displacement::new( item.x, item.y, )
    }
}

impl PartialEq for Displacement
{
    fn eq(&self, other: &Displacement) -> bool
    {
        self.x == other.x && self.y == other.y
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