/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

 **/
// External includes.
use specs::{ReadStorage, System, WriteExpect, WriteStorage};

// Standard includes.
use std::convert::From;

// Internal includes.
use crate::rrl_math::{Displacement, Position};
use crate::skills::{SkillActivation, SkillLookup, SkillPassiveOp, SkillTag, SkillType};
use crate::stats::{CreatureStats, StatModifier};
use crate::talents::{
    talent_range_func, TalentActivation, TalentActivationOp, TalentLookup, TalentType,
};
use crate::world::{execute_tile_func, Tilemap, VisibilityMap, VisibilityMapLookup};

pub struct AbilitySystem;

#[derive(SystemData)]
pub struct SystemDataT<'a> {
    map: WriteExpect<'a, Tilemap>,
    stats: ReadStorage<'a, CreatureStats>,
    visibility_map_lookup: WriteStorage<'a, VisibilityMapLookup>,
    pos: WriteStorage<'a, Position>,
    skills: WriteStorage<'a, SkillLookup>,
    talents: WriteStorage<'a, TalentLookup>,
}

impl<'a> System<'a> for AbilitySystem {
    type SystemData = SystemDataT<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        use specs::join::Join;

        let map = &mut *data.map;

        for (pos, skills, talent, stats, visibility_map_lookup) in (
            &data.pos,
            &mut data.skills,
            &mut data.talents,
            &data.stats,
            &mut data.visibility_map_lookup,
        )
            .join()
        {
            let pos = *pos;

            let visibility_map = visibility_map_lookup.get_or_add(map);

            for talent_type in
                talent.get_set(TalentActivation::Passive(TalentActivationOp::EveryRound))
            {
                match talent_type {
                    TalentType::ScanForSecrets(talent_bonus, talent_range) => {
                        let set = skills.get_set(SkillActivation::Passive(
                            SkillTag::Perception,
                            SkillPassiveOp::EveryRound,
                        ));

                        let mut skill_bonus = i64::from(*talent_bonus);
                        for skill in set {
                            if let SkillType::Skill(v) = skill {
                                skill_bonus += i64::from(*v)
                            }
                        }

                        let skill_bonus = skill_bonus + i64::from(stats.perception().modifier());

                        talent_range_func(
                            *talent_range,
                            &(pos, visibility_map, skill_bonus),
                            &mut *map,
                            |disp: Displacement,
                             data: &(Position, &VisibilityMap, i64),
                             data_mut: &mut Tilemap| {
                                let (pos, visibility_map, skill_bonus) = data;
                                let map = data_mut;

                                let scan_pos = *pos + disp;

                                let visibility_type = visibility_map.value_pos(scan_pos);

                                execute_tile_func(
                                    true,
                                    *skill_bonus,
                                    map,
                                    visibility_type,
                                    scan_pos,
                                );
                            },
                        );
                    }
                }
            }
        }
    }
}
