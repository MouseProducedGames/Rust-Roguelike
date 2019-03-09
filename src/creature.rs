pub trait Mobile
{
    fn move_self(&mut self, move_x: i32, move_y: i32);
}

pub struct Creature
{
    pos_x: i32,
    pos_y: i32,
}

impl Creature
{
    pub fn new(start_x: i32, start_y: i32) -> Self
    {
        Self {
            pos_x: start_x,
            pos_y: start_y,
        }
    }

    pub fn get_position(&self) -> ( i32, i32 )
    {
        return ( self.pos_x, self.pos_y );
    }
}

impl Mobile for Creature
{
    fn move_self(&mut self, move_x: i32, move_y: i32)
    {
        self.pos_x = self.pos_x + move_x;
        self.pos_y = self.pos_y + move_y;
    }
}
