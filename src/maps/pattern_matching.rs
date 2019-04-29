/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use enumflags2::BitFlags;

// Standard includes.
use std::collections::HashMap;

// Internal includes.
use super::{MapDisplacement, MapScanPosition, Mapping, Tilemap};

#[derive(EnumFlags, Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum PatternFlags {
    FlipHorizontal = 0b0000_0001,
    FlipVertical = 0b0000_0010,
    Rotate0 = 0b0000_0100,
    Rotate90 = 0b0000_1000,
    Rotate180 = 0b0001_0000,
    Rotate270 = 0b0010_0000,
}

fn flip_horizontal(x: u16, y: u16, width: u16) -> (u16, u16) {
    ((width - x) - 1, y)
}

fn flip_vertical(x: u16, y: u16, height: u16) -> (u16, u16) {
    (x, (height - y) - 1)
}

fn rotate_90(x: u16, y: u16, height: u16) -> (u16, u16) {
    ((height - y) - 1, x)
}

fn rotate_180(x: u16, y: u16, width: u16, height: u16) -> (u16, u16) {
    ((width - x) - 1, (height - y) - 1)
}

fn rotate_270(x: u16, y: u16, width: u16) -> (u16, u16) {
    (y, (width - x) - 1)
}

pub fn match_pattern(
    this: &Tilemap,
    this_at: MapScanPosition,
    pattern: &Tilemap,
    flags: BitFlags<PatternFlags>,
) -> bool {
    let (width, height) = (pattern.width(), pattern.height());
    for flag in flags.iter() {
        for y in 0..height {
            for x in 0..width {
                let (x, y) = match flag {
                    PatternFlags::FlipHorizontal => flip_horizontal(x, y, width),
                    PatternFlags::FlipVertical => flip_vertical(x, y, height),
                    PatternFlags::Rotate0 => (x, y),
                    PatternFlags::Rotate90 => rotate_90(x, y, height),
                    PatternFlags::Rotate180 => rotate_180(x, y, width, height),
                    PatternFlags::Rotate270 => rotate_270(x, y, width),
                };
                let pattern_at = pattern.get_position(y, x);
                let pattern_at = pattern_at.unwrap();
                let displace_this_at = MapDisplacement::new(pattern_at.x(), pattern_at.y());
                let current_this_at = this_at + displace_this_at;
                if current_this_at == false {
                    return false;
                }

                let this_at_position = current_this_at.unwrap();
                if this.tile_type(this_at_position) != pattern.tile_type(pattern_at)
                    || this.tile_func_type(this_at_position) != pattern.tile_func_type(pattern_at)
                {
                    return false;
                }
            }
        }
    }

    true
}

pub struct PatternLookup {
    values: HashMap<String, Tilemap>,
}

impl PatternLookup {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub fn get(&self, name: &str) -> Option<&Tilemap> {
        self.values.get(name)
    }

    pub fn insert(&mut self, name: &str, tilemap: Tilemap) {
        self.values.insert(String::from(name), tilemap);
    }
}
