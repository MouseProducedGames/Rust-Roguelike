/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/

// External includes

// internal include

pub trait Stat {
    fn value(&self) -> i32;

    fn value_mut(&mut self) -> &mut i32;
}

/* impl Sub for Stat
{
    fn sub( &self, other: &dyn Stat )
    {
        let output = self;
        output.value_mut() -= other.value();
        output
    }
}*/
