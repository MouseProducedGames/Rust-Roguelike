/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/

// External includes.
use specs::Entity;
use std::collections::HashMap;

// Internal includes.
use crate::rrl_math::Position;

pub struct _CreatureData
{
    pos: Position,
}

pub struct CreatureTracker
{
    lookup: HashMap<Entity, Position>,
}

impl CreatureTracker
{
    pub fn new() -> Self
    {
        Self {
            lookup: HashMap::new()
        }
    }

    pub fn check_collision( &self, entity: Entity, check_pos: Position ) -> Option< Entity >
    {
        let mut output = None;
        for ( other, other_pos ) in self.lookup.iter()
        {
            if entity != *other &&
                check_pos == *other_pos
            {
                output = Some( *other );
            }
        }

        output
    }

    pub fn set_position( &mut self, entity: Entity, pos: Position )
    {
        self.lookup.insert( entity, pos );
    }
}
