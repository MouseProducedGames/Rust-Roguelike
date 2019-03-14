/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

 **/

// External includes

// Internal includes

pub static absolute: f32 = 1.0;
pub static extreme: f32 = 0.8;
pub static strong: f32 = 0.4;
pub static moderate: f32 = 0.2;
pub static mild: f32 = 0.10;
pub static neutral: f32 = 0.0;

pub fn calc_opinion( my_faction_id: u32, other_faction_id: u32 ) -> LoveFearHate
{
    match my_faction_id == other_faction_id {
        // Preliminary values. Replace with actual faction system.
        true => LoveFearHate::new( 1.0 ),
        false => LoveFearHate::new( -1.0 ),
    }
}

pub struct LoveFearHate
{
    value: f32,
}

impl LoveFearHate
{
    pub fn new( value: f32 ) -> Self
    {
        Self { value: value }
    }

    pub fn absolute_value( &self ) -> f32
    {
        self.value
    }

    pub fn love_value( &self, my_rel_str: f32 ) -> f32
    {
        match self.value > 0.0 {
            // If their relative strength is greater than mine,
            // then increase the love value.
            true => ( self.value - my_rel_str.max( 0.0 ) ),
            false => 0.0
        }
    }

    pub fn fear_value( &self, my_rel_str: f32 ) -> f32
    {
        match self.value < 0.0 {
            // If my relative strength is less than theirs,
            // then increase the fear value.
            true => ( -self.value - my_rel_str.max( 0.0 ) ),
            false => 0.0
        }
    }

    pub fn hate_value( &self, my_rel_str: f32 ) -> f32
    {
        match self.value < 0.0 {
            // If my relative strength is greater than theirs,
            // then increase the hate value.
            true => ( -self.value + my_rel_str.min( 0.0 ) ),
            false => 0.0
        }
    }
}
