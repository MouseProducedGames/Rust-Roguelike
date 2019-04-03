/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes

// internal includes
use crate::rrl_math::Position;
use crate::stats::{CreatureStats, StatModifier};
use crate::world::{Lightmap, Mapping, Tilemap, VisibilityMap, VisibilityType};

fn inner_iter(
    to_x: u32,
    to_y: u32,
    check_pos_x: f64,
    check_pos_y: f64,
    visibility: &mut VisibilityMap,
    lightmap: &Lightmap,
    perception_mult: f64,
    pos: Position,
    map: &Tilemap,
) {
    let (pos_ux, pos_uy) = (pos.x as u32, pos.y as u32);
    let to_pos = Position::new(to_x as i32, to_y as i32);
    let disp_x = f64::from(to_pos.x - pos.x);
    let disp_y = f64::from(to_pos.y - pos.y);
    let dist = ((disp_x * disp_x) + (disp_y * disp_y)).sqrt();
    let disp_norm_x = disp_x / dist;
    let disp_norm_y = disp_y / dist;
    let disp_move_x = disp_norm_x / 2.0;
    let disp_move_y = disp_norm_y / 2.0;
    let mut move_pos_x: f64 = 0.0;
    let mut move_pos_y: f64 = 0.0;
    let mut check_pos_x = check_pos_x;
    let mut check_pos_y = check_pos_y;
    let fwidth = f64::from(lightmap.width() - 1);
    let fheight = f64::from(lightmap.height() - 1);
    while ((check_pos_x >= 0.0) && (check_pos_x <= fwidth))
        && ((check_pos_y >= 0.0) && (check_pos_y <= fheight))
    {
        let (check_pos_ux, check_pos_uy) = (check_pos_x as u32, check_pos_y as u32);
        if map.is_in_bounds(check_pos_ux, check_pos_uy) == false {
            break;
        }

        if (lightmap.value(check_pos_ux, check_pos_uy) * perception_mult) >= 1.0 {
            *visibility.value_mut(check_pos_ux, check_pos_uy) = VisibilityType::Visible;
        }

        if map.transparent(check_pos_ux, check_pos_uy) == false
            && ((check_pos_ux != pos_ux) || (check_pos_uy != pos_uy))
        {
            break;
        }

        check_pos_x += disp_move_x;
        check_pos_y += disp_move_y;
        move_pos_x += disp_move_x;
        move_pos_y += disp_move_y;
    }
}

pub fn calculate_visibility(
    lightmap: &Lightmap,
    pos: Position,
    // sight_range: i32,
    stats: &CreatureStats,
    map: &Tilemap,
    visibility: &mut VisibilityMap,
) {
    // let sight_range_sqr = f64::from(sight_range * sight_range);

    visibility.clear();

    let mut perception_mult: f64;
    if stats.perception().modifier() > 0 {
        perception_mult = 1.0 + (f64::from(stats.perception().modifier()) / 10.0);
    } else {
        perception_mult = 1.0 / (1.0 + (f64::from(stats.perception().modifier()) / 10.0));
    }
    // Square it, so it notably affects lighting.
    perception_mult = perception_mult * perception_mult;
    // Cube it, since lighting is squared.
    perception_mult = perception_mult * perception_mult;
    let perception_mult = perception_mult;
    let check_pos_x = f64::from(pos.x) + 0.5;
    let check_pos_y = f64::from(pos.y) + 0.5;
    for to_y in 0..map.height() {
        for to_x in (0..map.width()).step_by((map.width() - 1) as usize) {
            inner_iter(
                to_x,
                to_y,
                check_pos_x,
                check_pos_y,
                visibility,
                lightmap,
                perception_mult,
                pos,
                map,
            );
        }
    }

    for to_x in 0..map.width() {
        let to_y = 0;
        inner_iter(
            to_x,
            to_y,
            check_pos_x,
            check_pos_y,
            visibility,
            lightmap,
            perception_mult,
            pos,
            map,
        );
    }

    for to_x in 0..map.width() {
        let to_y = map.height() - 1;
        inner_iter(
            to_x,
            to_y,
            check_pos_x,
            check_pos_y,
            visibility,
            lightmap,
            perception_mult,
            pos,
            map,
        );
    }
}
