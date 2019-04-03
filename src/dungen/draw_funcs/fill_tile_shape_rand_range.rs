/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

 **/
// External includes.
extern crate rand;
use rand::rngs::ThreadRng;
use rand::Rng;

// Internal includes.
use crate::dungen::DungeonGenerator;
use crate::world::TiledArea;

pub struct FillTileShapeRandRange<'a> {
    start_range: u32,
    end_range: u32,
    rnd: &'a mut ThreadRng,
}

impl<'a> FillTileShapeRandRange<'a> {
    pub fn new(start_range: u32, end_range: u32, rnd: &'a mut ThreadRng) -> Self {
        Self {
            start_range,
            end_range,
            rnd,
        }
    }
}

impl<'a> DungeonGenerator for FillTileShapeRandRange<'a> {
    fn apply(&mut self, area: &mut dyn TiledArea) {
        let mut iter_index: u32 = 0;
        let mut keep_going: bool = true;
        while keep_going {
            match area.iter_surface_area(&mut iter_index) {
                Some(pos) => {
                    *area.tile_type_mut(pos) = self.rnd.gen_range(self.start_range, self.end_range);
                }
                _ => {
                    keep_going = false;
                    continue;
                }
            }
        }
    }
}
