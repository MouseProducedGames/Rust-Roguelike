/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

 **/

// External includes.
use specs::{Component, VecStorage};
use std::collections::HashMap;

// Internal includes.
use crate::talents::{TalentActivation, TalentType};

pub struct TalentLookup
{
    values: HashMap<TalentActivation, Vec<TalentType>>,
}

impl TalentLookup
{
    pub fn new() -> Self
    {
        Self { values: HashMap::new() }
    }

    pub fn get_set( &mut self, key: TalentActivation ) -> &Vec<TalentType>
    {
        self.values.entry(key).or_insert(vec![])
    }

    pub fn get_set_mut( &mut self, key: TalentActivation ) -> &mut Vec<TalentType>
    {
        self.values.entry(key).or_insert(vec![])
    }

    pub fn insert( &mut self, key: TalentActivation, value: TalentType )
    {
        self.get_set_mut(key).push( value );
    }
}

impl Component for TalentLookup
{
    type Storage = VecStorage<Self>;
}
