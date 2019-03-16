/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/

// External includes

// Internal includes
use crate::dungen::DungenCommon;
use crate::tiled_shapes_2d::TiledShape2D;
use super::{ Mapping, Tilemap };

pub struct TiledAreaFilter<'a>
{
    area: &'a mut TiledArea,
    shape_filter: &'a mut TiledShape2D,
}

impl<'a> TiledAreaFilter<'a>
{
    pub fn new( area: &'a mut TiledArea, shape_filter: &'a mut TiledShape2D ) -> Self
    {
        Self { area: area, shape_filter: shape_filter }
    }
}

impl<'a> DungenCommon for TiledAreaFilter<'a>
{
    fn finish( &mut self ) -> Tilemap
    {
        self.area.finish()
    }
}

impl Mapping for TiledArea
{
    fn height( &self ) -> usize
    {
        TiledArea::height( self ) as usize
    }

    fn width( &self ) -> usize
    {
        TiledArea::width( self ) as usize
    }
}

pub trait TiledArea
{
    fn circumference( &self ) -> u32;

    fn finish( &mut self ) -> Tilemap;
        
    fn height( &self ) -> u32;

    fn iter_circumference( &self, iter_index: &mut u32 ) -> Option< ( u32, u32 ) >;
    
    fn iter_surface_area( &self, iter_index: &mut u32 ) -> Option< ( u32, u32 ) >;
    
    fn surface_area( &self ) -> u32;

    fn value( &self, x: u32, y: u32 ) -> u32;

    fn value_mut( &mut self, x: u32, y: u32 ) -> &mut u32;

    fn width( &self ) -> u32;
}

impl DungenCommon for TiledArea
{
    fn finish( &mut self ) -> Tilemap
    {
        TiledArea::finish( self )
    }    
}

impl TiledArea for Tilemap
{
    fn circumference( &self ) -> u32
    {
        // Should be optimized: TWo gets, two adds, and a return.
        let half = self.height() + self.width();
        ( half + half ) as u32
    }

    fn finish( &mut self ) -> Tilemap
    {
        DungenCommon::finish( self )
    }
    
    fn height( &self ) -> u32
    {
        self.height() as u32
    }

    fn iter_circumference( &self, iter_index: &mut u32 ) -> Option< ( u32, u32 ) >
    {
        let ( width, height ) = ( self.width() as u32, self.height() as u32 );
        let index = *iter_index;
        *iter_index += 1;
        if index < width
        {
            Some( ( index, height ) )
        }
        else if index < ( width + height )
        {
            Some( ( width, height + ( index - width ) ) )
        }
        else if index < ( width + height + width )
        {
            let temp = index - ( width + height );
            Some( ( temp, height ) )
        }
        else if index < ( width + height + width + height )
        {
            let temp = index - ( width + height + width );
            Some( ( 0, temp ) )
        }
        else { None }
    }
    
    fn iter_surface_area( &self, iter_index: &mut u32 ) -> Option< ( u32, u32 ) >
    {
        let ( width, height ) = ( self.width() as u32, self.height() as u32 );
        let index = *iter_index;
        *iter_index += 1;
        let x = index % width;
        let y = index / width;
        if y == height
        {
            return None;
        }
        
        Some( ( ( x ), ( y ) ) )
    }
    
    fn surface_area( &self ) -> u32
    {
        ( self.height() * self.width() ) as u32
    }

    fn value( &self, x: u32, y: u32 ) -> u32
    {
        self.tile_type( x as usize, y as usize )
    }

    fn value_mut( &mut self, x: u32, y: u32 ) -> &mut u32
    {
        self.tile_type_mut( x as usize, y as usize )
    }

    fn width( &self ) -> u32
    {
        self.width() as u32
    }
}

impl<'a> TiledArea for TiledAreaFilter<'a>
{
    fn circumference( &self ) -> u32
    {
        self.shape_filter.circumference()
    }

    fn finish( &mut self ) -> Tilemap
    {
        self.area.finish()
    }    

    fn height( &self ) -> u32
    {
        self.area.height()
    }

    fn iter_circumference( &self, iter_index: &mut u32 ) -> Option< ( u32, u32 ) >
    {
        self.shape_filter.iter_circumference( iter_index )
    }
    
    fn iter_surface_area( &self, iter_index: &mut u32 ) -> Option< ( u32, u32 ) >
    {
        self.shape_filter.iter_surface_area( iter_index )
    }
    
    fn surface_area( &self ) -> u32
    {    
        self.shape_filter.surface_area()
    }

    fn value( &self, x: u32, y: u32 ) -> u32
    {
        self.area.value( x, y )
    }

    fn value_mut( &mut self, x: u32, y: u32 ) -> &mut u32
    {
        self.area.value_mut( x, y )
    }

    fn width( &self ) -> u32
    {
        self.area.width()
    }
}

impl TiledShape2D for TiledArea
{
    fn circumference( &self ) -> u32
    {
        TiledArea::circumference( self )
    }

    fn iter_circumference( &self, iter_index: &mut u32 ) -> Option< ( u32, u32 ) >
    {
        TiledArea::iter_circumference( self, iter_index )
    }

    fn iter_surface_area( &self, iter_index: &mut u32 ) -> Option< ( u32, u32 ) >
    {
        TiledArea::iter_surface_area( self, iter_index )
    }

    fn surface_area( &self ) -> u32
    {
        TiledArea::surface_area( self )
    }
}
