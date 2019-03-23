/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

 **/
// External dependencies.

extern crate rust_dice;

use specs::{ReadStorage, System, WriteExpect, WriteStorage};

// Internal dependencies.
use crate::creature::Visibility;
use crate::rrl_math::{calculate_hash, Displacement, Position};
use crate::skills::{
    SkillActivation, SkillPassiveOp, SkillLookup, SkillTag, SkillType
};
use crate::talents::{
    TalentActivation, TalentActivationOp, TalentLookup, TalentRange, talent_range_func,
    TalentType
};
use crate::world::{execute_tile_func, Tilemap, VisibilityMap, VisibilityType};

pub struct CreatureAbilitySystem;

#[derive(SystemData)]
pub struct SystemDataT<'a> {
    map: WriteExpect<'a, Tilemap>,
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

        for (pos, skills, talent, visibility) in
            (&data.pos, &mut data.skills, &mut data.talents, &data.visibility).join() {

                let pos = *pos;
                
                let maybe_visibility_map = visibility.visibility_lookup().get(&map_hash);

                for talent_type in
                    talent.get_set(TalentActivation::Passive(TalentActivationOp::EveryRound))
                {
                    match talent_type {
                        TalentType::ScanForSecrets(talent_bonus, talent_range) => {
                            let set = skills.get_set(
                                SkillActivation::Passive(SkillTag::Perception, SkillPassiveOp::EveryRound)
                            );

                            let mut skill_total = *talent_bonus as i64;
                            for skill in set
                            {
                                match skill {
                                    SkillType::Skill(v) => skill_total += *v as i64,
                                    _ => (),
                                };
                            }

                            talent_range_func(
                                talent_range,
                                &( pos, maybe_visibility_map, skill_total),
                                &mut *map,
                                |
                                disp: Displacement,
                                data: &(Position, Option<&VisibilityMap>, i64),
                                data_mut: &mut Tilemap,
                                |
                                {
                                    let ( pos, maybe_visibility_map, skill_total ) = data;
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
                                        *skill_total,
                                        &mut map,
                                        visibility_type,
                                        scan_pos,
                                    );
                                }
                            );
                        },
                    }
                }
            }
    }
}
