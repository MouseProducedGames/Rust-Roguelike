/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use specs::{Component, VecStorage};
pub use specs::{Entities, ReadExpect, ReadStorage, System, WriteExpect};

// Standard includes.

// Internal includes.
use crate::rrl_math::Position;
use crate::world::{calculate_light_level, Lightmap, Tilemap};

#[derive(Clone, Copy)]
pub struct LightSource {
    power: f64,
}

impl LightSource {
    pub fn new(power: f64) -> Self {
        Self { power }
    }

    fn power(self) -> f64 {
        self.power
    }

    fn power_mut(&mut self, value: f64) -> f64 {
        self.power = value;
        self.power()
    }
}

impl Component for LightSource {
    type Storage = VecStorage<Self>;
}

pub struct LightSourceSystem;

#[derive(SystemData)]
pub struct SystemDataT<'a> {
    tilemap: ReadExpect<'a, Tilemap>,
    lightmap: WriteExpect<'a, Lightmap>,
    light_sources: ReadStorage<'a, LightSource>,
    positions: ReadStorage<'a, Position>,
}

impl<'a> System<'a> for LightSourceSystem {
    type SystemData = SystemDataT<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        use specs::join::Join;

        let lightmap = &mut *data.lightmap;
        let tilemap = &*data.tilemap;

        for (light_source, position) in (&data.light_sources, &data.positions).join() {
            let light_source = *light_source;
            let position = *position;

            calculate_light_level(lightmap, position, light_source.power(), tilemap);
        }
    }
}
