/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

**/
// External includes

// internal includes
use crate::rrl_math::{MapPosition, Position};
use crate::world::{Lightmap, Mapping, TiledArea, Tilemap};

fn inner_iter(
    to: MapPosition,
    check_pos_x: f64,
    check_pos_y: f64,
    lightmap: &mut Lightmap,
    pos: Position,
    light_value: f64,
    map: &Tilemap,
) {
    let (pos_ux, pos_uy) = (pos.x as u16, pos.y as u16);
    let to_pos = Position::new(to.x as i32, to.y as i32);
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
        let map_pos = map.get_position(check_pos_x as u16, check_pos_y as u16);
        if map_pos == false {
            break;
        }

        {
            let current_dist_sqr = (move_pos_x * move_pos_x) + (move_pos_y * move_pos_y);
            *lightmap.value_mut(map_pos) =
                lightmap.value(map_pos).max(light_value / current_dist_sqr);
        }

        if map.transparent(map_pos) == false && ((map_pos.x != pos_ux) || (map_pos.y != pos_uy)) {
            break;
        }

        check_pos_x += disp_move_x;
        check_pos_y += disp_move_y;
        move_pos_x += disp_move_x;
        move_pos_y += disp_move_y;
    }
}

pub fn calculate_light_level(
    lightmap: &mut Lightmap,
    pos: Position,
    light_value: f64,
    map: &Tilemap,
) {
    let light_value = light_value * light_value;

    let check_pos_x = f64::from(pos.x) + 0.5;
    let check_pos_y = f64::from(pos.y) + 0.5;
    let mut iter_index: u32 = 0;
    while let Some(to) = map.iter_circumference(&mut iter_index) {
        inner_iter(
            to,
            check_pos_x,
            check_pos_y,
            lightmap,
            pos,
            light_value,
            map,
        );
    }
}
