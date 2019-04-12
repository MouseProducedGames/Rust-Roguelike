/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

 **/
// External includes.
use specs::{Component, Entities, NullStorage, ReadStorage, System, WriteExpect, WriteStorage};

// Standard includes.

// Internal includes.
use crate::ai::maslow::MaslowTree;
use crate::ai::Command;
use crate::factions::Faction;
use crate::game::EntityPositionTracker;
use crate::items::Inventory;
use crate::rrl_math::Position;
use crate::skills::SkillLookup;
use crate::stats::CreatureStats;
use crate::talents::TalentLookup;
use crate::world::{Tilemap, VisibilityMapLookup};

#[derive(Default)]
pub struct LogicMaslow;

impl Component for LogicMaslow {
    type Storage = NullStorage<Self>;
}

pub struct LogicMaslowSystem;

#[derive(SystemData)]
pub struct SystemDataT<'a> {
    creature_stats: WriteStorage<'a, CreatureStats>,
    entity_position_tracker: WriteExpect<'a, EntityPositionTracker>,
    entities: Entities<'a>,
    tilemap: WriteExpect<'a, Tilemap>,
    factions: WriteStorage<'a, Faction>,
    inventory: WriteStorage<'a, Inventory>,
    positions: WriteStorage<'a, Position>,
    skills: WriteStorage<'a, SkillLookup>,
    talents: WriteStorage<'a, TalentLookup>,
    visibility_map_lookup: WriteStorage<'a, VisibilityMapLookup>,
    logic: ReadStorage<'a, LogicMaslow>,
    maslow: ReadStorage<'a, MaslowTree>,
    commands: WriteStorage<'a, Command>,
}

impl<'a> System<'a> for LogicMaslowSystem {
    type SystemData = SystemDataT<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        use specs::join::Join;

        let entity_position_tracker = &mut *data.entity_position_tracker;
        let factions = &mut data.factions;
        let map = &mut *data.tilemap;

        for (
            entity,
            creature_stats,
            inventory,
            position,
            skills,
            talents,
            visibility_map_lookup,
            _,
            maslow,
            command,
        ) in (
            &data.entities,
            &mut data.creature_stats,
            &mut data.inventory,
            &mut data.positions,
            &mut data.skills,
            &mut data.talents,
            &mut data.visibility_map_lookup,
            &data.logic,
            &data.maslow,
            &mut data.commands,
        )
            .join()
        {
            let visibility_map = visibility_map_lookup.get_or_add_mut(map);

            *command = maslow.call(
                creature_stats,
                entity,
                entity_position_tracker,
                factions,
                inventory,
                position,
                skills,
                talents,
                map,
                visibility_map,
            );
        }
    }
}
