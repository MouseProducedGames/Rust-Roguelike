/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

 **/

// External includes

// Internal includes

pub static _ABSOLUTE: f32 = 1.0;
pub static _EXTREME: f32 = 0.8;
pub static _STRONG: f32 = 0.4;
pub static _MODERATE: f32 = 0.2;
pub static _MILD: f32 = 0.10;
pub static _NEUTRAL: f32 = 0.0;

pub fn _calc_opinion( my_faction_id: u32, other_faction_id: u32 ) -> _LoveFearHate
{
    if my_faction_id == other_faction_id
    {
        // Preliminary values. Replace with actual faction system.
        _LoveFearHate::_new( 1.0 )
    } else { _LoveFearHate::_new( -1.0 ) }
}

pub struct _LoveFearHate
{
    value: f32,
}

impl _LoveFearHate
{
    pub fn _new( value: f32 ) -> Self
    {
        Self { value }
    }

    pub fn _absolute_value( &self ) -> f32
    {
        self.value
    }

    pub fn _calc_love( &self, my_rel_str: f32 ) -> f32
    {
        if self.value > 0.0 {
            // If their relative strength is greater than mine,
            // then increase the love value.
            ( self.value - my_rel_str.max( 0.0 ) )
        } else { 0.0 }
    }

    pub fn _calc_fear( &self, my_rel_str: f32 ) -> f32
    {
        if self.value < 0.0 {
            // If my relative strength is less than theirs,
            // then increase the fear value.
            ( -self.value - my_rel_str.max( 0.0 ) )
        } else { 0.0 }
    }

    pub fn _calc_hate( &self, my_rel_str: f32 ) -> f32
    {
        if self.value < 0.0 {
            // If my relative strength is greater than theirs,
            // then increase the hate value.
            ( -self.value + my_rel_str.min( 0.0 ) )
        } else { 0.0 }
    }
}
