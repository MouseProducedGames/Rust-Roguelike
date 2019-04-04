/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

 **/
// External includes.
extern crate rand;
use rand::rngs::ThreadRng;

// Internal includes.
use crate::dungen::draw_funcs::{DrawTileShape, FillTileShape, FillTileShapeRandRange};
use crate::dungen::DungeonGenerator;
use crate::tiled_shapes_2d::TiledRect;
use crate::world::{Mapping, TiledArea, TiledAreaFilter};

pub struct _RandomlyTileDungeon<'a> {
    start_range: u32,
    end_range: u32,
    rnd: &'a mut ThreadRng,
}

impl<'a> _RandomlyTileDungeon<'a> {
    pub fn _new(start_range: u32, end_range: u32, rnd: &'a mut ThreadRng) -> Self {
        Self {
            start_range,
            end_range,
            rnd,
        }
    }
}

impl<'a> DungeonGenerator for _RandomlyTileDungeon<'a> {
    fn apply<TArea>(&mut self, area: &mut TArea)
    where
        TArea: TiledArea + Mapping,
    {
        FillTileShape::new(2).apply(area);
        DrawTileShape::new(1).apply(area);
        let (width, height) = (area.width(), area.height());
        let mut filter_area: TiledAreaFilter = TiledAreaFilter::new(
            area,
            Box::new(TiledRect::with_absolute_bounds(1, 1, width - 1, height - 1)),
        );
        FillTileShapeRandRange::new(self.start_range, self.end_range, self.rnd)
            .apply(&mut filter_area);
    }
}
