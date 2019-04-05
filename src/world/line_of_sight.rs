/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes

// internal includes
use crate::rrl_math::Position;
use crate::stats::{CreatureStats, StatModifier};
use crate::world::{
    Lightmap, MapPosition, Mapping, TiledArea, Tilemap, VisibilityMap, VisibilityType,
};

fn inner_iter(
    to: MapPosition,
    check_pos_x: f64,
    check_pos_y: f64,
    visibility: &mut VisibilityMap,
    lightmap: &Lightmap,
    perception_mult: f64,
    pos: Position,
    map: &Tilemap,
) {
    let (pos_ux, pos_uy) = (pos.x as u16, pos.y as u16);
    let to_pos = Position::new(i32::from(to.x()), i32::from(to.y()));
    let disp_x = f64::from(to_pos.x - pos.x);
    let disp_y = f64::from(to_pos.y - pos.y);
    let dist = ((disp_x * disp_x) + (disp_y * disp_y)).sqrt();
    let disp_norm_x = disp_x / dist;
    let disp_norm_y = disp_y / dist;
    let disp_move_x = disp_norm_x / 2.0;
    let disp_move_y = disp_norm_y / 2.0;
    let mut check_pos_x = check_pos_x;
    let mut check_pos_y = check_pos_y;
    let fwidth = f64::from(lightmap.width() - 1);
    let fheight = f64::from(lightmap.height() - 1);
    while ((check_pos_x >= 0.0) && (check_pos_x <= fwidth))
        && ((check_pos_y >= 0.0) && (check_pos_y <= fheight))
    {
        let map_pos = map.get_position(check_pos_x as u16, check_pos_y as u16);
        if map_pos == false {
            break;
        }
        let map_pos = map_pos.unwrap();

        if (lightmap.value(map_pos) * perception_mult) >= 1.0 {
            *visibility.value_mut(map_pos) = VisibilityType::Visible;
        }

        if map.transparent(map_pos) == false && ((map_pos.x() != pos_ux) || (map_pos.y() != pos_uy))
        {
            break;
        }

        check_pos_x += disp_move_x;
        check_pos_y += disp_move_y;
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

    let mut perception_mult: f64 = if stats.perception().modifier() > 0 {
        1.0 + (f64::from(stats.perception().modifier()) / 10.0)
    } else {
        1.0 / (1.0 + (f64::from(stats.perception().modifier()) / 10.0))
    };
    // Square it, so it notably affects lighting.
    perception_mult = perception_mult * perception_mult;
    // Cube it, since lighting is squared.
    perception_mult = perception_mult * perception_mult;
    let perception_mult = perception_mult;
    let check_pos_x = f64::from(pos.x) + 0.5;
    let check_pos_y = f64::from(pos.y) + 0.5;
    let mut iter_index: u32 = 0;
    while let Some(to) = map.iter_circumference(&mut iter_index) {
        inner_iter(
            to,
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
