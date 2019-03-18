/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

 **/

// External includes.
extern crate rand;
use rand::Rng;
use rand::rngs::ThreadRng;

// Internal includes.
use crate::dungen::DungeonGenerator;
use crate::dungen::draw_funcs::{ FillTileShape, DrawTileShape };
use crate::rrl_math::Bounds;
use crate::tiled_shapes_2d::TiledRect;
use crate::world::{ TiledArea, TiledAreaFilter };

#[derive( Copy, Clone, PartialEq, Eq )]
pub enum SplitType
{
    LongestDimension,
    _Random
}

pub struct SplitDungeon<'a>
{
    split_type: SplitType,
    min_bounds: Bounds,
    door_tile_type: u32,
    floor_tile_type: u32,
    wall_tile_type: u32,
    rnd: &'a mut ThreadRng
}

impl<'a> SplitDungeon<'a>
{
    pub fn new(
        split_type: SplitType,
        min_bounds: Bounds,
        door_tile_type: u32,
        floor_tile_type: u32,
        wall_tile_type: u32,
        rnd: &'a mut ThreadRng
    ) -> Self
    {
        Self {
            split_type: split_type,
            min_bounds: min_bounds,
            door_tile_type: door_tile_type,
            floor_tile_type: floor_tile_type,
            wall_tile_type: wall_tile_type,
            rnd: rnd
        }
    }
}

impl<'a> DungeonGenerator for SplitDungeon<'a>
{
    fn apply( &mut self, area: &mut Box<dyn TiledArea> )
    {
        let ( left, top, right, bottom ) =
            ( area.left(), area.top(), area.right(), area.bottom() );
        let ( width, height ) = ( area.width(), area.height() );

        if ( ( width > self.min_bounds.width ) && ( height > self.min_bounds.height ) ) ||
            (
                ( self.split_type == SplitType::LongestDimension ) &&
                    ( ( width > self.min_bounds.width ) || ( height > self.min_bounds.height ) )
            )
        {
        } else { return; }

        
        let split_width;
        // let split_on;
        match self.split_type {
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
                    split_width = self.rnd.gen_bool( 0.5 );
                }
            },
            SplitType::_Random => {
                split_width = self.rnd.gen_bool( 0.5 );
            }
        }

        let split_min;
        let split_max;
        let ( put_door_x, put_door_y );
        if split_width
        {
            split_min = self.min_bounds.width;
            split_max = width - self.min_bounds.width;
        }
        else
        {
            split_min = self.min_bounds.height;
            split_max = height - self.min_bounds.height;
        }

        
        let split_on;
        if split_max == split_min
        {
            split_on = split_min;
        }
        else if split_max < split_min
        {
            return;
        }
        else
        {
            split_on = self.rnd.gen_range( split_min, split_max );
        }

        let split_line;
        let ( room_left0, room_top0, room_right0, room_bottom0 );
        let ( room_left1, room_top1, room_right1, room_bottom1 );
        if split_width
        {
            split_line =
                TiledRect::with_absolute_bounds(
                    left + split_on, top,
                    left + split_on, bottom
                );
            put_door_x = left + split_on;
            put_door_y = self.rnd.gen_range( top + 1, bottom - 1 );
            room_left0 = left; room_top0 = top;
            room_right0 = left + split_on; room_bottom0 = bottom;
            room_left1 = left + split_on; room_top1 = top;
            room_right1 = right; room_bottom1 = bottom;
        }
        else
        {
            split_line =
                TiledRect::with_absolute_bounds(
                    left, top + split_on,
                    right, top + split_on
                );
            put_door_x = self.rnd.gen_range( left + 1, right - 1 );
            put_door_y = top + split_on;
            room_left0 = left; room_top0 = top;
            room_right0 = right; room_bottom0 = top + split_on;
            room_left1 = left; room_top1 = top + split_on;
            room_right1 = right; room_bottom1 = bottom;
        }

	{
            FillTileShape::new( self.floor_tile_type ).apply( area );
            DrawTileShape::new( self.wall_tile_type ).apply( area );

            let mut temp_area: Box<dyn TiledArea>;
            temp_area =
                Box::new(
                    TiledAreaFilter::new(
                        area,
                        Box::new( split_line )
                    )
                );
            FillTileShape::new( self.wall_tile_type ).apply( &mut temp_area );
        }
        {
            let rect =
                Box::new( TiledRect::with_absolute_bounds( room_left0, room_top0, room_right0, room_bottom0 ) );
            let mut temp_area: Box<dyn TiledArea>;
            temp_area = Box::new( TiledAreaFilter::new( area, rect ) );
            SplitDungeon::new(
                self.split_type,
                self.min_bounds,
                self.door_tile_type,
                self.floor_tile_type,
                self.wall_tile_type,
                self.rnd
            ).apply( &mut temp_area );
        }
        {
            let rect =
                Box::new( TiledRect::with_absolute_bounds( room_left1, room_top1, room_right1, room_bottom1 ) );
            let mut temp_area: Box<dyn TiledArea>;
            temp_area = Box::new( TiledAreaFilter::new( area, rect ) );
            SplitDungeon::new(
                self.split_type,
                self.min_bounds,
                self.door_tile_type,
                self.floor_tile_type,
                self.wall_tile_type,
                self.rnd
            ).apply( &mut temp_area );
        }
        *area.tile_type_mut( put_door_x, put_door_y ) = self.door_tile_type;
    }
}
