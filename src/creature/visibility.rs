/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/

// External includes
use specs::{ Component, VecStorage };
use std::collections::hash_map::HashMap;

// internal includes
use crate::world::VisibilityMap;

pub struct Visibility
{    
    visibility_lookup: HashMap< u64, VisibilityMap >,
}

impl Visibility
{
    pub fn new() -> Visibility
    {
        Self { visibility_lookup: HashMap::new() }
    }
    
    pub fn visibility_lookup( &self ) -> &HashMap< u64, VisibilityMap >
    {
        &self.visibility_lookup
    }
    
    pub fn visibility_lookup_mut( &mut self ) -> &mut HashMap< u64, VisibilityMap >
    {
        &mut self.visibility_lookup
    }
    
    /* pub fn get_visibility( &self, map: &Tilemap ) -> Option< &VisibilityMap >
    {
        self.visibility_lookup.get( &calculate_hash( map ) )
    } */
}

impl Component for Visibility
{
    type Storage = VecStorage<Self>;
}
