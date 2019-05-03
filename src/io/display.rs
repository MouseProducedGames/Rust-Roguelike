/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes.
use specs::{Entity, ReadStorage, World};

// Standard includes.
use std::marker::{Send, Sync};
use std::sync::MutexGuard;

// Internal includes.
use super::{DisplayOption, Input, InputData};
use crate::background::{OriginType, SpeciesType};
use crate::bodies::Body;
use crate::data_types::Name;
use crate::factions::Faction;
use crate::items::{Inventory, Item};
use crate::maps::{Tilemap, VisibilityMap};
use crate::rrl_math::Position;
use crate::skills::SkillLookup;
use crate::stats::CreatureStats;

pub trait Display: Drop + Send + Sync {
    fn blit_body(&mut self, world: &World, body: &Body);

    fn blit_inventory(&mut self, world: &World, inventory: &Inventory);

    fn blit_items(&mut self, names: ReadStorage<Name>, items: &[Entity]);

    fn blit_skills(&mut self, world: &World, skill_lookup: &SkillLookup);

    fn choose_display_option(&mut self, options: &'static [DisplayOption]) -> DisplayOption;

    fn choose_origin(&mut self, options: &'static [OriginType]) -> OriginType;

    fn choose_species(&mut self, options: &'static [SpeciesType]) -> SpeciesType;

    fn display_stats(&mut self, stats: CreatureStats);

    fn get_char(&self) -> char;

    fn present(&mut self);

    fn update(&self, input: &mut MutexGuard<Input>) {
        input.update(InputData {
            ch: self.get_char(),
        });
    }

    fn write_creature(&mut self, faction: Faction, creature_pos: Position, view_pos: Position);

    fn write_item(&mut self, item: Item, item_pos: Position, view_pos: Position);

    fn write_map(&mut self, view_pos: Position, map: &Tilemap, vis: &VisibilityMap);
}
