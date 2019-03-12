// External includes
use std::collections::hash_map::HashMap;

// internal includes
use super::creature_logic::CreatureLogic;
use super::game_state::GameState;
use super::linear;
use super::linear::{ Displacement, Position };
use super::tilemap::Tilemap;
use super::visibility::VisibilityMap;

pub trait Mobile
{
    fn move_self(&mut self, move_x: i32, move_y: i32);
}

pub struct Creature<'a>
{
    logic: &'a CreatureLogic,
    pos: linear::Position,
    sight_range: i32,
    visibility_lookup: HashMap<&'a Tilemap, VisibilityMap>,
}

impl<'a> Creature<'a>
{
    pub fn new(logic: &'a CreatureLogic, start_x: i32, start_y: i32) -> Self
    {
        Self {
            logic: logic,
            pos: Position::new(start_x, start_y),
            sight_range: 5,
            visibility_lookup: HashMap::new(),
        }
    }

    pub fn calculate_visibility(&mut self, map: &'a Tilemap)
    {
        if self.visibility_lookup.contains_key(map) == false
        {
            let ( map_width, map_height ) = map.bounds();
            self.visibility_lookup.insert(map, VisibilityMap::new( map_width, map_height ));
        }

        let pos = self.get_position();

        let visibility;
        match self.visibility_lookup.get_mut(map)
        {
            Some(vis_map) => visibility = vis_map,
            _ => panic!("We no longer have the visibility map we just added!"),
        }

        let sight_range = self.sight_range;
        let sight_range_sqr = ( sight_range * sight_range ) as f64;
        for to_y in -sight_range..sight_range
        {
            for to_x in -sight_range..sight_range
            {
                let disp = Displacement::new( to_x, to_y );

                let to_pos = pos + disp;
                let disp_x = ( to_pos.x - pos.x ) as f64;
                let disp_y = ( to_pos.y - pos.y ) as f64;
                let dist = ( ( disp_x * disp_x ) + ( disp_y * disp_y ) ).sqrt();
                let disp_norm_x = disp_x / dist;
                let disp_norm_y = disp_y / dist;
                let mut move_pos_x: f64 = 0.0;
                let mut move_pos_y: f64 = 0.0;
                let mut check_pos_x = ( pos.x as f64 ) + 0.5;
                let mut check_pos_y = ( pos.y as f64 ) + 0.5;
                while ( ( move_pos_x * move_pos_x ) + ( move_pos_y * move_pos_y ) ) <= sight_range_sqr
                {
                    let check_pos = Position::new( check_pos_x as i32, check_pos_y as i32 );
                    if map.is_pos_in_bounds( check_pos ) == false
                    {
                        break;
                    }
                    *visibility.value_mut( check_pos.x as usize, check_pos.y as usize ) = true;
                    if map.transparent_pos( check_pos ) == false
                    {
                        break;
                    }
                    
                    check_pos_x += disp_norm_x;
                    check_pos_y += disp_norm_y;
                    move_pos_x += disp_norm_x;
                    move_pos_y += disp_norm_y;
                }
            }
        }
    }

    pub fn get_position(&self) -> Position
    {
        self.pos
    }

    pub fn get_visibility<'b>(&self, map: &Tilemap) -> Option<&VisibilityMap>
    {
        self.visibility_lookup.get(map)
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
