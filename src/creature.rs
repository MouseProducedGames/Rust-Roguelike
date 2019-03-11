use super::linear;
use super::creature_logic::CreatureLogic;
use super::game_state::GameState;
use super::linear::{ Position };
use super::tilemap::Tilemap;

pub trait Mobile
{
    fn move_self(&mut self, move_x: i32, move_y: i32);
}

pub struct Creature<'a>
{
    logic: &'a CreatureLogic,
    pos: linear::Position,
}

impl<'a> Creature<'a>
{
    pub fn new(logic: &'a CreatureLogic, start_x: i32, start_y: i32) -> Self
    {
        Self {
            logic: logic,
            pos: Position::new(start_x, start_y),
        }
    }

    pub fn get_position(&self) -> Position
    {
        self.pos
    }

    pub fn update(&mut self, map: &Tilemap, game_state: &mut GameState)
    {
        self.logic.update( self, map, game_state );
    }
}

impl<'a> Mobile for Creature<'a>
{
    fn move_self(&mut self, move_x: i32, move_y: i32)
    {
        self.pos.x += move_x;
        self.pos.y += move_y;
    }
}
