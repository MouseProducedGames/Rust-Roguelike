// External includes

// internal includes
use super::linear::{ Displacement, Position };
use super::mapping::Mapping;
use super::tilemap::Tilemap;
use super::visibility::{ VisibilityMap, VisibilityType };

pub fn calculate_visibility(visibility: &mut VisibilityMap, pos: Position, sight_range: i32, map: &Tilemap)
{
    let sight_range_sqr = ( sight_range * sight_range ) as f64;

    for y in -( sight_range + 1 )..( sight_range + 1 )
    {
        for x in -( sight_range + 1 )..( sight_range + 1)
        {
            let check_pos = pos + Displacement::new( x, y );
            if visibility.is_pos_in_bounds( check_pos ) &&
                visibility.value_pos( check_pos ) == VisibilityType::Visible
            {
                *visibility.value_mut( check_pos.x as usize, check_pos.y as usize ) = VisibilityType::Seen;
            }
        }
    }
    
    for to_y in -sight_range..sight_range
    {
        for to_x in -sight_range..sight_range
        {
            let disp = Displacement::new( to_x, to_y );

            let to_pos = pos + disp;
            let disp_x = ( to_pos.x - pos.x ) as f64;
            let disp_y = ( to_pos.y - pos.y ) as f64;
            let dist = ( ( disp_x * disp_x ) + ( disp_y * disp_y ) ).sqrt();
            let disp_norm_x = disp_x / dist;
            let disp_norm_y = disp_y / dist;
            let mut move_pos_x: f64 = 0.0;
            let mut move_pos_y: f64 = 0.0;
            let mut check_pos_x = ( pos.x as f64 ) + 0.5;
            let mut check_pos_y = ( pos.y as f64 ) + 0.5;
            while ( ( move_pos_x * move_pos_x ) + ( move_pos_y * move_pos_y ) ) <= sight_range_sqr
            {
                let check_pos = Position::new( check_pos_x as i32, check_pos_y as i32 );
                if map.is_pos_in_bounds( check_pos ) == false
                {
                    break;
                }
                *visibility.value_mut( check_pos.x as usize, check_pos.y as usize ) = VisibilityType::Visible;
                if map.transparent_pos( check_pos ) == false
                {
                    break;
                }

                check_pos_x += disp_norm_x;
                check_pos_y += disp_norm_y;
                move_pos_x += disp_norm_x;
                move_pos_y += disp_norm_y;
            }
        }
    }
}
