use crate::rrl_math::Displacement;
/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

 **/
// External includes.

// Internal includes.
use crate::TalentRange;

pub fn talent_range_func<TData, TDataMut>(
    talent_range: TalentRange,
    data: &TData,
    data_mut: &mut TDataMut,
    fn_ptr: fn(Displacement, &TData, &mut TDataMut),
) {
    match talent_range {
        TalentRange::Radius(radius) => {
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
