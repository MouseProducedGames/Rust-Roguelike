/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes
use std::marker::{Send, Sync};

// internal includes
use crate::creature::background::SpeciesType;
use crate::creature::CreatureStats;
use crate::faction::Faction;
use crate::rrl_math::Position;
use crate::world::{Tilemap, VisibilityMap};

pub trait Display: Drop + Send + Sync {
    fn choose_species(&mut self, options: &Vec<SpeciesType>) -> SpeciesType;

    fn display_stats(&mut self, stats: CreatureStats);

    fn get_char(&self) -> char;

    fn present(&mut self);

    fn write_creature(&mut self, faction: Faction, creature_pos: Position, view_pos: Position);

    fn write_map(&mut self, view_pos: Position, map: &Tilemap, vis: &VisibilityMap);
}
