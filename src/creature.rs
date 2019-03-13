// External includes
use std::collections::hash_map::HashMap;

// internal includes
use super::creature_logic::CreatureLogic;
use super::game_state::GameState;
use super::linear::Position;
use super::line_of_sight;
use super::mapping::Mapping;
use super::tilemap::Tilemap;
use super::visibility::VisibilityMap;

pub trait Mobile
{
    fn move_self( &mut self, move_x: i32, move_y: i32 );
}

pub struct Creature<'a>
{
    logic: &'a CreatureLogic,
    pos: Position,
    sight_range: i32,
    visibility_lookup: HashMap< &'a Tilemap, VisibilityMap >,
}

impl<'a> Creature<'a>
{
    pub fn new( logic: &'a CreatureLogic, start_x: i32, start_y: i32 ) -> Self
    {
        Self {
            logic: logic,
            pos: Position::new( start_x, start_y ),
            sight_range: 5,
            visibility_lookup: HashMap::new(),
        }
    }

    pub fn calculate_visibility( &mut self, map: &'a Tilemap )
    {
        if self.visibility_lookup.contains_key( map ) == false
        {
            let ( map_width, map_height ) = map.bounds();
            self.visibility_lookup.insert( map, VisibilityMap::new( map_width, map_height )) ;
        }

        let pos = self.get_position();

        let visibility;
        match self.visibility_lookup.get_mut( map )
        {
            Some( vis_map ) => visibility = vis_map,
            _ => panic!( "We no longer have the visibility map we just added!" ),
        }

        let sight_range = self.sight_range;

        line_of_sight::calculate_visibility( visibility, pos, sight_range, map );
    }

    pub fn get_position( &self ) -> Position
    {
        self.pos
    }

    pub fn get_visibility<'b>( &self, map: &Tilemap ) -> Option< &VisibilityMap >
    {
        self.visibility_lookup.get( map )
    }

    pub fn update( &mut self, map: &Tilemap, game_state: &mut GameState )
    {
        self.logic.update( self, map, game_state );
    }
}

impl<'a> Mobile for Creature<'a>
{
    fn move_self( &mut self, move_x: i32, move_y: i32 )
    {
        self.pos.x += move_x;
        self.pos.y += move_y;
    }
}
