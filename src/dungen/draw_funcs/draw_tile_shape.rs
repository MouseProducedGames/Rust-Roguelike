/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

 **/

// External includes.

// Internal includes.
use crate::world::Tilemap ;
use crate::dungen::draw_funcs::FillTileShape;
use crate::tiled_shapes_2d::{ TiledShape2DCircumferenceIterator, TiledShapeDef2D };

pub trait DrawTileShape
{ 
    fn draw_tile_shape(
        &mut self,
        shape: &TiledShapeDef2D,
        tile_type: u32
    ) -> &mut Tilemap;
}

impl DrawTileShape for Tilemap
{
    fn draw_tile_shape(
        &mut self,
        shape: &TiledShapeDef2D,
        tile_type: u32
    ) -> &mut Tilemap
    {
        self.fill_tile_shape(
            &mut TiledShape2DCircumferenceIterator::new( shape ),
            tile_type
        )

        /* let ( right, bottom ) = ( left + width, top + height );

        {
            for x in left..right
            {
                *self.tile_type_mut( x, top ) = tile_type;
                // Range is exclusive on bottom.
                *self.tile_type_mut( x, bottom - 1 ) = tile_type;
            }
        }

        {
            for y in ( top + 1 )..( bottom - 1 )
            {
                *self.tile_type_mut( left, y ) = tile_type;
                // Range is exclusive on right.
                *self.tile_type_mut( right - 1, y ) = tile_type;
            }
        }

        self */
    }
}
