/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/

/* // External includes

// Internal includes

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct FactionData
{
    id: usize,
    _version: usize,
}

impl FactionData
{
    fn new( id: usize, _version: usize ) -> Self
    {
        Self {
            id: id,
            _version: _version,
        }
    }
}

pub struct FactionLookup
{
    alive_values: Vec< bool >,
    values: Vec< FactionData >,
    first_emptied: usize,
    last_filled: usize,
}

impl FactionLookup
{
    pub fn new() -> Self
    {
        Self {
            alive_values: vec![],
            values: vec![],
            first_emptied: 0,
            last_filled: 0,
        }
    }

    pub fn add( &mut self ) -> FactionData
    {
        let mut id: Option<usize>;
        let _version = 1_usize;
        if self.first_emptied < self.values.len() &&
            self.alive_values [ self.first_emptied ] == false
        {
            let first_emptied = self.first_emptied;
            id = Some( first_emptied );
            self.values[ first_emptied ] = FactionData::new( first_emptied, _version )
        }
        else if self.values.len() < self.values.capacity()
        {
            let mut found = false;
            id = None;
            for i in 0..self.alive_values.len()
            {
                if self.alive_values[ i ] == false
                {
                    id = Some( i );
                    self.values.push( FactionData::new( i, _version ) );
                    found = true;
                    break;
                }
            }

            if found == false
            {
                
            }
        }
        else
        {
            let index = self.values.len();
            id = Some( index );
            self.values.push( FactionData::new( index,_version ) );
        }
        match id {
            Some( index ) => {
                if index > self.last_filled
                {
                    self.last_filled = index;
                }
                self.values[ index ]
            },
            _ => panic!( "We could not find an empty slot, and we have not run out of available memory. This should be impossible!" ),
        }
    }

    pub fn get_faction_state( &self, faction: FactionData ) -> FactionData
    {
        let index = faction.id;
        self.values[ index ]
    }

    pub fn has_faction_changed( &self, faction: FactionData ) -> bool
    {
        let index = faction.id;
        let _version = faction._version;
        let current = self.values[ index ];
        faction != current
    }

    fn _mark_modified( &mut self, faction: FactionData ) -> FactionData
    {
        let index = faction.id;
        let _version = self.values[ index ]._version;
        self.values[ index ] = FactionData::new( index, _version + 1 );
        self.values[ index ]
    }
}
*/
