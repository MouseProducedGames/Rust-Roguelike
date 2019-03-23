use crate::rrl_math::Displacement;
/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

 **/
// External includes.

// Internal includes.
use crate::TalentRange;

pub fn talent_range_func<TData, TDataMut>(
    talent_range: &TalentRange,
    data: &TData,
    data_mut: &mut TDataMut,
    fn_ptr: fn(Displacement, &TData, &mut TDataMut),
) {
    match *talent_range {
        TalentRange::Radius(radius) => {
            let radius_sqr = (radius as u64) * (radius as u64);
            let iradius = radius as i32;
            for iyd in -iradius..=iradius {
                for ixd in -iradius..=iradius {
                    let xd = ixd as i64;
                    let yd = iyd as i64;
                    if ((yd * yd) + (xd * xd)) as u64 <= radius_sqr {
                        fn_ptr(Displacement::new(ixd, iyd), data, data_mut);
                    }
                }
            }
        }
    }
}
