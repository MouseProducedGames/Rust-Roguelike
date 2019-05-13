/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes

// Standard includes.

// Internal includes.
use super::ability::{Ability, AbilityRange};
use crate::game::GameValueFixed;
use crate::maps::{execute_tile_func, Lightmap, Tilemap, VisibilityMap};
use crate::rrl_math::{Displacement, Position};
use crate::skills::{SkillActivation, SkillLookup, SkillPassiveOp, SkillTag, SkillType};
use crate::stats::{CreatureStats, StatModifier};

pub fn ability_func(
    ability: Ability,
    _lightmap: &mut Lightmap,
    pos: &mut Position,
    skills: &mut SkillLookup,
    stats: &mut CreatureStats,
    map: &mut Tilemap,
    visibility_map: &mut VisibilityMap,
) {
    let Ability::ScanForSecrets(ability_modifier, ability_range) = ability;
    scan_for_secrets(
        ability_modifier,
        ability_range,
        pos,
        skills,
        stats,
        map,
        visibility_map,
    );
}

fn ability_range_func<TData, TDataMut>(
    ability_range: AbilityRange,
    data: &TData,
    data_mut: &mut TDataMut,
    fn_ptr: fn(Displacement, &TData, &mut TDataMut),
) {
    match ability_range {
        AbilityRange::Radius(radius) => {
            let radius_sqr = u64::from(radius) * u64::from(radius);
            let iradius = radius as i32;
            for iyd in -iradius..=iradius {
                for ixd in -iradius..=iradius {
                    let xd = i64::from(ixd);
                    let yd = i64::from(iyd);
                    if ((yd * yd) + (xd * xd)) as u64 <= radius_sqr {
                        fn_ptr(Displacement::new(ixd, iyd), data, data_mut);
                    }
                }
            }
        }
    }
}

fn scan_for_secrets(
    ability_modifier: i32,
    ability_range: AbilityRange,
    pos: &mut Position,
    skills: &mut SkillLookup,
    stats: &mut CreatureStats,
    map: &mut Tilemap,
    visibility_map: &mut VisibilityMap,
) {
    let mut skill_bonus = GameValueFixed::from_int(ability_modifier);
    if let Some(set) = skills.get_set(SkillActivation::Passive(
        SkillTag::Perception,
        SkillPassiveOp::EveryRound,
    )) {
        for skill in set {
            if let SkillType::Skill(skill_value) = skill {
                skill_bonus += skill_value.raw()
            }
        }
    }

    let skill_bonus = skill_bonus + stats.perception().modifier();

    ability_range_func(
        ability_range,
        &skill_bonus,
        &mut (map, pos, visibility_map),
        |disp: Displacement,
         data: &GameValueFixed,
         data_mut: &mut (&mut Tilemap, &mut Position, &mut VisibilityMap)| {
            let skill_bonus = data;
            let (map, pos, visibility_map) = data_mut;

            let scan_pos = **pos + disp;

            let visibility_type = visibility_map.value_pos(scan_pos);

            execute_tile_func(true, *skill_bonus, *map, visibility_type, scan_pos);
        },
    );
}
