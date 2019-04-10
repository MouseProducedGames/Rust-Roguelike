/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

 **/
// External includes.
use specs::{System, WriteExpect, WriteStorage};

// Standard includes.

// Internal includes.
use crate::abilities::{AbilityActivation, AbilityActivationOp};
use crate::rrl_math::Position;
use crate::skills::SkillLookup;
use crate::stats::CreatureStats;
use crate::talents::TalentLookup;
use crate::world::{Lightmap, Tilemap, VisibilityMapLookup};

pub struct AbilitySystem;

#[derive(SystemData)]
pub struct SystemDataT<'a> {
    lightmap: WriteExpect<'a, Lightmap>,
    map: WriteExpect<'a, Tilemap>,
    pos: WriteStorage<'a, Position>,
    skills: WriteStorage<'a, SkillLookup>,
    stats: WriteStorage<'a, CreatureStats>,
    talents: WriteStorage<'a, TalentLookup>,
    visibility_map_lookup: WriteStorage<'a, VisibilityMapLookup>,
}

impl<'a> System<'a> for AbilitySystem {
    type SystemData = SystemDataT<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        use specs::join::Join;

        let lightmap = &mut *data.lightmap;
        let map = &mut *data.map;

        for (pos, skills, stats, talent, visibility_map_lookup) in (
            &mut data.pos,
            &mut data.skills,
            &mut data.stats,
            &mut data.talents,
            &mut data.visibility_map_lookup,
        )
            .join()
        {
            let mut pos = *pos;

            let visibility_map = visibility_map_lookup.get_or_add_mut(map);

            for ability in
                talent.get_set(AbilityActivation::Passive(AbilityActivationOp::EveryRound))
            {
                ability.apply(lightmap, &mut pos, skills, stats, map, visibility_map);
            }
        }
    }
}
