/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/

// External includes

// Internal includes
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub fn calculate_hash<T: Hash>( t: &T ) -> u64
{
    let mut hasher = DefaultHasher::new();
    t.hash( &mut hasher );
    hasher.finish()
}
