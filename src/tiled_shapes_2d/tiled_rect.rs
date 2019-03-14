/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

 **/

// External dependencies

// Internal dependencies.
use crate::tiled_shapes_2d::TiledShapeDef2D;

pub struct TiledRect
{
    left: u32,
    top: u32,
    right: u32,
    bottom: u32,
}

impl TiledRect
{
    pub fn with_absolute_bounds(
        left: u32, top: u32,
        right: u32, bottom: u32
    ) -> Self
    {
        let ( use_left, use_right, use_top, use_bottom );
        if left > right
        { use_right = left; use_left = right; }
        else
        { use_left = left; use_right = right; }
        if top > bottom
        { use_top = bottom; use_bottom = top; }
        else
        { use_top = top; use_bottom = bottom; }

        Self {
            left: use_left, top: use_top,
            right: use_right, bottom: use_bottom
        }
    }

    pub fn height( &self ) -> u32
    {
        self.bottom - self.top
    }
    
    pub fn width( &self ) -> u32
    {
        self.right - self.left
    }
}

impl TiledShapeDef2D for TiledRect
{
    fn circumference( &self ) -> u32
    {
        // Should be optimized: TWo gets, two adds, and a return.
        let half = self.height() + self.width();
        half + half
    }

    fn iter_circumference( &self, iter_index: &mut u32 ) -> Option< ( u32, u32 ) >
    {
        let ( width, height ) = ( self.width(), self.height() );
        let index = *iter_index;
        *iter_index += 1;
        if index < width
        {
            return Some( ( index, 0 ) );
        }
        else if index < ( width + height )
        {
            return Some( ( ( width - 1 ), ( index - width ) ) );
        }
        else if index < ( width + height + width )
        {
            let temp = index - ( width + height );
            return Some( ( temp, ( height - 1 ) ) );
        }
        else if index < ( width + height + width + height )
        {
            let temp = index - ( width + height + width );
            return Some( ( 0, temp ) );
        }
        else
        {
            return None;
        }
    }
    
    fn iter_surface_area( &self, iter_index: &mut ( u32, u32 ) ) -> Option< ( u32, u32 ) >
    {
        iter_index.0 += 1;

        if iter_index.0 >= self.right
        {
            iter_index.0 = self.left;
            iter_index.1 += 1;

            if iter_index.1 >= self.bottom
            {
                *iter_index = ( 0, 0 );
                return None;
            }
        }

        Some( *iter_index )
    }
    
    fn surface_area( &self ) -> u32
    {
        self.height() * self.width()
    }
}
