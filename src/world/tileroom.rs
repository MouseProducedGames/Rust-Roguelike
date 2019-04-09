/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.

// Standard includes.

// Internal includes.
use super::{MapPosition, Mapping, Tilemap};
use crate::dungen::DungenCommon;
use crate::tiled_shapes_2d::TiledShape2D;

pub struct TiledAreaFilter<'a> {
    shape_filter: &'a mut dyn TiledShape2D,
    area: &'a mut dyn TiledArea,
}

impl<'a> TiledAreaFilter<'a> {
    pub fn new(
        area: &'a mut dyn TiledArea,
        shape_filter: &'a mut dyn TiledShape2D
    ) -> Self {
        Self {
            shape_filter,
            area,
        }
    }

    /*     pub fn new_boxed( area: Box<dyn TiledArea>, shape_filter: Box<dyn TiledShape2D> ) -> Box<dyn TiledArea>
    {
        let output: Box<dyn TiledArea> =
            Box::new( Self { area: area, shape_filter: shape_filter } );
        output
    } */

    /* pub fn height(&self) -> u16 {
        self.shape_filter.height()
    }

    pub fn width(&self) -> u16 {
        self.shape_filter.width()
    } */
}

impl<'a> DungenCommon for TiledAreaFilter<'a> {
    fn finish(&mut self) -> Tilemap {
        self.area.finish()
    }
}

impl<'a> Mapping<'a> for TiledAreaFilter<'a> {
    fn height(&self) -> u16 {
        (self.shape_filter.bottom() - self.shape_filter.top()) + 1
    }

    fn width(&self) -> u16 {
        (self.shape_filter.right() - self.shape_filter.left()) + 1
    }
}

pub trait TiledArea {
    fn bottom(&self) -> u16;

    fn circumference(&self) -> u16;

    fn finish(&mut self) -> Tilemap;

    // fn height(&self) -> u16;

    fn iter_circumference(&self, iter_index: &mut u32) -> Option<MapPosition>;

    fn iter_surface_area(&self, iter_index: &mut u32) -> Option<MapPosition>;

    fn left(&self) -> u16;

    fn right(&self) -> u16;

    fn surface_area(&self) -> u16;

    fn tile_func_type(&self, pos: MapPosition) -> u32;

    fn tile_func_type_mut(&mut self, pos: MapPosition) -> &mut u32;

    fn tile_type(&self, pos: MapPosition) -> u32;

    fn tile_type_mut(&mut self, pos: MapPosition) -> &mut u32;

    fn top(&self) -> u16;

    // fn width(&self) -> u16;
}

impl<'a> Mapping<'a> for TiledArea + 'a {
    fn height(&self) -> u16 {
        (self.bottom() - self.top()) + 1
    }

    fn width(&self) -> u16 {
        (self.right() - self.left()) + 1
    }
}

impl DungenCommon for TiledArea {
    fn finish(&mut self) -> Tilemap {
        TiledArea::finish(self)
    }
}

impl TiledArea for Tilemap {
    fn bottom(&self) -> u16 {
        self.height() - 1
    }

    fn circumference(&self) -> u16 {
        // Should be optimized: TWo gets, two adds, and a return.
        let half = self.height() + self.width();
        (half + half) as u16
    }

    fn finish(&mut self) -> Tilemap {
        DungenCommon::finish(self)
    }

    /* fn height(&self) -> u16 {
        self.height() as u16
    } */

    fn iter_circumference(&self, iter_index: &mut u32) -> Option<MapPosition> {
        let (width, height) = (u32::from(self.width()), u32::from(self.height()));
        let index = *iter_index;
        *iter_index += 1;
        if index < width {
            Some(MapPosition::new(index as u16, 0))
        } else if index < (width + height) {
            let temp = (index - width) as u16;
            Some(MapPosition::new(self.width() - 1, temp))
        } else if index < (width + height + width) {
            let temp = (index - (width + height)) as u16;
            Some(MapPosition::new(temp, self.height() - 1))
        } else if index < (width + height + width + height) {
            let temp = (index - (width + height + width)) as u16;
            Some(MapPosition::new(0, temp))
        } else {
            None
        }
    }

    fn iter_surface_area(&self, iter_index: &mut u32) -> Option<MapPosition> {
        let (width, height) = (u32::from(self.width()), u32::from(self.height()));
        let index = *iter_index;
        *iter_index += 1;
        let x = (index % width) as u16;
        let y = (index / width) as u16;
        if y >= (height as u16) {
            return None;
        }

        Some(MapPosition::new(x, y))
    }

    fn left(&self) -> u16 {
        0
    }

    fn right(&self) -> u16 {
        self.width() - 1
    }

    fn surface_area(&self) -> u16 {
        (self.height() * self.width()) as u16
    }

    fn tile_func_type(&self, pos: MapPosition) -> u32 {
        self.tile_func_type(pos)
    }

    fn tile_func_type_mut(&mut self, pos: MapPosition) -> &mut u32 {
        self.tile_func_type_mut(pos)
    }

    fn tile_type(&self, pos: MapPosition) -> u32 {
        self.tile_type(pos)
    }

    fn tile_type_mut(&mut self, pos: MapPosition) -> &mut u32 {
        self.tile_type_mut(pos)
    }

    fn top(&self) -> u16 {
        0
    }

    /* fn width(&self) -> u16 {
        self.width()
    } */
}

impl<'a> TiledArea for TiledAreaFilter<'a> {
    fn bottom(&self) -> u16 {
        // self.area.bottom().min( self.shape_filter.bottom() )
        // self.shape_filter.bottom()
        self.shape_filter.bottom() - self.shape_filter.top()
    }

    fn circumference(&self) -> u16 {
        self.shape_filter.circumference()
    }

    fn finish(&mut self) -> Tilemap {
        self.area.finish()
    }

    /* fn height(&self) -> u16 {
        // ( ( self.bottom() - self.top() ) + 1 ).min( self.area.height() )
        // self.shape_filter.height()
        (self.bottom() - self.top()) + 1
    } */

    fn iter_circumference(&self, iter_index: &mut u32) -> Option<MapPosition> {
        //         let adjuxt_x = self.shape_filter.left() - self.area.left();
        //         let adjuxt_y = self.shape_filter.top() - self.area.top();
        self.shape_filter.iter_circumference(iter_index)
        //         ( ( x + adjust_x ))
    }

    fn iter_surface_area(&self, iter_index: &mut u32) -> Option<MapPosition> {
        self.shape_filter.iter_surface_area(iter_index)
    }

    fn left(&self) -> u16 {
        // self.area.left().max( self.shape_filter.left() )
        // self.shape_filter.left()
        0
    }

    fn right(&self) -> u16 {
        // self.area.right().min( self.shape_filter.right() )
        self.shape_filter.right() - self.shape_filter.left()
    }

    fn surface_area(&self) -> u16 {
        self.shape_filter.surface_area()
    }

    fn tile_func_type(&self, pos: MapPosition) -> u32 {
        let pos = MapPosition::new(
            self.shape_filter.left() + pos.x(),
            self.shape_filter.top() + pos.y(),
        );
        self.area.tile_func_type(pos)
    }

    fn tile_func_type_mut(&mut self, pos: MapPosition) -> &mut u32 {
        let pos = MapPosition::new(
            self.shape_filter.left() + pos.x(),
            self.shape_filter.top() + pos.y(),
        );
        self.area.tile_func_type_mut(pos)
    }

    fn tile_type(&self, pos: MapPosition) -> u32 {
        let pos = MapPosition::new(
            self.shape_filter.left() + pos.x(),
            self.shape_filter.top() + pos.y(),
        );
        self.area.tile_type(pos)
    }

    fn tile_type_mut(&mut self, pos: MapPosition) -> &mut u32 {
        let pos = MapPosition::new(
            self.shape_filter.left() + pos.x(),
            self.shape_filter.top() + pos.y(),
        );
        self.area.tile_type_mut(pos)
    }

    fn top(&self) -> u16 {
        // self.area.top().max( self.shape_filter.top() )
        // self.shape_filter.top()
        0
    }

    /* fn width(&self) -> u16 {
        // ( ( self.right() - self.left() ) + 1 ).min( self.area.width() )
        // self.shape_filter.width()
        (self.right() - self.left()) - 1
    } */
}

impl TiledShape2D for TiledArea {
    fn bottom(&self) -> u16 {
        TiledArea::bottom(self)
    }

    fn circumference(&self) -> u16 {
        TiledArea::circumference(self)
    }

    fn iter_circumference(&self, iter_index: &mut u32) -> Option<MapPosition> {
        TiledArea::iter_circumference(self, iter_index)
    }

    fn iter_surface_area(&self, iter_index: &mut u32) -> Option<MapPosition> {
        TiledArea::iter_surface_area(self, iter_index)
    }

    fn left(&self) -> u16 {
        TiledArea::left(self)
    }

    fn right(&self) -> u16 {
        TiledArea::right(self)
    }

    fn surface_area(&self) -> u16 {
        TiledArea::surface_area(self)
    }

    fn top(&self) -> u16 {
        TiledArea::top(self)
    }
}
