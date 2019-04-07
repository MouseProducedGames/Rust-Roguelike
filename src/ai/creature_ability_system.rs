/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

 **/
// External includes.
use specs::{ReadStorage, System, WriteExpect, WriteStorage};

// Standard includes.

// Internal includes.
use super::Visibility;
use crate::rrl_math::{calculate_hash, Displacement, Position};
use crate::skills::{SkillActivation, SkillLookup, SkillPassiveOp, SkillTag, SkillType};
use crate::stats::{CreatureStats, StatModifier};
use crate::talents::{
    talent_range_func, TalentActivation, TalentActivationOp, TalentLookup, TalentType,
};
use crate::world::{execute_tile_func, Tilemap, VisibilityMap, VisibilityType};

pub struct CreatureAbilitySystem;

#[derive(SystemData)]
pub struct SystemDataT<'a> {
    map: WriteExpect<'a, Tilemap>,
    stats: ReadStorage<'a, CreatureStats>,
    visibility: ReadStorage<'a, Visibility>,
    pos: WriteStorage<'a, Position>,
    skills: WriteStorage<'a, SkillLookup>,
    talents: WriteStorage<'a, TalentLookup>,
}

impl<'a> System<'a> for CreatureAbilitySystem {
    type SystemData = SystemDataT<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        use specs::join::Join;

        let mut map = data.map;
        let map_hash = calculate_hash(&*map);

        for (pos, skills, talent, stats, visibility) in (
            &data.pos,
            &mut data.skills,
            &mut data.talents,
            &data.stats,
            &data.visibility,
        )
            .join()
        {
            let pos = *pos;

            let maybe_visibility_map = visibility.visibility_lookup().get(&map_hash);

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
                            &(pos, maybe_visibility_map, skill_bonus),
                            &mut *map,
                            |disp: Displacement,
                             data: &(Position, Option<&VisibilityMap>, i64),
                             data_mut: &mut Tilemap| {
                                let (pos, maybe_visibility_map, skill_bonus) = data;
                                let mut map = data_mut;

                                let scan_pos = *pos + disp;

                                let visibility_type;
                                if let Some(visibility_map) = maybe_visibility_map {
                                    visibility_type = visibility_map.value_pos(scan_pos);
                                } else {
                                    visibility_type = VisibilityType::None;
                                }

                                execute_tile_func(
                                    true,
                                    *skill_bonus,
                                    &mut map,
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
