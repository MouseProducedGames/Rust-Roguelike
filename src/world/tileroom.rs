/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/

// External includes

// Internal includes
use crate::dungen::DungenCommon;
use crate::tiled_shapes_2d::TiledShape2D;
use super::{ Mapping, Tilemap };

pub struct TiledAreaFilter
{
    area: Box<dyn TiledArea>,
    shape_filter: Box<dyn TiledShape2D>,
}

impl TiledAreaFilter
{
    pub fn new( area: Box<dyn TiledArea>, shape_filter: Box<dyn TiledShape2D> ) -> Self
    {
        Self { area: area, shape_filter: shape_filter }
    }

    pub fn new_boxed( area: Box<dyn TiledArea>, shape_filter: Box<dyn TiledShape2D> ) -> Box<dyn TiledArea>
    {
        let output: Box<dyn TiledArea> =
            Box::new( Self { area: area, shape_filter: shape_filter } );
        output
    }
}

impl DungenCommon for TiledAreaFilter
{
    fn finish( &mut self ) -> Tilemap
    {
        self.area.finish()
    }
}

impl Mapping for TiledArea
{
    fn height( &self ) -> u32
    {
        TiledArea::height( self ) as u32
    }

    fn width( &self ) -> u32
    {
        TiledArea::width( self ) as u32
    }
}

pub trait TiledArea
{
    fn bottom( &self ) -> u32;
    
    fn circumference( &self ) -> u32;

    fn finish( &mut self ) -> Tilemap;
        
    fn height( &self ) -> u32;

    fn iter_circumference( &self, iter_index: &mut u32 ) -> Option< ( u32, u32 ) >;
    
    fn iter_surface_area( &self, iter_index: &mut u32 ) -> Option< ( u32, u32 ) >;

    fn left( &self ) -> u32;
    
    fn right( &self ) -> u32;
    
    fn surface_area( &self ) -> u32;

    fn tile_type( &self, x: u32, y: u32 ) -> u32;

    fn tile_type_mut( &mut self, x: u32, y: u32 ) -> &mut u32;

    fn top( &self ) -> u32;
    
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
    fn bottom( &self ) -> u32
    {
        self.height()
    }
    
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
    
    fn left( &self ) -> u32
    {
        0
    }
    
    fn right( &self ) -> u32
    {
        self.width()
    }
    
    fn surface_area( &self ) -> u32
    {
        ( self.height() * self.width() ) as u32
    }

    fn tile_type( &self, x: u32, y: u32 ) -> u32
    {
        self.tile_type( x, y )
    }

    fn tile_type_mut( &mut self, x: u32, y: u32 ) -> &mut u32
    {
        self.tile_type_mut( x, y )
    }

    fn top( &self ) -> u32
    {
        0
    }
    
    fn width( &self ) -> u32
    {
        self.width() as u32
    }
}

impl TiledArea for TiledAreaFilter
{
    fn bottom( &self ) -> u32
    {
        self.area.bottom().min( self.shape_filter.bottom() )
    }
    
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
        self.bottom() - self.top()
    }

    fn iter_circumference( &self, iter_index: &mut u32 ) -> Option< ( u32, u32 ) >
    {
        self.shape_filter.iter_circumference( iter_index )
    }
    
    fn iter_surface_area( &self, iter_index: &mut u32 ) -> Option< ( u32, u32 ) >
    {
        self.shape_filter.iter_surface_area( iter_index )
    }
    
    fn left( &self ) -> u32
    {
        self.area.left().max( self.shape_filter.left() )
    }
    
    fn right( &self ) -> u32
    {
        self.area.right().min( self.shape_filter.right() )
    }
    
    fn surface_area( &self ) -> u32
    {    
        self.shape_filter.surface_area()
    }

    fn tile_type( &self, x: u32, y: u32 ) -> u32
    {
        self.area.tile_type( x, y )
    }

    fn tile_type_mut( &mut self, x: u32, y: u32 ) -> &mut u32
    {
        self.area.tile_type_mut( x, y )
    }

    fn top( &self ) -> u32
    {
        self.area.top().max( self.shape_filter.top() )
    }
    
    fn width( &self ) -> u32
    {
        self.right() - self.left()
    }
}

impl TiledShape2D for TiledArea
{
    fn bottom( &self ) -> u32
    {
        TiledArea::bottom( self )
    }

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

    fn left( &self ) -> u32
    {
        TiledArea::left( self )
    }

    fn right( &self ) -> u32
    {
        TiledArea::right( self )
    }

    fn surface_area( &self ) -> u32
    {
        TiledArea::surface_area( self )
    }

    fn top( &self ) -> u32
    {
        TiledArea::top( self )
    }
}

impl TiledShape2D for Box<dyn TiledArea>
{
    fn bottom( &self ) -> u32
    {
        self.bottom()
    }
    
    fn circumference( &self ) -> u32
    {
        self.circumference()
    }

    fn iter_circumference( &self, iter_index: &mut u32 ) -> Option< ( u32, u32 ) >
    {
        self.iter_circumference( iter_index )
    }

    fn iter_surface_area( &self, iter_index: &mut u32 ) -> Option< ( u32, u32 ) >
    {
        self.iter_surface_area( iter_index )
    }

    fn left( &self ) -> u32
    {
        self.left()
    }
    
    fn right( &self ) -> u32
    {
        self.right()
    }
    
    fn surface_area( &self ) -> u32
    {
        self.surface_area()
    }

    fn top( &self ) -> u32
    {
        self.top()
    }
}
