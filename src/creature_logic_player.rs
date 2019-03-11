use super::creature::{ Creature, Mobile };
use super::creature_logic::CreatureLogic;
use super::game_state::GameState;
use super::linear::Displacement;
use super::tilemap::Tilemap;
use super::tiletype::{ TILE_TYPE_DATA, TILE_TYPE_INDEX_VOID };

pub struct CreatureLogicPlayer
{
}

impl CreatureLogicPlayer
{
    pub fn new() -> Self { Self {} }
}

impl CreatureLogic for CreatureLogicPlayer
{
    fn update(&self, target: &mut Creature, map: &Tilemap, game_state: &mut GameState)
    {
        let ( map_height, map_width ) = map.bounds();
        // get_char refreshes the screen. Why??
        let command = game_state.window().get_char();
        let player_move;
        match command
        {
            '1' =>   player_move = Displacement::new(-1,  1),
            '2' =>   player_move = Displacement::new( 0,  1),
            '3' =>   player_move = Displacement::new( 1,  1),
            '4' =>   player_move = Displacement::new(-1,  0),
            '6' =>   player_move = Displacement::new( 1,  0),
            '7' =>   player_move = Displacement::new(-1, -1),
            '8' =>   player_move = Displacement::new( 0, -1),
            '9' =>   player_move = Displacement::new( 1, -1),
            'q' => { player_move = Displacement::new( 0,  0); game_state.kill(); },
            _ =>     player_move = Displacement::new( 0,  0),
        }

        let player_pos = target.get_position();
        let player_new_pos = player_pos + player_move;
        let tile_type;
        if (player_new_pos.x < 0) || (player_new_pos.x as usize >= map_width) ||
            (player_new_pos.y < 0) || (player_new_pos.y as usize >= map_height)
        {
            tile_type = TILE_TYPE_INDEX_VOID;
        }
        else
        {
            tile_type = map.tile( player_new_pos.x as usize, player_new_pos.y as usize );
        }

        let tile_type_data = &TILE_TYPE_DATA[ tile_type as usize ];
        if tile_type_data.passable()
        {
            target.move_self( player_move.x, player_move.y );
        }
    }
}

    
