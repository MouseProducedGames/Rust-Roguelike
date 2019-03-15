/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

 **/

// External includes.
extern crate rand;
use rand::Rng;
use rand::rngs::ThreadRng;

// Internal includes.
use crate::dungen::draw_funcs::{ DrawTileShape, FillTileShape, FillTileShapeRandRange };
use crate::tiled_shapes_2d::TiledRect;
use crate::world::{ Mapping, Tilemap };

#[derive( Copy, Clone, PartialEq, Eq )]
pub enum SplitType
{
    LongestDimension,
    Random
}

pub trait RandomlySplitDungeon
{
    fn randomly_split_dungeon(
        &mut self,
        area: &TiledRect, split_type: SplitType,
        min_width: u32, min_height: u32,
        door_tile_type: u32, wall_tile_type: u32,
        rnd: &mut ThreadRng
    ) -> &mut Tilemap;
}

impl RandomlySplitDungeon for Tilemap
{
    fn randomly_split_dungeon(
        &mut self,
        area: &TiledRect, split_type: SplitType,
        min_width: u32, min_height: u32,
        door_tile_type: u32, wall_tile_type: u32,
        rnd: &mut ThreadRng
    ) -> &mut Tilemap
    {
        let ( left, top, right, bottom ) =
            ( area.left(), area.top(), area.right(), area.bottom() );
        let ( width, height ) = ( area.width(), area.height() );

        match ( ( width > min_width ) && ( height > min_height ) ) ||
         ( ( split_type == SplitType::LongestDimension ) && ( ( width > min_width ) || ( height > min_height ) ) ) {
            true => (),
            false => return self,
        }

        
        let split_width;
        // let split_on;
        match split_type {
            SplitType::LongestDimension => {
                if width > height
                {
                    split_width = true;
                }
                else if height > width
                {
                    split_width = false;
                }
                else
                {
                    split_width = rnd.gen_bool( 0.5 );
                }
            },
            SplitType::Random => {
                split_width = rnd.gen_bool( 0.5 );
            }
        }

        /* let ( put_door_x, put_door_y );
        match split_width {
            true => {
                split_on = width / 2;
            },
            false => {
                split_on = height / 2;
            }
        } */

        let split_min;
        let split_max;
        let ( put_door_x, put_door_y );
        match split_width {
            true => {
                split_min = min_width;
                split_max = width - min_width;
            },
            false => {
                split_min = min_height;
                split_max = height - min_height;
            }
        }

        
        let split_on;
        if split_max == split_min
        {
            split_on = split_min;
        }
        else if split_max < split_min
        {
            return self;
        }
        else
        {
            split_on = rnd.gen_range( split_min, split_max );
        }

        let split_line;
        let ( room_left0, room_top0, room_right0, room_bottom0 );
        let ( room_left1, room_top1, room_right1, room_bottom1 );
        match split_width {
            true => {
                split_line =
                    TiledRect::with_absolute_bounds(
                        left + split_on, top,
                        left + split_on + 1, bottom
                    );
                put_door_x = left + split_on;
                put_door_y = rnd.gen_range( top + 1, bottom - 1 );
                room_left0 = left; room_top0 = top;
                room_right0 = left + split_on; room_bottom0 = bottom;
                room_left1 = left + split_on; room_top1 = top;
                room_right1 = right; room_bottom1 = bottom;
            },
            false => {
                split_line =
                    TiledRect::with_absolute_bounds(
                        left, top + split_on,
                        right, top + split_on + 1
                    );
                put_door_x = rnd.gen_range( left + 1, right - 1 );
                put_door_y = top + split_on;
                room_left0 = left; room_top0 = top;
                room_right0 = right; room_bottom0 = top + split_on;
                room_left1 = left; room_top1 = top + split_on;
                room_right1 = right; room_bottom1 = bottom;
            },
        }

        self
            .fill_tile_shape( &TiledRect::with_absolute_bounds( left, top, right, bottom ), door_tile_type )
            .draw_tile_shape( &TiledRect::with_absolute_bounds( left, top, right, bottom ), wall_tile_type );
        // .fill_tile_shape( &split_line, 1 );
        let split_type0 = split_type;
        let split_type1 = split_type;
        self.randomly_split_dungeon(
            &TiledRect::with_absolute_bounds( room_left0, room_top0, room_right0, room_bottom0 ),
            split_type0,
            min_width, min_height,
            door_tile_type, wall_tile_type,
            rnd
        );
        self.randomly_split_dungeon(
            &TiledRect::with_absolute_bounds( room_left1, room_top1, room_right1, room_bottom1 ),
            split_type1,
            min_width, min_height,
            door_tile_type, wall_tile_type,
            rnd
        );
        *self.tile_type_mut( put_door_x as usize, put_door_y as usize ) = 2;
        self
    }
}
