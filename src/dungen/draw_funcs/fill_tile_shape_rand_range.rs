/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

 **/
// External includes.
use rand::{thread_rng, Rng};

// Internal includes.
use crate::dungen::DungeonGenerator;
use crate::world::{Mapping, TiledArea};

pub struct FillTileShapeRandRange {
    start_range: u32,
    end_range: u32,
}

impl FillTileShapeRandRange {
    pub fn new(start_range: u32, end_range: u32) -> Self {
        Self {
            start_range,
            end_range,
        }
    }
}

impl DungeonGenerator for FillTileShapeRandRange {
    fn apply<TArea>(&mut self, area: &mut TArea)
    where
        TArea: TiledArea + Mapping,
    {
        let mut iter_index: u32 = 0;
        let mut keep_going: bool = true;
        while keep_going {
            match area.iter_surface_area(&mut iter_index) {
                Some(pos) => {
                    *area.tile_type_mut(pos) = thread_rng().gen_range(self.start_range, self.end_range);
                }
                _ => {
                    keep_going = false;
                    continue;
                }
            }
        }
    }
}
