// External dependencies.
use specs::{ Component, NullStorage, ReadExpect, ReadStorage, System, WriteExpect, WriteStorage };
use rand::Rng;

// Internal dependencies.
use super::super::game_state::GameState;
use crate::rrl_math::{ Displacement, Position };
use crate::world::Tilemap;

#[derive(Default)]
pub struct CreatureLogicWander;

impl Component for CreatureLogicWander
{
    type Storage = NullStorage<Self>;
}

pub struct CreatureLogicWanderSystem;

impl<'a> System<'a> for CreatureLogicWanderSystem
{
    type SystemData = (
        ReadExpect< 'a, Tilemap >,
        ReadStorage< 'a, CreatureLogicWander >,
        WriteExpect< 'a, GameState >,
        WriteStorage< 'a, Position >,
    );
    
    fn run( &mut self, ( map, creature_logic_wander, game_state, mut pos ): Self::SystemData )
    {
        use specs::join::Join;
        
        let mut game_state = game_state;
        let map = map;

        for ( _, pos ) in ( &creature_logic_wander, &mut pos ).join()
        {
            let command = game_state.rng().gen_range(1, 10);
            let target_move;
            match command
            {
                1 =>   target_move = Displacement::new(-1,  1),
                2 =>   target_move = Displacement::new( 0,  1),
                3 =>   target_move = Displacement::new( 1,  1),
                4 =>   target_move = Displacement::new(-1,  0),
                5 =>   target_move = Displacement::new( 0,  0),
                6 =>   target_move = Displacement::new( 1,  0),
                7 =>   target_move = Displacement::new(-1, -1),
                8 =>   target_move = Displacement::new( 0, -1),
                9 =>   target_move = Displacement::new( 1, -1),
                _ =>   target_move = Displacement::new( 0,  0),
            }

            let new_pos = *pos + target_move;

            if map.passable_pos( new_pos )
            {
                *pos = new_pos;
            }
        }
    }
}
